use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder,web::Path,web::Json};
mod models;
use crate::models::ppl::{AddPeopleRequest,UpdatedPeopleURL};
use validator::Validate;

#[get("/peoples")]
async fn get_pizza() -> impl Responder {
    HttpResponse::Ok().body("Pizzzas available!")
}

#[post("/addpeople")]
async fn buy_pizza(body: Json<AddPeopleRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid{
        Ok(_) => {
            let people_name = body.people_name.clone();
            HttpResponse::Ok().body(format!("People added is {people_name}"))
        }
        Err(_) => HttpResponse::Ok().body("People name required")
    }
}

#[patch("/updatepeople")]
async fn update_pizza(updated_people_url: Path<UpdatedPeopleURL>) -> impl Responder {
    let uuid = updated_people_url.into_inner().uuid;
    
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
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
