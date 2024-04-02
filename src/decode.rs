use axum::{body::Body, http::StatusCode, response::Response};
use serde_json::Value;


fn parser_payload(o: Option<&str>) -> Result<Value, StatusCode> {
    match o {
        None => Err(StatusCode::EXPECTATION_FAILED),
        Some(part) => {
            let decoded = process_part(part)?;
            Ok(decoded)
        }
    }
  }
  
  
  fn process_part(part: &str) -> Result<Value, StatusCode> {
    let decoded = base64::decode_config(part, base64::URL_SAFE).unwrap();
    let decoded = std::str::from_utf8(&decoded).unwrap();
    let decoded = serde_json::from_str::<serde_json::Value>(decoded).unwrap();
    Ok(decoded)
  }
  
  

  pub async fn decode_token(jwt:String)->Result<Value, StatusCode>{
      let mut splits = jwt.split(".");
 
      let payload = parser_payload(splits.nth(0)).unwrap();
      let username = payload.get("username").unwrap().as_str().unwrap();
  
      println!("{:?}",username);
      Ok(username.into())

  }
  



pub async fn my_middleware(str: String) -> Response {
    let decode_result = decode_token(str).await;

    match decode_result {
        Ok(username) => {
            println!("print cheystunnadi neney ra bobby {:?}",username);
            Response::new(Body::from(format!("Username: {}", username)))
        }
        Err(status_code) => {
            // Handle decoding errors
            Response::builder()
                .status(status_code)
                .body(Body::empty()) // Or provide an error message in the body
                .unwrap() // Unwrap because Response::builder() never fails
        }
    }
}

