use actix_web::HttpResponse;

pub(crate) fn get_info_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}
