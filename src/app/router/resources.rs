pub mod favicon;

use actix_web::web::{self, ServiceConfig};
use favicon::favicon;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(web::resource("/favicon.ico").route(web::get().to(favicon)));
}
