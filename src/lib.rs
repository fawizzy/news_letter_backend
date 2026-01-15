pub mod configuration;
pub mod routes;
pub mod startup;


use actix_web::{ HttpRequest,  Responder};


async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}", name)
}







