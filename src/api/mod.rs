use actix_web::web;

mod init;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(init::init_db_endpoint);
}
