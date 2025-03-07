pub mod game;
pub mod blockchain;

use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(game::configure_routes)
            .configure(blockchain::configure_routes),
    );
}

