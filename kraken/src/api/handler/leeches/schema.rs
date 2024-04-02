use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use url::Url;
use uuid::Uuid;

use crate::api::handler::common::de_optional;

/// The request to create a new leech
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct CreateLeechRequest {
    /// Name of the leech
    pub name: String,
    /// Address of the leech with schema
    pub address: Url,
    /// Description of the leech
    pub description: Option<String>,
}

/// The request to update a leech
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct UpdateLeechRequest {
    /// Name of the leech
    pub name: Option<String>,
    /// Address of the leech
    pub address: Option<Url>,
    /// Description of the leech
    #[serde(default, deserialize_with = "de_optional")]
    pub description: Option<Option<String>>,
}

/// The simple representation of a leech
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct SimpleLeech {
    /// uuid of the leech
    pub uuid: Uuid,
    /// Name of the leech
    pub name: String,
    /// Address of the leech
    pub address: Url,
}

/// The response that hold all leeches
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct ListLeeches {
    /// The list of leeches
    pub leeches: Vec<SimpleLeech>,
}

/// The configuration of a leech
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct LeechConfig {
    /// The tls config
    #[serde(flatten)]
    pub tls: LeechTlsConfig,
    /// The secret of the leech
    pub secret: String,
}

/// The tls related part of a leech's config
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct LeechTlsConfig {
    /// PEM encoded CA managed by kraken
    pub ca: String,

    /// PEM encoded certificate
    pub cert: String,

    /// PEM encoded private key for the certificate
    pub key: String,

    /// The randomly generated fake domain for the kraken to be used for sni
    pub sni: String,
}
