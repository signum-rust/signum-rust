use serde::Deserialize;

/// Represents each of the types of request that can be made to the SRS Peer to Peer API.
/// Currently ignores the 'protocol' field, since that is always `B1` and has never changed.
/// May need to include that later if SRS changes.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename_all_fields = "camelCase")]
#[serde(tag = "requestType")]
pub enum RequestType {
    AddPeers { peers: Vec<String> },
    GetInfo { payload: GetInfoRequestModel },
    GetPeers {},
}

#[derive(Debug, Deserialize)]
pub struct GetInfoRequestModel {
    pub announced_address: Option<String>,
    pub application: String,
    pub version: String,
    pub platform: String,
    pub share_address: bool,
    pub network_name: String,
}
