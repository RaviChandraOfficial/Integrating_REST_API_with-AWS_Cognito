use crate::sensor::{CurrentUser, NoteModel, NoteModelResponse};
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
        let notes = sqlx::query_as("SELECT * FROM sensor")
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



    fn filter_db_record(note: &NoteModel) -> NoteModelResponse {
        NoteModelResponse {
            // id: note.id.to_owned(),
            // user_name: note.user_name.to_owned(),
            // location: note.location.to_owned(),
            // data: note.data.to_owned(),
            sensor_id:note.sensor_id.to_owned(),
            value:note.value.to_owned(),
        }
    }
    



