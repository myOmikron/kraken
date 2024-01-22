//! Implementation of the [`AttackResultsService`] and [`BacklogService`]
//!
//! As well as the [`start_rpc_server`] starting both.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;

use kraken_proto::backlog_service_server::{BacklogService, BacklogServiceServer};
use kraken_proto::push_attack_service_server::{PushAttackService, PushAttackServiceServer};
use kraken_proto::{
    any_attack_response, push_attack_request, AnyAttackResponse, BacklogRequest, BacklogResponse,
    PushAttackRequest, PushAttackResponse,
};
use log::{error, info, warn};
use rorm::{query, FieldAccess, Model};
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use crate::chan::global::GLOBAL;
use crate::config::Config;
use crate::models::{AttackType, InsertAttackError, Leech, LeechApiKey, Workspace};
use crate::modules::attacks::{AttackContext, HandleAttackResponse};

/// Helper type to implement result handler to
pub struct Results;

#[tonic::async_trait]
impl PushAttackService for Results {
    async fn push_attack(
        &self,
        request: Request<PushAttackRequest>,
    ) -> Result<Response<PushAttackResponse>, Status> {
        let PushAttackRequest {
            workspace_uuid,
            api_key,
            response: Some(response),
        } = request.into_inner()
        else {
            return Err(Status::invalid_argument("Missing attack response"));
        };
        let workspace_uuid = Uuid::try_parse(&workspace_uuid)
            .map_err(|_| Status::invalid_argument("Invalid UUID supplied"))?;

        let mut tx = GLOBAL
            .db
            .start_transaction()
            .await
            .map_err(status_from_database)?;

        // Check api key and get user
        let (user,) = query!(&mut tx, (LeechApiKey::F.user,))
            .condition(LeechApiKey::F.key.equals(api_key))
            .optional()
            .await
            .map_err(status_from_database)?
            .ok_or(Status::new(Code::Unauthenticated, "Invalid api key"))?;
        let user_uuid = *user.key();

        if !Workspace::is_user_member_or_owner(&mut tx, workspace_uuid, user_uuid)
            .await
            .map_err(status_from_database)?
        {
            return Err(Status::permission_denied(
                "You are not part of this workspace",
            ));
        }

        tx.commit().await.map_err(status_from_database)?;

        let attack = AttackContext::new(
            workspace_uuid,
            user_uuid,
            match &response {
                push_attack_request::Response::DnsResolution(_) => AttackType::DnsResolution,
                push_attack_request::Response::HostsAlive(_) => AttackType::HostAlive,
                push_attack_request::Response::TcpPortScan(_) => AttackType::TcpPortScan,
                push_attack_request::Response::BruteforceSubdomain(_) => {
                    AttackType::BruteforceSubdomains
                }
                push_attack_request::Response::CertificateTransparency(_) => {
                    AttackType::QueryCertificateTransparency
                }
                push_attack_request::Response::ServiceDetection(_) => AttackType::ServiceDetection,
                push_attack_request::Response::UdpServiceDetection(_) => {
                    AttackType::UdpServiceDetection
                }
            },
        )
        .await
        .map_err(|e| match e {
            InsertAttackError::DatabaseError(x) => status_from_database(x),
            InsertAttackError::WorkspaceInvalid => Status::internal("Workspace does not exist"),
            InsertAttackError::UserInvalid => unreachable!("User was queried beforehand"),
        })?;

        let result = match response {
            push_attack_request::Response::DnsResolution(repeated) => {
                attack.handle_vec_response(repeated.responses).await
            }
            push_attack_request::Response::HostsAlive(repeated) => {
                attack.handle_vec_response(repeated.responses).await
            }
            push_attack_request::Response::TcpPortScan(repeated) => {
                attack.handle_vec_response(repeated.responses).await
            }
            push_attack_request::Response::BruteforceSubdomain(repeated) => {
                attack.handle_vec_response(repeated.responses).await
            }
            push_attack_request::Response::CertificateTransparency(response) => {
                attack.handle_response(response).await
            }
            push_attack_request::Response::ServiceDetection(response) => {
                attack.handle_response(response).await
            }
            push_attack_request::Response::UdpServiceDetection(repeated) => {
                attack.handle_vec_response(repeated.responses).await
            }
        };

        Ok(Response::new(PushAttackResponse {
            uuid: attack.set_finished(result).await.to_string(),
        }))
    }
}

#[tonic::async_trait]
impl BacklogService for Results {
    async fn submit_backlog(
        &self,
        request: Request<BacklogRequest>,
    ) -> Result<Response<BacklogResponse>, Status> {
        auth_leech(&request).await?;

        let mut attack_cache = HashMap::new();

        let entries = request.into_inner().responses;
        for entry in entries {
            let AnyAttackResponse {
                attack_uuid,
                response: Some(response),
            } = entry
            else {
                continue;
            };

            let Ok(attack_uuid) = Uuid::from_str(&attack_uuid) else {
                error!("Malformed attack uuid: {attack_uuid}");
                continue;
            };

            let Some(attack_context) = (match attack_cache.entry(attack_uuid) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(
                    AttackContext::existing(attack_uuid)
                        .await
                        .map_err(status_from_database)?,
                ),
            })
            .as_ref() else {
                warn!("Unknown attack uuid: {attack_uuid}");
                continue;
            };

            let result: Result<(), _> = match response {
                any_attack_response::Response::DnsResolution(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::HostsAlive(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::TcpPortScan(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::BruteforceSubdomain(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::CertificateTransparency(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::ServiceDetection(response) => {
                    attack_context.handle_response(response).await
                }
                any_attack_response::Response::UdpServiceDetection(response) => {
                    attack_context.handle_response(response).await
                }
            };

            if let Err(error) = result {
                error!("Backlog entry failed: {error}");
            }
        }

        Ok(Response::new(BacklogResponse {}))
    }
}

/// Authenticates a leech by checking the `x-leech-secret` header.
pub async fn auth_leech<T>(request: &Request<T>) -> Result<(), Status> {
    let secret = request
        .metadata()
        .get("x-leech-secret")
        .ok_or_else(|| Status::unauthenticated("Missing `x-leech-secret` header"))?;
    let secret = secret
        .to_str()
        .map_err(|_| Status::unauthenticated("Invalid `x-leech-secret`"))?;
    query!(&GLOBAL.db, (Leech::F.uuid,))
        .condition(Leech::F.secret.equals(secret))
        .optional()
        .await
        .map_err(status_from_database)?
        .ok_or_else(|| Status::unauthenticated("Invalid `x-leech-secret`"))?;
    Ok(())
}

/// Starts the gRPC server
///
/// **Parameter**:
/// - `config`: Reference to [Config]
///
/// Returns an error if the rpc listen address is invalid
pub fn start_rpc_server(config: &Config) -> Result<(), AddrParseError> {
    let listen_address = config.server.rpc_listen_address.parse()?;
    let listen_port = config.server.rpc_listen_port;
    let tls_config = GLOBAL.tls.tonic_server();

    tokio::spawn(async move {
        info!("Starting gRPC server");
        // TLS config should be valid is it is constructed by our TLS manager
        #[allow(clippy::expect_used)]
        if let Err(err) = Server::builder()
            .tls_config(tls_config)
            .expect("The tls config should be valid")
            .add_service(PushAttackServiceServer::new(Results))
            .add_service(BacklogServiceServer::new(Results))
            .serve(SocketAddr::new(listen_address, listen_port))
            .await
        {
            // TODO: add loop to continuously restart the gRPC server
            error!("Error running gRPC server: {err}");
        }
    });

    Ok(())
}

/// Convert [`rorm::Error`] to [`tonic::Status`]
fn status_from_database(err: rorm::Error) -> Status {
    error!("Database error in rpc endpoint: {err}");
    Status::new(Code::Internal, "Database error")
}
