use chrono::DateTime;
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::api::handler::common::de_optional;
use crate::api::handler::common::schema::SimpleTag;
use crate::api::handler::domains::schema::SimpleDomain;
use crate::api::handler::findings::schema::FullFinding;
use crate::api::handler::hosts::schema::SimpleHost;
use crate::api::handler::ports::schema::SimplePort;
use crate::api::handler::services::schema::SimpleService;
use crate::chan::ws_manager::schema::AggregationType;

/// The request to add a new object affected by a finding
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateFindingAffectedRequest {
    /// The object's uuid
    pub uuid: Uuid,

    /// The object's type
    pub r#type: AggregationType,

    /// Notes about the affected provided by the user
    ///
    /// May be used for documenting command invocation or other information
    /// that are provided by the user
    pub details: String,

    /// A screenshot
    ///
    /// The file must have been uploaded through the image upload.
    pub screenshot: Option<Uuid>,

    /// A log file
    pub log_file: Option<Uuid>,
}

/// The request to update an affected object's details
// The `#[serde(skip_serializing_if = "Option::is_none")]` is required by the frontend.
// The update is echoed over the websocket to allow live editing
// and the frontend needs to differentiate between no update and set to `None`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateFindingAffectedRequest {
    /// A screenshot
    ///
    /// The file must have been uploaded through the image upload.
    #[serde(skip_serializing_if = "Option::is_none")] // see above
    #[serde(default, deserialize_with = "de_optional")]
    pub screenshot: Option<Option<Uuid>>,

    /// A log file
    #[serde(skip_serializing_if = "Option::is_none")] // see above
    #[serde(default, deserialize_with = "de_optional")]
    pub log_file: Option<Option<Uuid>>,
}

/// An affected object's details and the finding it is affected by
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullFindingAffected {
    /// The finding this object is affected by
    pub finding: FullFinding,

    /// The affected object
    pub affected: FindingAffectedObject,

    /// List of tags for the affected object
    pub affected_tags: Vec<SimpleTag>,

    /// Notes about the finding provided by the user
    ///
    /// May be used for documenting command invocation or other information
    /// that are provided by the user
    pub user_details: String,

    /// Details of the finding that comes from the attack module
    ///
    /// This field should only be read-only for the user
    pub tool_details: Option<String>,

    /// The uuid to download a screenshot with
    pub screenshot: Option<Uuid>,

    /// The uuid to download a log file with
    pub log_file: Option<Uuid>,

    /// The point in time this object was attached to the finding
    pub created_at: DateTime<Utc>,
}

/// The object affected by a finding
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum FindingAffectedObject {
    /// An affected domain
    Domain(SimpleDomain),

    /// An affected host
    Host(SimpleHost),

    /// An affected port
    Port(SimplePort),

    /// An affected service
    Service(SimpleService),
}

/// The path parameter of an object affected by a finding
#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone)]
pub struct PathFindingAffected {
    /// Workspace uuid
    pub w_uuid: Uuid,

    /// Finding uuid
    pub f_uuid: Uuid,

    /// The affected object's uuid
    pub a_uuid: Uuid,
}
