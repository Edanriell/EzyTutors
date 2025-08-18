use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get()
            .to(health_check_handler));
}
 
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses")
            .route("/", web::post().to(new_course))
            // Add a new route for getting courses for a
            // tutor (represented by the tutor_id variable).
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
            // Add a new route to get course details.
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail)));
} 