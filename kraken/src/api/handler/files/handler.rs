use std::fs::File;

use actix_files::NamedFile;
use actix_web::http::header::ContentLength;
use actix_web::web::Header;
use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::web::Payload;
use actix_web::web::Query;
use image::ImageFormat;
use log::error;
use rorm::prelude::*;
use rorm::query;
use rorm::FieldAccess;
use uuid::Uuid;

use crate::api::extractors::SessionUser;
use crate::api::handler::common::error::ApiError;
use crate::api::handler::common::error::ApiResult;
use crate::api::handler::common::schema::PathUuid;
use crate::api::handler::common::schema::UuidResponse;
use crate::api::handler::files::schema::PathFile;
use crate::api::handler::files::schema::UploadQuery;
use crate::api::handler::files::utils::media_file_path;
use crate::api::handler::files::utils::media_thumbnail_path;
use crate::api::handler::files::utils::stream_into_file;
use crate::api::handler::files::utils::stream_into_file_with_magic;
use crate::chan::global::GLOBAL;
use crate::models::DeferCommit;
use crate::models::MediaFile;
use crate::models::MediaFileInsertOutcome;
use crate::models::Workspace;

/// Uploads an image to the workspace and generates a thumbnail for it
///
/// The returned uuid can be used to attach the image for example to a finding.
#[swaggapi::post("/workspace/{uuid}/files/images", tags("Files"))]
pub async fn upload_image(
    path: Path<PathUuid>,
    Query(query): Query<UploadQuery>,
    SessionUser(user_uuid): SessionUser,
    Header(content_length): Header<ContentLength>,
    body: Payload,
) -> ApiResult<Json<UuidResponse>> {
    let workspace_uuid = path.into_inner().uuid;
    let file_uuid = Uuid::new_v4();

    let image_format = query
        .filename
        .rsplit_once('.')
        .and_then(|(_, ext)| ImageFormat::from_extension(ext))
        .ok_or(ApiError::InvalidImage)?;

    let file_path = media_file_path(file_uuid);
    let ((delete_file_guard, sha256), magic_format) =
        stream_into_file_with_magic::<sha2::Sha256>(file_path.as_ref(), content_length, body)
            .await?
            .ok_or(ApiError::InvalidImage)?;

    if image_format != magic_format {
        return Err(ApiError::InvalidImage);
    }

    let mut deferred_tx = GLOBAL.db.start_transaction().await?;
    let outcome = MediaFile::get_or_insert(
        DeferCommit(&mut deferred_tx), // The tx should be committed after all fs operations
        file_uuid,
        query.filename,
        sha256,
        true,
        user_uuid,
        workspace_uuid,
    )
    .await?;

    let (file_uuid, delete_file_guard) = match outcome {
        MediaFileInsertOutcome::Found(existing_file) => {
            return Ok(Json(UuidResponse {
                uuid: existing_file,
            }));
        }
        MediaFileInsertOutcome::Inserted => (
            file_uuid,               // Continue generating thumbnail for uploaded file
            Some(delete_file_guard), // Keep the uploaded file
        ),
        MediaFileInsertOutcome::NeedsThumbnail(existing_file) => (
            existing_file, // Continue generating thumbnail for existing file
            None,          // Dump the uploaded file
        ),
    };

    tokio::task::spawn_blocking(move || {
        let mut reader = image::io::Reader::open(media_file_path(file_uuid))?;
        reader.set_format(image_format);
        let image = reader.decode()?;

        let image = image.thumbnail(256, 256);
        image.save_with_format(media_thumbnail_path(file_uuid), image_format)
    })
    .await
    .map_err(|panic| {
        error!("Image converter paniced: {panic}");
        ApiError::InternalServerError
    })?
    .map_err(|error| {
        error!("Image converter errored: {error}");
        ApiError::InternalServerError
    })?;
    deferred_tx.commit().await?;

    if let Some(guard) = delete_file_guard {
        guard.dont();
    }
    Ok(Json(UuidResponse { uuid: file_uuid }))
}

/// Uploads a file to the workspace
///
/// The returned uuid can be used to attach the file for example to a finding.
#[swaggapi::post("/workspace/{uuid}/files", tags("Files"))]
pub async fn upload_file(
    path: Path<PathUuid>,
    Query(query): Query<UploadQuery>,
    SessionUser(user_uuid): SessionUser,
    Header(content_length): Header<ContentLength>,
    body: Payload,
) -> ApiResult<Json<UuidResponse>> {
    let workspace_uuid = path.into_inner().uuid;
    let file_uuid = Uuid::new_v4();

    let file_path = media_file_path(file_uuid);
    #[allow(clippy::unwrap_used)] // None is only returned iff the hook returns Err which it doesn't
    let (delete_file_guard, sha256) =
        stream_into_file::<sha2::Sha256>(file_path.as_ref(), content_length, body, |_| Ok(()))
            .await?
            .unwrap();

    let outcome = MediaFile::get_or_insert(
        DeferCommit(&GLOBAL.db), // We are not performing any fs operations after this
        file_uuid,
        query.filename,
        sha256,
        false,
        user_uuid,
        workspace_uuid,
    )
    .await?;

    match outcome {
        MediaFileInsertOutcome::Found(existing_file) => Ok(Json(UuidResponse {
            uuid: existing_file,
        })),
        MediaFileInsertOutcome::Inserted => {
            delete_file_guard.dont();
            Ok(Json(UuidResponse { uuid: file_uuid }))
        }
        MediaFileInsertOutcome::NeedsThumbnail(_) => {
            error!("Impossible state");
            Err(ApiError::InternalServerError)
        }
    }
}

/// Downloads a thumbnail from the workspace
#[swaggapi::get("/workspace/{w_uuid}/files/{f_uuid}/thumbnail", tags("Files"))]
pub async fn download_thumbnail(
    path: Path<PathFile>,
    SessionUser(u_uuid): SessionUser,
) -> ApiResult<NamedFile> {
    let PathFile { w_uuid, f_uuid } = path.into_inner();

    if !Workspace::is_user_member_or_owner(&GLOBAL.db, w_uuid, u_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let (name, is_image) = query!(&GLOBAL.db, (MediaFile::F.name, MediaFile::F.is_image))
        .condition(MediaFile::F.uuid.equals(f_uuid))
        .optional()
        .await?
        .ok_or(ApiError::NotFound)?;
    if !is_image {
        return Err(ApiError::NotFound);
    }

    File::open(media_thumbnail_path(f_uuid))
        .and_then(|file| NamedFile::from_file(file, name))
        .map(|file| file.use_etag(true).use_last_modified(true))
        .map_err(|err| {
            error!("Failed to open file for download: {err}");
            ApiError::InternalServerError
        })
}

/// Downloads a file from the workspace
#[swaggapi::get("/workspace/{w_uuid}/files/{f_uuid}", tags("Files"))]
pub async fn download_file(
    path: Path<PathFile>,
    SessionUser(u_uuid): SessionUser,
) -> ApiResult<NamedFile> {
    let PathFile { w_uuid, f_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, u_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let (name,) = query!(&mut tx, (MediaFile::F.name,))
        .condition(MediaFile::F.uuid.equals(f_uuid))
        .optional()
        .await?
        .ok_or(ApiError::NotFound)?;

    tx.commit().await?;

    File::open(media_file_path(f_uuid))
        .and_then(|file| NamedFile::from_file(file, name))
        .map(|file| file.use_etag(true).use_last_modified(true))
        .map_err(|err| {
            error!("Failed to open file for download: {err}");
            ApiError::InternalServerError
        })
}
