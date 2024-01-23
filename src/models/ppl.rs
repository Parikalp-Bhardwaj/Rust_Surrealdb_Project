use serde::{Deserialize,Serialize};
use validator::{Validate};

#[derive(Validate,Deserialize,Serialize)]
pub struct AddPeopleRequest{
    #[validate(length(min=1,message = "People name required"))]
    pub people_name: String,

}

#[derive(Validate,Deserialize,Serialize)]
pub struct UpdatedPeopleURL{
    pub uuid: String,
}