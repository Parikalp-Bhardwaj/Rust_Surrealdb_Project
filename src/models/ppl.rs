use serde::{Deserialize,Serialize};
use validator::{Validate};

#[derive(Validate,Deserialize,Serialize)]
pub struct AddPeopleRequest{
    #[validate(length(min=1,message = "People name required"))]
    pub people_name: String,
    #[validate(range(min = 18, max = 40, message = "age should be in range"))]
    pub age: i32,
    #[validate(length(min=1,message = "People country name required"))]
    pub country: String,    

}

#[derive(Validate,Deserialize,Serialize)]
pub struct UpdatedPeopleURL{
    pub uuid: String,
}

#[derive(Debug,Deserialize,Serialize,Validate)]
pub struct People{
    pub uuid: String,
    pub people_name: String,
    pub age: i32,
    pub country: String,
}

impl People{
    pub fn new(uuid: String, people_name: String, age: i32, country: String) -> People{
        People{
            uuid,
            people_name,
            age,
            country,
        }
    }
}