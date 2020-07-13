pub mod routes;
pub mod auth;
pub mod handlers;

use serde_derive::*;
use crate::db::Db;

#[derive(Serialize, Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppData {
    pub jwt_secret: String,
    pub secret_key: String,
    pub db: Db,
}

//impl Into<User> for UserLogin {
    //fn into(self) -> User {
        //User {
            //id: None, email: self.email,
            //username: self.username, 
            //password: self.password,
            //created_at: Utc::now().timestamp() as i32,
        //}
    //}
//}
