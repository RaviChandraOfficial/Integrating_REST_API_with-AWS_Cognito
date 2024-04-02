


use axum::extract::Extension::TypedHeader;

use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;





#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    email: String,
    // Add other fields as per your token structure
    // iss, aud, and exp are handled by jsonwebtoken
}



// Assuming `get_cognito_keys` fetches and caches JWKS keys from AWS Cognito
async fn get_cognito_keys() -> Result<DecodingKey, Box<dyn std::error::Error>> {
    // Fetch or retrieve your cached JWKS here
    // Convert JWKS to `DecodingKey`
    Ok(DecodingKey::from_rsa_pem(b"your_rsa_key_here")?) // Simplified, replace with actual logic
}

async fn verify_jwt(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let decoding_key = get_cognito_keys().await?;
    let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

// Your existing function with JWT verification integrated
pub async fn get_data(
    State(pool): State<PgPool>,
    TypedHeader(authorization): TypedHeader<headers::Authorization<headers::authorization::Bearer>>,

) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    // Extract the JWT from the authorization header
    let jwt = authorization.token();

    // Verify the JWT
    match verify_jwt(jwt).await {
        Ok(claims) => {
            // Proceed with your existing logic if the token is valid
            // Your existing database query and response construction goes here
        },
        Err(_) => {
            // Return an error response if the JWT is invalid
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid token",
            });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        },
    }
}
