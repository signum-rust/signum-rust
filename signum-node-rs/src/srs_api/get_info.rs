use actix_web::HttpResponse;

use crate::configuration::PeerToPeerSettings;

use super::{
    outgoing_json::{OutgoingJsonBuiler, OutgoingRequest},
    request_models::GetInfoRequestModel,
};

pub(crate) fn get_info_handler(
    model: GetInfoRequestModel,
    settings: &PeerToPeerSettings,
) -> Result<HttpResponse, actix_web::Error> {
    let myinfo = OutgoingJsonBuiler::new(settings).get_info().finish()?;
    Ok(HttpResponse::Ok().json(myinfo))
    // HttpResponse::Ok().finish()
}
