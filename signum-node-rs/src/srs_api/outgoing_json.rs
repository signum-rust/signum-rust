use actix_web::ResponseError;
use anyhow::Context;
use serde::Serialize;

use crate::configuration::PeerToPeerSettings;

pub struct OutgoingJsonBuiler {
    protocol: String,
    settings: PeerToPeerSettings,
}

impl OutgoingJsonBuiler {
    pub fn new(settings: &PeerToPeerSettings) -> Self {
        Self {
            protocol: "B1".to_string(),
            settings: settings.clone(),
        }
    }

    pub fn get_info(&self) -> OutgoingGetInfoRequest {
        OutgoingGetInfoRequest::new(self.protocol.clone(), &self.settings)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingGetInfoRequest {
    protocol: String,
    request_type: String,
    announced_address: String,
    application: String,
    version: String,
    platform: String,
    share_address: bool,
    network_name: String,
}

impl OutgoingRequest for OutgoingGetInfoRequest {}

impl OutgoingGetInfoRequest {
    pub(crate) fn new(protocol: String, settings: &PeerToPeerSettings) -> Self {
        Self {
            protocol,
            request_type: "getInfo".to_string(),
            announced_address: settings.my_address.clone(),
            application: "SignumRust".to_string(),
            version: "0.1.0".to_string(),
            platform: settings.platform.clone(),
            share_address: settings.share_address,
            network_name: settings.network_name.clone(),
        }
    }
}

pub trait OutgoingRequest: Serialize {
    fn finish(&self) -> Result<serde_json::Value, OutgoingRequestError> {
        Ok(serde_json::to_value(self).context("couldn't parse json from struct")?)
    }
}

#[derive(thiserror::Error)]
pub enum OutgoingRequestError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for OutgoingRequestError {}

impl std::fmt::Debug for OutgoingRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}
