
use aws_sdk_cognitoidentityprovider::{error::SdkError, types::{AttributeType, AuthFlowType}, Client};
use axum::{http::{self, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json};
use serde_json::json;

use crate::sensor::{ConfirmSignUpBody, SignInBody, SignOutBody, SignUpBody, TokenInformation};



use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Response {
    message:String
}



#[derive(Serialize)]
struct ApiResponse {
    message: String,
}



pub async fn sign_up_handler(
    Extension(client): Extension<Client>,
    Json(body): Json<SignUpBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();

    let user_attributes = vec![
        AttributeType::builder().name("email").value(&body.email).build()]
    .into_iter()
    .filter_map(Result::ok) // Only keep Ok results, discarding any Err results
    .collect::<Vec<AttributeType>>();

    let response = client.sign_up()
        .client_id(client_id)
        .username(&body.username)
        .password(&body.password)
        .set_user_attributes(Some(user_attributes)) // Use set_user_attributes with Some()
        .send()
        .await;

    match response {
        Ok(_) => {
            let response = Response {
                message: "Sign up successful. Please check your email to confirm the account.".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        },
        Err(e) => {
            let error = json!({ "error": "Sign up failed", "details": e.to_string() });
            Err((StatusCode::BAD_REQUEST, Json(error)))
        }
    }
}








pub async fn confirm_sign_up_handler(
    Extension(client): Extension<Client>,
    Json(body): Json<ConfirmSignUpBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();


    let response = client.confirm_sign_up()
        .client_id(client_id)
        .username(&body.username)
        .confirmation_code(&body.confirmation_code)
        .send()
        .await;

    match response {
        Ok(_) => {
            Ok((StatusCode::OK, Json(json!({ "message": "Email verification successful" }))))
        },
        Err(e) => {
            println!("Email verification error: {:?}", e); // Log or handle the error appropriately
            let error_message = json!({
                "error": "Email verification failed",
                "details": e.to_string()
            });
            Err((StatusCode::BAD_REQUEST, Json(error_message)))
        }
    }
}



use std::fs;
 
pub async fn sign_in_handler(
    Extension(client): Extension<Client>,
    Json(body): Json<SignInBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();


        let response = client.initiate_auth()
            .client_id(client_id)
            .auth_flow(AuthFlowType::UserPasswordAuth)
            .auth_parameters("USERNAME", &body.username)
            .auth_parameters("PASSWORD", &body.password)
            .send()
            .await;
// try to print the response.
    match response {
        Ok(value) => {
            let response = TokenInformation {
                id_token:value.authentication_result().unwrap().id_token().unwrap().to_string(),
                access_token: value.authentication_result().unwrap().access_token().unwrap().to_string(),
                refesh_token:value.authentication_result().unwrap().refresh_token().unwrap().to_string()
            };
            Ok((StatusCode::OK, Json(response)))
        },
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            Err((StatusCode::UNAUTHORIZED, Json(error)))
        }
    }
}







pub async fn sign_out_handler(
    Extension(client): Extension<Client>,
    // Json(body): Json<SignOutBody>,
    headers: HeaderMap
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {


    // println!("sign out handler bro, auth_header checking: {:?}", headers
    // .get(http::header::AUTHORIZATION)
    // .ok_or(StatusCode::BAD_REQUEST).unwrap()
    // .to_str()
    // .unwrap());
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .ok_or(StatusCode::BAD_REQUEST).unwrap()
        .to_str()
        .unwrap();


    let global_sign_out_builder = client.global_sign_out()
    .access_token(auth_header);


    match global_sign_out_builder.send().await{
        Ok(_) => {
            let success_response = serde_json::json!({
                "status": "success","message": "User is logged out"
            });
            Ok((StatusCode::OK, Json(success_response)))
        },
        Err(error) => {
            let error_response = serde_json::json!({
                "status": "error","message": format!("{}",error.to_string())
            });
            Err((StatusCode::OK, Json(error_response)))
        },
    }
}



