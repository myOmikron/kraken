/*
 * kraken
 *
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@omikron.dev
 * Generated by: https://openapi-generator.tech
 */

/// AggregatedPort : An open port on a host



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregatedPort {
    /// The port's uuid
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,
    /// Port number
    #[serde(rename = "port")]
    pub port: i32,
    #[serde(rename = "protocol")]
    pub protocol: crate::models::PortProtocol,
    /// The host this service is attached to
    #[serde(rename = "host")]
    pub host: uuid::Uuid,
    /// The services that link to this port
    #[serde(rename = "services")]
    pub services: Vec<uuid::Uuid>,
    /// A comment to the port
    #[serde(rename = "comment")]
    pub comment: String,
}

impl AggregatedPort {
    /// An open port on a host
    pub fn new(uuid: uuid::Uuid, port: i32, protocol: crate::models::PortProtocol, host: uuid::Uuid, services: Vec<uuid::Uuid>, comment: String) -> AggregatedPort {
        AggregatedPort {
            uuid,
            port,
            protocol,
            host,
            services,
            comment,
        }
    }
}


