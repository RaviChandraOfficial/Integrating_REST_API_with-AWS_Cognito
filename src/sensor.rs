use serde::{Deserialize, Serialize};

// define a struct for the request body
#[derive(Deserialize)]
pub struct Request {
    pub id: i32,
    pub name:String,
    pub location: String,
    pub data: String
}



// define a struct for the query parameters
#[derive(Deserialize)]
pub struct Deleteuser {
    pub id: i32,
}


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: i32,
    pub user_name: String,
    pub location :String,
    pub data :String,
    pub name:String,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: i32,
    pub user_name: String,
    pub location: String,
    pub data : String,
    pub name:String
}


#[derive(Serialize, Deserialize)]
pub struct TokenInformation {
    pub id_token:String,
    pub access_token: String,
    pub refesh_token:String
}


#[derive(Serialize, Deserialize)]
pub struct SignUpBody {
    pub username: String,
    pub email:String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignInBody {
    pub username: String,
    pub password: String,
}



#[derive(Deserialize)]
pub struct ConfirmSignUpBody {
    pub username: String,
    pub otp: String,
}



#[derive(Debug,Clone)]
pub struct CurrentUser{
    pub username:String
}