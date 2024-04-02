use chrono::DateTime;
use chrono::Utc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::api::handler::common::de_optional;
use crate::api::handler::finding_definitions::schema::SimpleFindingDefinition;
use crate::chan::ws_manager::schema::AggregationType;
use crate::models::FindingSeverity;

/// The request to create a new finding
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateFindingRequest {
    /// Name of the new finding definition
    ///
    /// This must be unique
    pub definition: Uuid,

    /// The severity of this specific instance of the finding
    pub severity: FindingSeverity,

    /// Notes about the finding provided by the user
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

/// The request to update an existing finding
// The `#[serde(skip_serializing_if = "Option::is_none")]` is required by the frontend.
// The update is echoed over the websocket to allow live editing
// and the frontend needs to differentiate between no update and set to `None`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UpdateFindingRequest {
    /// Name of the new finding definition
    ///
    /// This must be unique
    #[serde(skip_serializing_if = "Option::is_none")] // see above
    pub definition: Option<Uuid>,

    /// The severity of this specific instance of the finding
    #[serde(skip_serializing_if = "Option::is_none")] // see above
    pub severity: Option<FindingSeverity>,

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

/// A simple finding
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleFinding {
    /// The uuid of the finding
    pub uuid: Uuid,

    /// The uuid of the finding definition
    pub definition: Uuid,

    /// The name of the finding definition
    pub name: String,

    /// The CVE of the finding definition
    pub cve: Option<String>,

    /// The severity of the finding
    pub severity: FindingSeverity,

    /// The count of affected aggregations
    pub affected_count: u64,

    /// The point in time this finding definition was created
    pub created_at: DateTime<Utc>,
}

/// A full finding
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FullFinding {
    /// The uuid of the finding
    pub uuid: Uuid,

    /// The finding's definition
    pub definition: SimpleFindingDefinition,

    /// The severity of the finding
    pub severity: FindingSeverity,

    /// List of all affected objects
    pub affected: Vec<SimpleFindingAffected>,

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

    /// The point in time this finding was created
    pub created_at: DateTime<Utc>,
}

/// The uuid's for objects affected by findings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimpleFindingAffected {
    /// The finding this affected belongs to
    pub finding: Uuid,

    /// The affected object's type
    pub affected_type: AggregationType,

    /// The affected object's uuid
    pub affected_uuid: Uuid,
}

/// A list of simple findings
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListFindings {
    /// The findings
    pub findings: Vec<SimpleFinding>,
}

/// The path parameter of a finding
#[derive(Serialize, Deserialize, JsonSchema, Debug, Copy, Clone)]
pub struct PathFinding {
    /// Workspace uuid
    pub w_uuid: Uuid,
    /// Finding uuid
    pub f_uuid: Uuid,
}
