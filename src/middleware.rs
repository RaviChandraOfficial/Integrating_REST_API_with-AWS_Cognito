use axum::{extract::Request, http::{self, StatusCode}, middleware::Next, response::Response};
use jsonwebtokens_cognito::KeySet;
use crate::{auth, sensor::CurrentUser};

pub async fn middle_ware_function(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok()).unwrap();

    let user_pool_region = std::env::var("USER_POOL_REGION").unwrap();
    let user_pool_id = std::env::var("USER_POOL_ID").unwrap();
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let keyset = KeySet::new(user_pool_region, user_pool_id).unwrap();
    let verifier = keyset
        .new_access_token_verifier(&[&client_id])
        .build()
        .unwrap();
    
    match keyset.verify(&auth_header, &verifier).await {
        Ok(result) => {
            if let Some(username) = result.get("username").and_then(|v| v.as_str()) {
                request.extensions_mut().insert(CurrentUser { username: username.to_string() });
            } else {
                // Log or handle the absence of a username more specifically if needed
                println!("Username missing ");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
    
    Ok(next.run(request).await)
    
}

