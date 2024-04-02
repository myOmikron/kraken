use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;

use crate::api::handler::common::error::ApiError;
use crate::api::handler::common::error::ApiResult;
use crate::api::handler::common::schema::PathUuid;
use crate::api::handler::common::schema::UuidResponse;
use crate::api::handler::global_tags::schema::CreateGlobalTagRequest;
use crate::api::handler::global_tags::schema::UpdateGlobalTag;
use crate::chan::global::GLOBAL;
use crate::models::GlobalTag;

/// Create a global tag.
///
/// This action requires admin privileges.
#[swaggapi::post("/globalTags", tags("Global Tags"))]
pub async fn create_global_tag(req: Json<CreateGlobalTagRequest>) -> ApiResult<Json<UuidResponse>> {
    let req = req.into_inner();

    let uuid = GlobalTag::insert(&GLOBAL.db, req.name, req.color).await?;

    Ok(Json(UuidResponse { uuid }))
}

/// Update a global tag
///
/// One of the options must be set
///
/// Requires admin privileges.
#[swaggapi::put("/globalTags/{uuid}", tags("Global Tags"))]
pub async fn update_global_tag(
    req: Json<UpdateGlobalTag>,
    path: Path<PathUuid>,
) -> ApiResult<HttpResponse> {
    let path = path.into_inner();
    let req = req.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;

    query!(&mut tx, (GlobalTag::F.uuid,))
        .condition(GlobalTag::F.uuid.equals(path.uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    if let Some(name) = &req.name {
        if name.is_empty() {
            return Err(ApiError::InvalidName);
        }
    }

    update!(&mut tx, GlobalTag)
        .condition(GlobalTag::F.uuid.equals(path.uuid))
        .begin_dyn_set()
        .set_if(GlobalTag::F.name, req.name)
        .set_if(GlobalTag::F.color, req.color.map(|x| x.into()))
        .finish_dyn_set()
        .map_err(|_| ApiError::EmptyJson)?
        .exec()
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a global tag
///
/// Requires admin privileges.
#[swaggapi::delete("/globalTags/{uuid}", tags("Global Tags"))]
pub async fn delete_global_tag(path: Path<PathUuid>) -> ApiResult<HttpResponse> {
    let path = path.into_inner();
    let mut tx = GLOBAL.db.start_transaction().await?;

    query!(&mut tx, (GlobalTag::F.uuid,))
        .condition(GlobalTag::F.uuid.equals(path.uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    rorm::delete!(&mut tx, GlobalTag)
        .condition(GlobalTag::F.uuid.equals(path.uuid))
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
