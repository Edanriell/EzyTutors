use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            // A POST request to /courses to create a new course
            .route("/", web::post().to(post_new_course))
            // A GET request to /courses/{tutor_id} to retrieve all courses for a tutor
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
            // A GET request to /courses/{tutor_id}/{course_id} to retrieve the details for a particular course_id
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_details)),
    );
}