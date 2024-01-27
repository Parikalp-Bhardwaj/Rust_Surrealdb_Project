use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder,web::Path,web::Json, web::Data};
mod models;
use crate::models::ppl::{AddPeopleRequest,UpdatedPeopleURL,People};
use validator::Validate;
mod db;
use crate::db::Database;
use uuid::Uuid;


#[get("/get_people")]
async fn get_people(db: Data<Database>) -> impl Responder {
    // HttpResponse::Ok().body("Pizzzas available!")
    let people = db.get_all_people().await;
    println!("{:?} ",people);
    match people {
        Some(found_people) => {HttpResponse::Ok().body(format!("{:?}",found_people ))},
        None => HttpResponse::Ok().body("Error!"),
    }

}

#[post("/add_people")]
async fn add_people(body: Json<AddPeopleRequest>, db:Data<Database>) -> impl Responder {
    let is_valid = body.validate();
    println!("{:?} is_valid ",is_valid);
    match is_valid{
        Ok(_) => {
            let people_name = body.people_name.clone();
            let _age = body.age;
            let _country = body.country.clone();
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_people = db.add_new_people(People::new(String::from(
                new_uuid), 
                people_name,
                _age,
                _country)).await;
            match new_people{
                Some(added_people) => {
                    HttpResponse::Ok().body(format!("People added is {:?}",added_people))
                },
                None => HttpResponse::Ok().body(format!("Error added people")),
            }
            
        }
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .map(|(field, field_errors)| {
                    let field_error_messages: Vec<String> = field_errors
                        .iter()
                        .map(|error| format!("{}: {}", field, error.message.clone().unwrap_or_default()))
                        .collect();
                    field_error_messages.join(", ")
                })
                .collect();

            HttpResponse::BadRequest().body(format!("Validation error: {}", error_messages.join(", ")))
        }
        // Err(ValidationErrors({"age": Field([ValidationError { code: "range", message: Some("age should be in range"), params: {"max": Number(40.0), "min": Number(18.0), "value": Number(17)} }])})) is_valid 
    }
}

#[patch("/updatepeople")]
async fn update_people(updated_people_url: Path<UpdatedPeopleURL>) -> impl Responder {
    let uuid = updated_people_url.into_inner().uuid;

    
    HttpResponse::Ok().body("Updating a people!")
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init()
            .await
            .expect("error connecting to database");
    
    let db_data = Data::new(db);
    HttpServer::new(move|| {
        App::new()
            .app_data(db_data.clone())
            .service(get_people)
            .service(add_people)
            .service(update_people)
            
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
