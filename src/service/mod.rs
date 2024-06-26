mod auth;
mod user;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    user::init(cfg);
    auth::init(cfg);
}
