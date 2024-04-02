use serde::{Deserialize, Serialize};

// define a struct for the request body
#[derive(Deserialize)]
pub struct Request {
    pub id: i32,
    pub sensor_name: String,
    pub data: String,
    pub location: String,
}



// define a struct for the query parameters
#[derive(Deserialize)]
pub struct Query {
    pub id: i32,
}


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: i32,
    pub sensor_name: String,
    pub location :String,
    pub data :String,
}


#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: i32,
    pub sensor_name: String,
    pub location: String,
    pub data : String
}


#[derive(Deserialize, Serialize)]
pub struct Getid {
    pub id:i32
}




#[derive(Serialize, Deserialize)]
pub struct TokenInformation {
    pub id_token:String,
    pub access_token: String,
    pub refesh_token:String
}




pub struct Signin{
    pub username:String,
    pub password:String,
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


#[derive(Serialize, Deserialize)]
pub struct SignOutBody {
    pub access_token: String
}





#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub username: String,
    pub password: String,
}





#[derive(Clone)]
pub struct TokenWrapper(pub String);




#[derive(Deserialize)]
pub struct ConfirmSignUpBody {
    pub username: String,
    pub confirmation_code: String,
}


#[derive(Serialize)]
pub struct ConfirmSignUpResponse {
    pub message: String,
}
