use super::state::AppState;
use actix_web::{web, HttpResponse};

// Application state registered with the Actix web application is made available
// to all handler functions as an extractor object of type web::Data<T>, where T
// is the type of the custom application state that developers have defined.
pub async fn health_check_handler(app_state: web::Data<AppState>) ->
HttpResponse {
    // Data members of the application state struct (AppState)
    // can be directly accessed using standard dot notation.
    let health_check_response = &app_state.health_check_response;
    // A field representing shared mutable state (visit_count) has
    // to be locked first before accessing to prevent multiple
    // threads from updating the value of the field simultaneously.
    let mut visit_count = app_state.visit_count.lock().unwrap();
    // Construct a response string to send back to the browser client.
    let response = format!("{} {} times", health_check_response,
                           visit_count);
    // Update the value of the field representing shared
    // mutable state. Since the lock on this data has already
    // been acquired, the value of the field can be updated
    // safely. The lock on the data is automatically released
    // when the handler function finishes execution.
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}