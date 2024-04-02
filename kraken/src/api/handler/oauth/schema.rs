use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::api::handler::oauth_applications::schema::SimpleOauthClient;
use crate::api::handler::workspaces::schema::SimpleWorkspace;

/// The information about an oauth request
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct OpenRequestInfo {
    /// Workspace about the open request
    pub workspace: SimpleWorkspace,
    /// The oauth application
    pub oauth_application: SimpleOauthClient,
}

/// Query parameters for `/accept` and `/deny`
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct OAuthDecisionQuery {
    /// Should kraken remember this decision?
    #[serde(default)]
    pub remember: bool,
}
