use actix_web::HttpResponse;

pub(crate) fn get_info_handler(
    announced_address: Option<String>,
    application: String,
    version: String,
    platform: String,
    share_address: bool,
    network_name: String,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
