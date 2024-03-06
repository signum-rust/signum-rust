use actix_web::HttpResponse;

pub(crate) fn add_peers_handler(_peers: Vec<String>) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
