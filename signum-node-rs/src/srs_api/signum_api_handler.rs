use actix_web::{
    web::{Data, Json},
    Responder,
};

use crate::{
    configuration::PeerToPeerSettings,
    srs_api::{add_peers, get_info, get_peers},
};

use super::request_models;

#[tracing::instrument(skip_all)]
pub async fn signum_api_handler(
    settings: Data<PeerToPeerSettings>,
    request_object: Json<request_models::RequestType>,
) -> Result<impl Responder, actix_web::Error> {
    tracing::debug!("Request Object: {:#?}", &request_object);
    let settings = settings.into_inner();

    match request_object.0 {
        request_models::RequestType::AddPeers { peers } => add_peers::add_peers_handler(peers),
        request_models::RequestType::GetInfo(payload) => {
            get_info::get_info_handler(payload, &settings)
        }
        request_models::RequestType::GetPeers {} => get_peers::get_peers_handler(),
    }
}
