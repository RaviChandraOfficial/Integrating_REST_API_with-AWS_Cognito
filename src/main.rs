use axum::{http::HeaderValue, middleware::from_fn, routing::{delete, get, post, put}, Extension, Router};
use my_rest_api::{auth,  handler, middleware::middle_ware_function};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use aws_sdk_cognitoidentityprovider::Client;
use dotenv::dotenv;

use axum::http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},Method};

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);


    // Initialize a connection pool to the PostgreSQL database with specific configurations.
    let pool = PgPoolOptions::new()
        .connect(&db_connection_str)                // Connect to the database using the connection string.
        .await         // Asynchronously wait for the connection to be established.
        .expect("can't connect to database");       // Panic if the connection cannot be established.



    // Configure CORS settings for the application
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
    // .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);


    println!("Connected to url:");
    // Create the Axum application with routes and middleware
    let app = create_router(pool,client).layer(cors);

    // Prepare a TCP listener on port 3000 of all network interfaces.
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

     // Launch the Axum web server to handle incoming HTTP requests.
    axum::serve(listener, app).await.unwrap();
}


pub fn create_router(pool:Pool<Postgres>,client:Client) -> Router {
    let app = Router::new()
    .route("/get/user", get(handler::get_data))          // Route for fetching all users.    // Route for fetching a user by ID.
    .route("/post/user", post(handler::post_data))       // Route for creating a new user.
    .route("/put/user", put(handler::put_data))          // Route for updating an existing user.
    .route("/delete/user", delete(handler::delete_data))
    .route("/signout", post(auth::sign_out))
    .route_layer(from_fn(middle_ware_function))
    .route("/signup", post(auth::sign_up))
    .route("/signup_confirm", post(auth::confirm_sign_up))
    .route("/signin", post(auth::sign_in))
    .layer(Extension(client))
    .with_state(pool);                                       
    app
}


