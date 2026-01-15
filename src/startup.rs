use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::{greet, routes::{health_check, subscribe}};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
   let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/greet", web::get().to(greet))
            .route("/subscription", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();


    Ok(server)
}