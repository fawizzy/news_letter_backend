use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use actix_web::middleware::Logger;
use sqlx::PgPool;

use crate::{
    greet,
    routes::{health_check, subscribe},
};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/greet", web::get().to(greet))
            .route("/subscription", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
