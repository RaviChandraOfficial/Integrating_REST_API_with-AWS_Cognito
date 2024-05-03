use crate::sensor::{CurrentUser, Deleteuser, NoteModel, NoteModelResponse, Request};
use axum::{response::IntoResponse, Extension};
use serde_json::json;

use axum::{
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use axum::extract::State;




/// Retrieves all sensor records from the `sensor_list` table.
///
/// This asynchronous function queries the database for all sensor records. It transforms the retrieved
/// records into a more convenient format for the response. If successful, it returns all sensor records
/// in JSON format; otherwise, it provides an appropriate error response.
///
/// 
///
/// * `State(pool)` - The database connection pool used to access the database asynchronously.
///
/// 
///
/// - A successful response with HTTP status code `200 OK` and a JSON object containing all sensor records.
/// - An error response with HTTP status code `500 Internal Server Error` if there is a problem accessing the database.
///
/// 
///
/// The function can return an error if there is a problem accessing the database, such as a connection issue,
/// which prevents the query from executing successfully.



//  GET request to fetch all sensor data from the database.
pub async fn get_data(
    Extension(current_user): Extension<CurrentUser>,

    State(pool): State<PgPool>,// state: wrapper used for sharing the data  accross asynchronus tasks
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let notes = sqlx::query_as("SELECT * FROM sensor_list  WHERE user_name = $1")
        .bind(current_user.username)
        .fetch_all(&pool) // Fetches all records asynchronously.
        .await      // Waits for the database operation to complete.
        .map_err(|e| {                 // Error handling in case the database query fails.
        // Constructs a JSON response for the error case.
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        // Returns an internal server error status along with the JSON error message.
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    
// Maps each database record to a NoteModelResponse structure for the response.
    let note_responses = notes
        .iter()
        .map(|note| filter_db_record(&note))// Applies the filter_db_record function to each note.
        .collect::<Vec<NoteModelResponse>>();       // Collects the results into a vector.
 // Constructs the final JSON response with the status, total number of notes, and the note data.
    let json_response = serde_json::json!({
        "status": "success",
        "results": note_responses.len(),            // Includes the count of all notes.
        "notes": note_responses                 // Includes the serialized note data.
    });
    // Returns the JSON response with a success status.
    Ok(Json(json_response))
    }











/// Inserts a new sensor record into the `sensor_list` table.
///
/// This asynchronous function accepts a JSON request payload containing the details of the sensor to be inserted.
/// It validates the request, inserts the new record into the database, and returns a response indicating
/// the success or failure of the operation.
///
/// 
///
/// - `#[axum::debug_handler]`: Marks this function for special logging and debugging by Axum, 
///   providing more detailed errors if the function's signature is incorrect.
///
/// 
///
/// * `State(pool)`: The database connection pool used to access the database asynchronously.
/// * `Json(request)`: A JSON payload containing the new sensor's details (`id`, `sensor_name`, `location`, `data`).
///
/// 
///
/// - A successful response with HTTP status code `200 OK` and a JSON object indicating success and the inserted record's ID and name.
/// - An error response with HTTP status code `400 Bad Request` or `500 Internal Server Error`, including a JSON object describing the error.
///
/// 
///
/// The function can return errors in several scenarios:
/// - If the request payload is invalid or incomplete.
/// - If inserting the record violates database constraints, such as duplicate IDs.
/// - If there's a database access error.


// Handler for the POST request to insert a new sensor data entry into the database.
#[axum::debug_handler]
pub async fn post_data(
    Extension(current_user): Extension<CurrentUser>,
     // Extracts the PostgreSQL connection pool from the application state.
    State(pool): State<PgPool>,// state: wrapper used for sharing the data  accross asynchronus tasks
    // Deserialize the incoming JSON request body into a `Request` struct.
    Json(request): Json<Request>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Extract fields from the request body.
    let id: i32 = request.id;
    let data = request.data;
    let location = request.location;
    let name = request.name;

    // Execute an INSERT query to add a new record to the sensor_list table.
    let _query_result =
        sqlx::query("INSERT INTO sensor_list (id,name,location, data,user_name) VALUES ($1, $2, $3, $4, $5)")
            .bind(id)
            .bind(name.to_string())
            .bind(location.to_string())
            .bind(data.to_string())
            .bind(current_user.username)
            .fetch_one(&pool)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        });
    
        let created_note = sqlx::query_as("SELECT * FROM sensor_list WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;
    
        let json_response = serde_json::json!({"status": "success","data": serde_json::json!({
            "note": filter_db_record(&created_note)
        })});
        Ok(Json(json_response))
}



fn filter_db_record(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        user_name: note.user_name.to_owned(),
        location: note.location.to_owned(),
        data: note.data.to_owned(),
        name:note.name.to_owned()
    }
}


/// Updates an existing record in the `sensor_list` table with new values.
///
/// This asynchronous function receives updated sensor information through a JSON payload,
/// attempts to update the corresponding record in the database, and returns a JSON response
/// indicating success or failure.
///
/// 
///
/// * `State(pool)` - The database connection pool, wrapped in Actix's `State` for shared state access.
/// * `Json(request)` - The JSON payload containing the updated sensor data, deserialized into a `Request` struct.
///
///
///
/// An `impl IntoResponse` which is either:
/// - A success response with HTTP status 200 and a JSON body containing the updated sensor ID and name.
/// - An error response with an appropriate HTTP status code (e.g., 500 for internal server error) and a JSON body detailing the error.
///
/// 
///
/// This function returns an error if:
/// - There's a problem with the database connection or query execution (e.g., constraint violations).
/// - The specified record does not exist or cannot be updated for some reason.




pub async fn put_data(
    Extension(current_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(request): Json<Request>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = request.id;
    let name = request.name;
    let data = request.data;
    let location = request.location;


    let _update_result= sqlx::query("UPDATE sensor_list SET name=$2 , location=$3 , data=$4 WHERE id=$1 and user_name =$5")
        .bind(id.clone())
        .bind(name.to_string())
        .bind(location.to_string())
        .bind(data.to_string())
        .bind(current_user.username)
        .execute(&pool)
        .await
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    });

    let updated_note = sqlx::query_as("SELECT * FROM sensor_list WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"status": "error","message": format!("{:?}", e)})),
        )
    })?;

    let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "note": filter_db_record(&updated_note)
    })});

    Ok(Json(note_response))
}


/// Deletes a sensor record from the `sensor_list` table based on a given ID.
///
/// This asynchronous function takes an ID from a JSON request, attempts to delete the corresponding
/// sensor record from the database, and returns an appropriate response.
///
/// 
///
/// * `State(pool)` - The database connection pool, allowing access to the database within the asynchronous function.
/// * `Json(request)` - A JSON payload containing the `id` of the sensor record to be deleted, deserialized into a `Query` struct.
///
/// 
///
/// - HTTP status code `204 No Content` on successful deletion, indicating that the operation was successful and there's no additional content to send in the response.
/// - An error response with an appropriate HTTP status code (e.g., `404 Not Found` if the sensor ID does not exist in the database, or `500 Internal Server Error` for any database access issues) and a JSON body detailing the error.
///
/// 
///
/// This function can result in an error response in the following scenarios:
/// - If there's an issue executing the delete operation (e.g., database connectivity problems).
/// - If the specified ID does not match any records in the database, resulting in a `404 Not Found` error.

pub async fn delete_data(
    Extension(current_user): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(request): Json<Deleteuser>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = request.id;
    // Execute an SQL DELETE operation to remove an entry from the `sensor_list` table
    let query_result = sqlx::query("DELETE FROM sensor_list WHERE id = $1 and user_name =$2")
        .bind(id)
        .bind(current_user.username)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            )
        })?;
    
    // Check if the DELETE operation affected any rows.
    
    match query_result.rows_affected() {
        0 => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Information with ID: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        },
        _ => {
            let response = serde_json::json!({
                "status": "Success",
                "message": format!("Information with ID: {} deleted successfully", id)
            });
            Ok(Json(response))
        },
    }
    
}


