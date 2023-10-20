//! This module holds the aggregated data of ports

use actix_toolbox::tb_middleware::Session;
use actix_web::get;
use actix_web::web::{Data, Json, Path, Query};
use futures::TryStreamExt;
use rorm::conditions::{BoxedCondition, Condition};
use rorm::{and, query, Database, FieldAccess, Model};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::api::handler::{
    get_page_params, ApiError, ApiResult, PageParams, PathUuid, PortResultsPage,
};
use crate::models::{Port, PortProtocol, Workspace};

/// Query parameters for filtering the ports to get
#[derive(Deserialize, IntoParams)]
pub struct GetAllPortsQuery {
    /// Only get ports associated with a specific host
    pub host: Option<Uuid>,
}

/// The simple representation of a port
#[derive(Serialize, ToSchema)]
pub struct SimplePort {
    /// Uuid of the port
    pub uuid: Uuid,
    /// Port number
    #[schema(example = 1337)]
    pub port: u16,
    /// Port protocol
    pub protocol: PortProtocol,
    /// The host this port is assigned to
    pub host: Uuid,
    /// A comment to the port
    pub comment: String,
}

/// List the ports of a workspace
#[utoipa::path(
    tag = "Ports",
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Retrieve all ports of a workspace", body = PortResultsPage),
        (status = 400, description = "Client error", body = ApiErrorResponse),
        (status = 500, description = "Server error", body = ApiErrorResponse),
    ),
    params(PathUuid, PageParams, GetAllPortsQuery),
    security(("api_key" = []))
)]
#[get("/workspaces/{uuid}/ports")]
pub async fn get_all_ports(
    path: Path<PathUuid>,
    page_params: Query<PageParams>,
    filter_params: Query<GetAllPortsQuery>,
    db: Data<Database>,
    session: Session,
) -> ApiResult<Json<PortResultsPage>> {
    let user_uuid: Uuid = session.get("uuid")?.ok_or(ApiError::SessionCorrupt)?;
    let path = path.into_inner();

    let mut tx = db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    let (limit, offset) = get_page_params(page_params).await?;

    fn build_condition(workspace: Uuid, filter_params: &GetAllPortsQuery) -> BoxedCondition<'_> {
        match filter_params {
            GetAllPortsQuery { host: Some(host) } => and![
                Port::F.workspace.equals(workspace),
                Port::F.host.equals(*host)
            ]
            .boxed(),
            GetAllPortsQuery { host: None } => Port::F.workspace.equals(workspace).boxed(),
        }
    }

    let (total,) = query!(&mut tx, (Port::F.uuid.count(),))
        .condition(build_condition(path.uuid, &filter_params))
        .one()
        .await?;

    let ports: Vec<_> = query!(&mut tx, Port)
        .condition(build_condition(path.uuid, &filter_params))
        .limit(limit)
        .offset(offset)
        .stream()
        .map_ok(|x| SimplePort {
            uuid: x.uuid,
            port: u16::from_ne_bytes(x.port.to_ne_bytes()),
            protocol: x.protocol,
            comment: x.comment,
            host: *x.host.key(),
        })
        .try_collect()
        .await?;

    tx.commit().await?;

    Ok(Json(PortResultsPage {
        items: ports,
        limit,
        offset,
        total: total as u64,
    }))
}