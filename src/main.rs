use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};

#[get("/peoples")]
async fn get_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizzzas available!")
}

#[post("/addpeople")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying a pizza!")
}

#[patch("/updatepeople")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a pizza!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizza)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
