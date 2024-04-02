use std::collections::HashMap;

use actix_web::web::Json;
use actix_web::web::Path;
use actix_web::HttpResponse;
use futures::TryStreamExt;
use ipnetwork::IpNetwork;
use rorm::and;
use rorm::conditions::DynamicCollection;
use rorm::db::sql::value::Value;
use rorm::insert;
use rorm::prelude::ForeignModelByField;
use rorm::query;
use rorm::update;
use rorm::FieldAccess;
use rorm::Model;
use uuid::Uuid;

use crate::api::extractors::SessionUser;
use crate::api::handler::aggregation_source::schema::FullAggregationSource;
use crate::api::handler::aggregation_source::schema::SimpleAggregationSource;
use crate::api::handler::common::error::ApiError;
use crate::api::handler::common::error::ApiResult;
use crate::api::handler::common::schema::Page;
use crate::api::handler::common::schema::PathUuid;
use crate::api::handler::common::schema::SimpleTag;
use crate::api::handler::common::schema::UuidResponse;
use crate::api::handler::common::utils::get_page_params;
use crate::api::handler::common::utils::query_many_severities;
use crate::api::handler::common::utils::query_single_severity;
use crate::api::handler::findings::schema::ListFindings;
use crate::api::handler::hosts::schema::SimpleHost;
use crate::api::handler::ports::schema::CreatePortRequest;
use crate::api::handler::ports::schema::FullPort;
use crate::api::handler::ports::schema::GetAllPortsQuery;
use crate::api::handler::ports::schema::PathPort;
use crate::api::handler::ports::schema::PortRelations;
use crate::api::handler::ports::schema::UpdatePortRequest;
use crate::api::handler::services::schema::SimpleService;
use crate::chan::global::GLOBAL;
use crate::chan::ws_manager::schema::AggregationType;
use crate::chan::ws_manager::schema::WsMessage;
use crate::models::AggregationSource;
use crate::models::AggregationTable;
use crate::models::FindingAffected;
use crate::models::GlobalTag;
use crate::models::Host;
use crate::models::ManualPort;
use crate::models::Port;
use crate::models::PortGlobalTag;
use crate::models::PortWorkspaceTag;
use crate::models::Service;
use crate::models::Workspace;
use crate::models::WorkspaceTag;
use crate::modules::filter::GlobalAST;
use crate::modules::filter::PortAST;
use crate::modules::raw_query::RawQueryBuilder;
use crate::query_tags;
/// List the ports of a workspace
#[swaggapi::post("/workspaces/{uuid}/ports/all", tags("Ports"))]
pub async fn get_all_ports(
    path: Path<PathUuid>,
    params: Json<GetAllPortsQuery>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<Page<FullPort>>> {
    let path = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    let (limit, offset) = get_page_params(params.page).await?;

    let global_filter = params
        .global_filter
        .as_deref()
        .map(GlobalAST::parse)
        .transpose()?
        .unwrap_or_default();

    let port_filter = params
        .port_filter
        .as_deref()
        .map(PortAST::parse)
        .transpose()?
        .unwrap_or_default();

    // Count host's uuid instead of directly service's to force the implicit join required by the conditions
    let mut count_query = RawQueryBuilder::new((Port::F.host.uuid.count(),));
    let mut select_query = RawQueryBuilder::new((
        Port::F.uuid,
        Port::F.port,
        Port::F.protocol,
        Port::F.certainty,
        Port::F.comment,
        Port::F.created_at,
        Port::F.host.select_as::<Host>(),
        Port::F.workspace,
    ));

    port_filter.apply_to_query(&global_filter, &mut count_query);
    port_filter.apply_to_query(&global_filter, &mut select_query);

    count_query.append_eq_condition(Port::F.workspace, Value::Uuid(path.uuid));
    select_query.append_eq_condition(Port::F.workspace, Value::Uuid(path.uuid));

    if let Some(host_uuid) = params.host {
        count_query.append_eq_condition(Port::F.host, Value::Uuid(host_uuid));
        select_query.append_eq_condition(Port::F.host, Value::Uuid(host_uuid));
    }

    select_query.order_desc(Port::F.created_at);
    select_query.limit_offset(limit, offset);

    let (total,) = count_query.one(&mut tx).await?;
    let ports: Vec<_> = select_query.stream(&mut tx).try_collect().await?;

    let mut tags = HashMap::new();

    query_tags!(
        tags,
        tx,
        (
            PortWorkspaceTag::F.workspace_tag as WorkspaceTag,
            PortWorkspaceTag::F.port
        ),
        PortWorkspaceTag::F.port,
        (
            PortGlobalTag::F.global_tag as GlobalTag,
            PortGlobalTag::F.port
        ),
        PortGlobalTag::F.port,
        ports.iter().map(|x| x.0)
    );

    let mut sources = SimpleAggregationSource::query(
        &mut tx,
        path.uuid,
        AggregationTable::Port,
        ports.iter().map(|x| x.0),
    )
    .await?;

    let severities =
        query_many_severities(&mut tx, FindingAffected::F.port, ports.iter().map(|x| x.0)).await?;

    tx.commit().await?;

    let items = ports
        .into_iter()
        .map(
            |(uuid, port, protocol, certainty, comment, created_at, host, workspace)| FullPort {
                uuid,
                port: port as u16,
                protocol,
                comment,
                certainty,
                host: SimpleHost {
                    uuid: host.uuid,
                    ip_addr: host.ip_addr.ip(),
                    os_type: host.os_type,
                    response_time: host.response_time,
                    certainty: host.certainty,
                    workspace: *host.workspace.key(),
                    comment: host.comment,
                    created_at: host.created_at,
                },
                workspace: *workspace.key(),
                tags: tags.remove(&uuid).unwrap_or_default(),
                sources: sources.remove(&uuid).unwrap_or_default(),
                severity: severities.get(&uuid).copied(),
                created_at,
            },
        )
        .collect();

    Ok(Json(Page {
        items,
        limit,
        offset,
        total: total as u64,
    }))
}

/// Retrieve all information about a single port
#[swaggapi::get("/workspaces/{w_uuid}/ports/{p_uuid}", tags("Ports"))]
pub async fn get_port(
    path: Path<PathPort>,

    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<FullPort>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges)?;
    }

    let port = query!(&mut tx, Port)
        .condition(and!(
            Port::F.workspace.equals(path.w_uuid),
            Port::F.uuid.equals(path.p_uuid)
        ))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    let host = query!(&mut tx, Host)
        .condition(Host::F.uuid.equals(*port.host.key()))
        .one()
        .await?;

    let mut tags: Vec<_> = query!(&mut tx, (PortGlobalTag::F.global_tag as GlobalTag,))
        .condition(PortGlobalTag::F.port.equals(path.p_uuid))
        .stream()
        .map_ok(|(tag,)| SimpleTag::from(tag))
        .try_collect()
        .await?;

    let global_tags: Vec<_> = query!(
        &mut tx,
        (PortWorkspaceTag::F.workspace_tag as WorkspaceTag,)
    )
    .condition(PortWorkspaceTag::F.port.equals(path.p_uuid))
    .stream()
    .map_ok(|(tag,)| SimpleTag::from(tag))
    .try_collect()
    .await?;

    tags.extend(global_tags);

    let sources = query!(&mut tx, (AggregationSource::F.source_type,))
        .condition(AggregationSource::F.aggregated_uuid.equals(host.uuid))
        .stream()
        .map_ok(|(x,)| x)
        .try_collect()
        .await?;

    let severity = query_single_severity(&mut tx, FindingAffected::F.port, path.p_uuid).await?;

    tx.commit().await?;

    Ok(Json(FullPort {
        uuid: port.uuid,
        port: port.port as u16,
        protocol: port.protocol,
        certainty: port.certainty,
        host: SimpleHost {
            uuid: host.uuid,
            ip_addr: host.ip_addr.ip(),
            os_type: host.os_type,
            response_time: host.response_time,
            certainty: host.certainty,
            comment: host.comment,
            workspace: path.w_uuid,
            created_at: host.created_at,
        },
        comment: port.comment,
        tags,
        sources,
        severity,
        workspace: path.w_uuid,
        created_at: port.created_at,
    }))
}

/// Manually add a port
#[swaggapi::post("/workspaces/{uuid}/ports", tags("Ports"))]
pub async fn create_port(
    req: Json<CreatePortRequest>,
    path: Path<PathUuid>,

    SessionUser(user): SessionUser,
) -> ApiResult<Json<UuidResponse>> {
    let CreatePortRequest {
        ip_addr,
        port,
        certainty,
        protocol,
    } = req.into_inner();
    let PathUuid { uuid: workspace } = path.into_inner();
    Ok(Json(UuidResponse {
        uuid: ManualPort::insert(
            &GLOBAL.db,
            workspace,
            user,
            IpNetwork::from(ip_addr),
            port,
            certainty,
            protocol,
        )
        .await?,
    }))
}

/// Update a port
///
/// You must include at least on parameter
#[swaggapi::put("/workspaces/{w_uuid}/ports/{p_uuid}", tags("Ports"))]
pub async fn update_port(
    req: Json<UpdatePortRequest>,
    path: Path<PathPort>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<HttpResponse> {
    let req = req.into_inner();

    if req.workspace_tags.is_none() && req.global_tags.is_none() && req.comment.is_none() {
        return Err(ApiError::EmptyJson);
    }

    let mut tx = GLOBAL.db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, path.w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    query!(&mut tx, (Port::F.uuid,))
        .condition(Port::F.uuid.equals(path.p_uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    if let Some(global_tags) = &req.global_tags {
        GlobalTag::exist_all(&mut tx, global_tags.iter().copied())
            .await?
            .ok_or(ApiError::InvalidUuid)?;

        rorm::delete!(&mut tx, PortGlobalTag)
            .condition(PortGlobalTag::F.port.equals(path.p_uuid))
            .await?;

        if !global_tags.is_empty() {
            insert!(&mut tx, PortGlobalTag)
                .return_nothing()
                .bulk(
                    &global_tags
                        .iter()
                        .map(|x| PortGlobalTag {
                            uuid: Uuid::new_v4(),
                            port: ForeignModelByField::Key(path.p_uuid),
                            global_tag: ForeignModelByField::Key(*x),
                        })
                        .collect::<Vec<_>>(),
                )
                .await?;
        }
    }

    if let Some(workspace_tags) = &req.workspace_tags {
        WorkspaceTag::exist_all(&mut tx, workspace_tags.iter().copied())
            .await?
            .ok_or(ApiError::InvalidUuid)?;

        rorm::delete!(&mut tx, PortWorkspaceTag)
            .condition(PortWorkspaceTag::F.port.equals(path.p_uuid))
            .await?;

        if !workspace_tags.is_empty() {
            insert!(&mut tx, PortWorkspaceTag)
                .return_nothing()
                .bulk(
                    &workspace_tags
                        .iter()
                        .map(|x| PortWorkspaceTag {
                            uuid: Uuid::new_v4(),
                            port: ForeignModelByField::Key(path.p_uuid),
                            workspace_tag: ForeignModelByField::Key(*x),
                        })
                        .collect::<Vec<_>>(),
                )
                .await?;
        }
    }

    if let Some(comment) = req.comment {
        update!(&mut tx, Port)
            .condition(Port::F.uuid.equals(path.p_uuid))
            .set(Port::F.comment, comment)
            .exec()
            .await?;
    }

    tx.commit().await?;

    // Send WS messages
    if let Some(tags) = req.workspace_tags {
        GLOBAL
            .ws
            .message_workspace(
                path.w_uuid,
                WsMessage::UpdatedWorkspaceTags {
                    uuid: path.p_uuid,
                    workspace: path.w_uuid,
                    aggregation: AggregationType::Port,
                    tags,
                },
            )
            .await;
    }
    if let Some(tags) = req.global_tags {
        GLOBAL
            .ws
            .message_workspace(
                path.w_uuid,
                WsMessage::UpdatedGlobalTags {
                    uuid: path.p_uuid,
                    workspace: path.w_uuid,
                    aggregation: AggregationType::Port,
                    tags,
                },
            )
            .await;
    }

    Ok(HttpResponse::Ok().finish())
}

/// Delete the port
///
/// This only deletes the aggregation. The raw results are still in place
#[swaggapi::delete("/workspaces/{w_uuid}/ports/{p_uuid}", tags("Ports"))]
pub async fn delete_port(
    path: Path<PathPort>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<HttpResponse> {
    let PathPort { w_uuid, p_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;

    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    query!(&mut tx, (Port::F.uuid,))
        .condition(and!(
            Port::F.uuid.equals(p_uuid),
            Port::F.workspace.equals(w_uuid)
        ))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    rorm::delete!(&mut tx, Port)
        // We can omit the check if the workspace is the same as we have already checked it in the query before
        .condition(Port::F.uuid.equals(p_uuid))
        .await?;

    tx.commit().await?;

    let msg = WsMessage::DeletedPort {
        workspace: w_uuid,
        port: p_uuid,
    };
    GLOBAL.ws.message_workspace(w_uuid, msg).await;

    Ok(HttpResponse::Ok().finish())
}

/// Get all data sources which referenced this port
#[swaggapi::get("/workspaces/{w_uuid}/ports/{p_uuid}/sources", tags("Ports"))]
pub async fn get_port_sources(
    path: Path<PathPort>,
    SessionUser(user_uuid): SessionUser,
) -> ApiResult<Json<FullAggregationSource>> {
    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, path.w_uuid, user_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }
    let source =
        FullAggregationSource::query(&mut tx, path.w_uuid, AggregationTable::Port, path.p_uuid)
            .await?;
    tx.commit().await?;
    Ok(Json(source))
}

/// Get a port's direct relations
#[swaggapi::get("/workspaces/{w_uuid}/ports/{p_uuid}/relations", tags("Ports"))]
pub async fn get_port_relations(path: Path<PathPort>) -> ApiResult<Json<PortRelations>> {
    let mut tx = GLOBAL.db.start_transaction().await?;

    let services = query!(&mut tx, Service)
        .condition(Service::F.port.equals(path.p_uuid))
        .stream()
        .map_ok(|s| SimpleService {
            uuid: s.uuid,
            name: s.name,
            version: s.version,
            certainty: s.certainty,
            host: *s.host.key(),
            port: s.port.map(|x| *x.key()),
            comment: s.comment,
            workspace: *s.workspace.key(),
            created_at: s.created_at,
        })
        .try_collect()
        .await?;

    let (host,) = query!(&mut tx, (Port::F.host as Host,))
        .condition(Port::F.uuid.equals(path.p_uuid))
        .optional()
        .await?
        .ok_or(ApiError::InvalidUuid)?;

    tx.commit().await?;

    Ok(Json(PortRelations {
        host: SimpleHost {
            uuid: host.uuid,
            ip_addr: host.ip_addr.ip(),
            response_time: host.response_time,
            certainty: host.certainty,
            os_type: host.os_type,
            comment: host.comment,
            workspace: *host.workspace.key(),
            created_at: host.created_at,
        },
        services,
    }))
}

/// Get a port's findings
#[swaggapi::get("/workspaces/{w_uuid}/ports/{p_uuid}/findings", tags("Ports"))]
pub async fn get_port_findings(
    path: Path<PathPort>,
    SessionUser(u_uuid): SessionUser,
) -> ApiResult<Json<ListFindings>> {
    let PathPort { w_uuid, p_uuid } = path.into_inner();

    let mut tx = GLOBAL.db.start_transaction().await?;
    if !Workspace::is_user_member_or_owner(&mut tx, w_uuid, u_uuid).await? {
        return Err(ApiError::MissingPrivileges);
    }

    let findings = ListFindings::query_through_affected(
        &mut tx,
        w_uuid,
        FindingAffected::F.port.equals(p_uuid),
    )
    .await?;

    tx.commit().await?;
    Ok(Json(findings))
}
