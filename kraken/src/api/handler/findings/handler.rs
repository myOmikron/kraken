use std::collections::HashMap;

use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;
use futures::TryStreamExt;
use rorm::prelude::*;
use rorm::query;
use rorm::update;

use crate::api::extractors::SessionUser;
use crate::api::handler::common::error::ApiError;
use crate::api::handler::common::error::ApiResult;
use crate::api::handler::common::schema::PathUuid;
use crate::api::handler::common::schema::UuidResponse;
use crate::api::handler::finding_definitions::schema::SimpleFindingDefinition;
use crate::api::handler::findings::schema::CreateFindingRequest;
use crate::api::handler::findings::schema::FullFinding;
use crate::api::handler::findings::schema::ListFindings;
use crate::api::handler::findings::schema::PathFinding;
use crate::api::handler::findings::schema::SimpleFinding;
use crate::api::handler::findings::schema::UpdateFindingRequest;
use crate::api::handler::findings::utils::finding_affected_into_simple;
use crate::chan::global::GLOBAL;
use crate::chan::ws_manager::schema::WsMessage;
use crate::models::Finding;
use crate::models::FindingAffected;
use crate::models::FindingDefinition;
use crate::models::FindingDetails;
use crate::models::Workspace;
use crate::modules::cache::EditorCached;

/// Creates a new finding
#[swaggapi::post("/workspace/{uuid}/findings", tags("Findings"))]
pub async fn create_finding(
    path: Path<PathUuid>,
    SessionUser(user_uuid): SessionUser,
    Json(request): Json<CreateFindingRequest>,
) -> ApiResult<Json<UuidResponse>> {
    let workspace_uuid = path.into_inner().uuid;

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, workspace_uuid, user_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let uuid = Finding::insert(
        &mut tx,
        workspace_uuid,
        request.definition,
        request.severity,
        request.details,
        None,
        request.screenshot,
        request.log_file,
    )
    .await?;

    tx.commit().await?;
    Ok(Json(UuidResponse { uuid }))
}

/// Gets a workspace's findings
#[swaggapi::get("/workspace/{uuid}/findings", tags("Findings"))]
pub async fn get_all_findings(
    path: Path<PathUuid>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<ListFindings>> {
    let workspace_uuid = path.into_inner().uuid;

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, workspace_uuid, user_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let mut affected_lookup = HashMap::new();
    let affected = query!(
        &mut tx,
        (FindingAffected::F.uuid, FindingAffected::F.finding)
    )
    .condition(FindingAffected::F.workspace.equals(workspace_uuid))
    .all()
    .await?;

    for (_, finding) in affected {
        affected_lookup
            .entry(*finding.key())
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let findings = query!(
        &mut tx,
        (
            Finding::F.uuid,
            Finding::F.definition.uuid,
            Finding::F.definition.name,
            Finding::F.definition.cve,
            Finding::F.severity,
            Finding::F.created_at,
        )
    )
    .condition(Finding::F.workspace.equals(workspace_uuid))
    .stream()
    .map_ok(
        |(uuid, definition, name, cve, severity, created_at)| SimpleFinding {
            uuid,
            definition,
            name,
            cve,
            severity,
            created_at,
            affected_count: *affected_lookup.get(&uuid).unwrap_or(&0),
        },
    )
    .try_collect()
    .await?;

    tx.commit().await?;
    Ok(Json(ListFindings { findings }))
}

/// Gets a single finding
#[swaggapi::get("/workspace/{w_uuid}/findings/{f_uuid}", tags("Findings"))]
pub async fn get_finding(
    path: Path<PathFinding>,
    SessionUser(u_uuid): SessionUser,
) -> ApiResult<Json<FullFinding>> {
    let PathFinding { w_uuid, f_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, u_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let finding = query!(&mut tx, Finding)
        .condition(Finding::F.uuid.equals(f_uuid))
        .optional()
        .await?
        .ok_or(ApiError::NotFound)?;

    let details = query!(&mut tx, FindingDetails)
        .condition(FindingDetails::F.uuid.equals(*finding.details.key()))
        .one()
        .await?;

    let definition = query!(&mut tx, FindingDefinition)
        .condition(FindingDefinition::F.uuid.equals(*finding.definition.key()))
        .one()
        .await?;

    let affected = query!(&mut tx, FindingAffected)
        .condition(FindingAffected::F.finding.equals(f_uuid))
        .stream()
        .map_err(ApiError::DatabaseError)
        .and_then(|x| std::future::ready(finding_affected_into_simple(x)))
        .try_collect()
        .await?;

    tx.commit().await?;
    Ok(Json(FullFinding {
        uuid: finding.uuid,
        definition: SimpleFindingDefinition {
            uuid: definition.uuid,
            name: definition.name,
            cve: definition.cve,
            severity: definition.severity,
            #[rustfmt::skip]
            summary: GLOBAL.editor_cache.fd_summary.get(*finding.definition.key()).await?.ok_or(ApiError::InvalidUuid)?.0,
            created_at: definition.created_at,
        },
        severity: finding.severity,
        affected,
        #[rustfmt::skip]
        user_details: GLOBAL.editor_cache.finding_details.get(finding.uuid).await?.unwrap_or_default().0,
        tool_details: details.tool_details,
        screenshot: details.screenshot.map(|x| *x.key()),
        log_file: details.log_file.map(|x| *x.key()),
        created_at: finding.created_at,
    }))
}

/// Updates a finding
#[swaggapi::put("/workspace/{w_uuid}/findings/{f_uuid}", tags("Findings"))]
pub async fn update_finding(
    path: Path<PathFinding>,
    SessionUser(u_uuid): SessionUser,
    Json(request): Json<UpdateFindingRequest>,
) -> ApiResult<HttpResponse> {
    let PathFinding { w_uuid, f_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, u_uuid).await? {
        return Err(ApiError::NotFound);
    }

    if matches!(
        &request,
        UpdateFindingRequest {
            definition: None,
            severity: None,
            screenshot: None,
            log_file: None
        }
    ) {
        return Err(ApiError::EmptyJson);
    }

    // Check finding's existence and get the details uuid
    let (d_uuid,) = query!(&mut tx, (Finding::F.details,))
        .condition(Finding::F.uuid.equals(f_uuid))
        .optional()
        .await?
        .ok_or(ApiError::NotFound)?;

    // Update the parts which are stored in Finding
    if let Ok(update) = update!(&mut tx, Finding)
        .condition(Finding::F.uuid.equals(f_uuid))
        .begin_dyn_set()
        .set_if(
            Finding::F.definition,
            request.definition.map(ForeignModelByField::Key),
        )
        .set_if(Finding::F.severity, request.severity)
        .finish_dyn_set()
    {
        update.await?;
    }

    // Update the parts which are stored in FindingDetails
    FindingDetails::update(
        &mut tx,
        *d_uuid.key(),
        None,
        request.screenshot,
        request.log_file,
    )
    .await?;

    tx.commit().await?;
    GLOBAL
        .ws
        .message_workspace(
            w_uuid,
            WsMessage::UpdatedFinding {
                workspace: w_uuid,
                finding: f_uuid,
                update: request,
            },
        )
        .await;
    Ok(HttpResponse::Ok().finish())
}

/// Deletes a finding
#[swaggapi::delete("/workspace/{w_uuid}/findings/{f_uuid}", tags("Findings"))]
pub async fn delete_finding(
    path: Path<PathFinding>,
    SessionUser(u_uuid): SessionUser,
) -> ApiResult<HttpResponse> {
    let PathFinding { w_uuid, f_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, u_uuid).await? {
        return Err(ApiError::NotFound);
    }

    let deleted = Finding::delete(&mut tx, f_uuid).await?;

    tx.commit().await?;
    if deleted {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApiError::NotFound)
    }
}
