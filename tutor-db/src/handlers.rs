use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) ->
HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response,
                           visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
    // The code for the health_check_handler function
    // keeps track of how many times the handler is
    // invoked and records that in the application state
    // state.rs. It returns the visit count as part of the HTTP response.
}

pub async fn get_courses_for_tutor(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(i32, i32)>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}

pub async fn post_new_course(
    _new_course: web::Json<Course>,
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    HttpResponse::Ok().json("success")
}