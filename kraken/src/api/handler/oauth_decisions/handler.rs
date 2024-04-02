use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;
use futures::TryStreamExt;
use rorm::and;
use rorm::query;
use rorm::FieldAccess;
use rorm::Model;

use crate::api::extractors::SessionUser;
use crate::api::handler::common::error::ApiError;
use crate::api::handler::common::error::ApiResult;
use crate::api::handler::common::schema::PathUuid;
use crate::api::handler::oauth_decisions::schema::FullOauthDecision;
use crate::api::handler::oauth_decisions::schema::ListOauthDecisions;
use crate::api::handler::users::schema::SimpleUser;
use crate::api::handler::workspaces::schema::SimpleWorkspace;
use crate::chan::global::GLOBAL;
use crate::models::OAuthDecision;
use crate::models::Workspace;
use crate::models::WorkspaceAccessToken;

/// Retrieve a user's remembered oauth decisions
#[swaggapi::get("/oauthDecisions", tags("OAuth Decisions"))]
pub async fn get_decisions(
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<ListOauthDecisions>> {
    let decisions = query!(
        &GLOBAL.db,
        (
            OAuthDecision::F.uuid,
            OAuthDecision::F.application.name,
            OAuthDecision::F.workspace as Workspace,
            OAuthDecision::F.workspace.owner as SimpleUser,
            OAuthDecision::F.action,
        )
    )
    .condition(OAuthDecision::F.user.equals(user_uuid))
    .stream()
    .map_ok(|(uuid, app, workspace, owner, action)| FullOauthDecision {
        uuid,
        app,
        workspace: SimpleWorkspace {
            uuid: workspace.uuid,
            name: workspace.name,
            description: workspace.description,
            created_at: workspace.created_at,
            owner,
            archived: workspace.archived,
        },
        action,
    })
    .try_collect()
    .await?;
    Ok(Json(ListOauthDecisions { decisions }))
}

/// Revoke a user's remembered oauth decision
#[swaggapi::delete("/oauthDecisions/{uuid}", tags("OAuth Decisions"))]
pub async fn revoke_decision(
    SessionUser(user_uuid): SessionUser,
    path: Path<PathUuid>,
) -> ApiResult<HttpResponse> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let decision = query!(&mut tx, OAuthDecision)
        .condition(OAuthDecision::F.uuid.equals(path.uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    if *decision.user.key() != user_uuid {
        return Err(ApiError::MissingPrivileges);
    }

    rorm::delete!(&GLOBAL.db, OAuthDecision)
        .condition(OAuthDecision::F.uuid.equals(path.uuid))
        .await?;

    rorm::delete!(&mut tx, WorkspaceAccessToken)
        .condition(and!(
            WorkspaceAccessToken::F.user.equals(user_uuid),
            WorkspaceAccessToken::F
                .workspace
                .equals(*decision.workspace.key()),
            WorkspaceAccessToken::F
                .application
                .equals(*decision.application.key())
        ))
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Ok().finish())
}
