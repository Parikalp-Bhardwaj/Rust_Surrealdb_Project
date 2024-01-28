use crate::models::People;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Debug, Clone)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        client.use_ns("surreal").use_db("peoples").await.unwrap();
        Ok(Database {
            client,
            name_space: String::from("surreal"),
            db_name: String::from("peoples"),
        })
    }

    pub async fn get_all_people(&self) -> Option<Vec<People>> {
        let result = self.client.select("people").await;
        println!("result {:?} ", result);
        match result {
            Ok(all_people) => Some(all_people),
            Err(_) => None,
        }
    }

    pub async fn add_new_people(&self, new_people: People) -> Option<People> {
        let add_people_data = self
            .client
            .create(("people", new_people.uuid.clone()))
            .content(new_people)
            .await;

        match add_people_data {
            Ok(people_data) => people_data,
            Err(_) => None,
        }
    }

    pub async fn updated_people(&self, uuid: String, update_people: People) -> Option<People> {
        let find_people: Result<Option<People>, Error> =
            self.client.select(("people", &uuid)).await;

        match find_people {
            Ok(found) => match found {
                Some(_found_people) => {
                    let updated_people: Result<Option<People>, Error> = self
                        .client
                        .update(("people", &uuid))
                        // .merge(People {
                        //     uuid,
                        //     people_name: String::from("null"),
                        //     age: 0,
                        //     country: String::from("null"),
                        // })
                        .content(update_people)
                        .await;
                    match updated_people {
                        Ok(updated) => updated,
                        Err(_) => None,
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

}
