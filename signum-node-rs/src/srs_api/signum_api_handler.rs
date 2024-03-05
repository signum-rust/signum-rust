use actix_web::{web::Json, HttpResponse, Responder};

use crate::srs_api::{add_peers, get_info, get_peers};

use super::request_models;

#[tracing::instrument(skip_all)]
pub async fn signum_api_handler(
    request_object: Json<request_models::RequestType>,
) -> impl Responder {
    tracing::debug!("Request Object: {:#?}", &request_object);
    let _response = match request_object.0 {
        request_models::RequestType::AddPeers { peers } => add_peers::add_peers_handler(peers),
        request_models::RequestType::GetInfo { payload } => get_info::get_info_handler(
            payload.announced_address,
            payload.application,
            payload.version,
            payload.platform,
            payload.share_address,
            payload.network_name,
        ),
        request_models::RequestType::GetPeers {} => get_peers::get_peers_handler(),
    };
    HttpResponse::Ok().finish()
}
