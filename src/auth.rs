use aws_sdk_cognitoidentityprovider::{types::{builders::AttributeTypeBuilder, AuthFlowType}, Client};
use axum::{http::{self, HeaderMap, StatusCode}, response::IntoResponse, Extension, Json};

use crate::sensor::{ConfirmSignUpBody, SignInBody, SignUpBody, TokenInformation};

use base64::{engine::general_purpose, Engine};
use ring::hmac;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Response {
    message:String
}

//HMAC (Hash-based Message Authentication Code)
fn generate_secret_hash(client_secret: &str, user_name: &str, client_id: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, client_secret.as_bytes());
    let msg = [user_name.as_bytes(), client_id.as_bytes()].concat();
    let signature = hmac::sign(&key, &msg);

    let encoded_hash = general_purpose::STANDARD.encode(signature.as_ref());

    encoded_hash
}


pub async fn sign_up(
    Extension(client): Extension<Client>,
    Json(body): Json<SignUpBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();

    let client_secret = generate_secret_hash(
        &std::env::var("CLIENT_SECRET").unwrap(),
        &body.username,
        &client_id,
    );
    let user_email = AttributeTypeBuilder::default()
    .name("email").value(&body.email).build().unwrap();

    let signup = client
        .sign_up().client_id(client_id)
        .secret_hash(client_secret)
        .username(&body.username)
        .password(&body.password)
        .user_attributes(user_email);

    match signup.send().await {
        Ok(response) => {
            let success_response = if response.user_confirmed {
                serde_json::json!({
                    "status": "success","message": "user confirmed succesfully."
                })
            } else {
                serde_json::json!({
                    "status": "success","message": "User requires confirmation. Check email for a verification code."
                })
            };

            Ok((StatusCode::CREATED, Json(success_response)))
        }
        Err(error) => {
            let error_response = serde_json::json!({
                "status": "error","message": format!("{}",error.to_string())
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }

}



pub async fn confirm_sign_up(
    Extension(client): Extension<Client>,
    Json(body): Json<ConfirmSignUpBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();

    let client_secret = generate_secret_hash(
        &std::env::var("CLIENT_SECRET").unwrap(),
        &body.username,
        &client_id,
    );


    let response = client.confirm_sign_up()
        .client_id(client_id)
        .secret_hash(client_secret)
        .username(&body.username)
        .confirmation_code(&body.otp)
        .send()
        .await;

        match response {
            Ok(_) => {
                
                let success_response = serde_json::json!({
                    "status": "success","message": "User is confirmed and ready to use."
                });
                Ok((StatusCode::OK, Json(success_response)))
            }
            Err(error) => {
                let error_response = serde_json::json!({
                    "status": "error","message": format!("{}",error.to_string())
                });
                Err((StatusCode::OK, Json(error_response)))
            }
        }
}


 
pub async fn  sign_in(
    Extension(client): Extension<Client>,
    Json(body): Json<SignInBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = generate_secret_hash(
        &std::env::var("CLIENT_SECRET").unwrap(),
        &body.username,
        &client_id,
    );

    let _user_pool_id = std::env::var("USER_POOL_ID").unwrap();

        let response = client.initiate_auth()
            .client_id(client_id)
            .auth_flow(AuthFlowType::UserPasswordAuth)
            .auth_parameters("USERNAME", &body.username)
            .auth_parameters("PASSWORD", &body.password)
            .auth_parameters("SECRET_HASH", client_secret)
            .send()
            .await;
    match response {
        Ok(value) => {
            let response = TokenInformation {
                id_token:value
                .authentication_result()
                .unwrap()
                .id_token()
                .unwrap()
                .to_string(),
                access_token: value.authentication_result().unwrap().access_token().unwrap().to_string(),
                refesh_token:value.authentication_result().unwrap().refresh_token().unwrap().to_string(),
            };

            

            Ok((StatusCode::OK, Json(response)))
        },
        Err(e) => {
            let error = serde_json::json!({ "error": e.to_string() });
            Err((StatusCode::UNAUTHORIZED, Json(error)))
        }
    }
}







// pub async fn sign_out(
    
//     Extension(client): Extension<Client>,
//     headers: HeaderMap
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let auth_header = headers
//         .get(http::header::AUTHORIZATION)
//         .ok_or(StatusCode::BAD_REQUEST).unwrap()
//         .to_str()
//         .unwrap();

//     let sign_out = client.global_sign_out()
//     .access_token(auth_header).send().await;


//     match sign_out{
//         Ok(_) => {
//             let success_response = serde_json::json!({
//                 "status": "success","message": "User is logged out"
//             });
//             Ok((StatusCode::OK, Json(success_response)))
//         },
//         Err(error) => {
//             let error_response = serde_json::json!({
//                 "status": "error","message": format!("{}",error.to_string())
//             });
//             Err((StatusCode::OK, Json(error_response)))
//         },
//     }
// }


pub async fn sign_out(
    Extension(client): Extension<Client>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .ok_or(StatusCode::BAD_REQUEST).unwrap()
        .to_str()
        .unwrap();

    let global_sign_out_builder = client.global_sign_out()
    .access_token(auth_header).send().await;
   
     
     match global_sign_out_builder{
        
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


