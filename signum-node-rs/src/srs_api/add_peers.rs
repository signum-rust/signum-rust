use actix_web::HttpResponse;

pub(crate) fn add_peers_handler(peers: Vec<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
