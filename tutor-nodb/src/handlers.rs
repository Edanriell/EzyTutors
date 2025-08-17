use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::models::Course;
use chrono::Utc;

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

// The handler function takes two parameters: data payload from HTTP request and application state.
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock()
        // Since the course collection is
        // protected by a Mutex, we have
        // to lock it first to access the data
        .unwrap()
        .clone()
        // Convert the course collection (stored within AppState) into an iterator so
        // that we can iterate through each element in the collection for processing.
        .into_iter()
        // Review each element in the collection, and filter
        // only for the courses corresponding to the tutor_id
        // (received as part of the HTTP request).
        .filter(|course| course.tutor_id == new_course.tutor_id)
        // The number of elements in the filtered list is retrieved.
        // This is used to generate the ID for the next course.
        .count();
    // Create a new course instance
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1) as i32),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    // Add the new course instance to the course collection that is part
    // of the application state (AppState).
    app_state.courses.lock().unwrap().push(new_course);
    // Send back an HTTP response to the web client.
    HttpResponse::Ok().json("Added course")
}

// The #[cfg(test)] annotation on the tests module tells Rust to
// compile and run the tests only when the Cargo test command
// is run, and not for the cargo build or cargo run commands.
#[cfg(test)]
// Tests in Rust are written within the tests module.
mod tests {
    // Import all handler declarations from the parent module (which hosts the tests module).
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    // Normal Rust tests are annotated with #[test], but since this is an asynchronous
    // test function, we have to alert the async runtime of Actix
    // Web to execute this async test function.
    #[actix_rt::test]
    async fn post_course_test() {
        // Construct a web::Json<T> object representing
        // the request data payload (the new course data from the tutor).
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        // Construct a web::Data<T> object representing the application state.
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        // Invoke the handler function with application
        // state and a simulated request data payload.
        let resp = new_course(course, app_state).await;
        // Verify whether the HTTP status response code (returned from the handler) indicates success.
        assert_eq!(resp.status(), StatusCode::OK);
    }
}