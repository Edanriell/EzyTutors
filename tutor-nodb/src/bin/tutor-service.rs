use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Initialize the application state.
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        // The courses field is initialized with a Mutex-protected empty vector.
        courses: Mutex::new(vec![])
    });
    // Define the web application.
    let app = move || {
        App::new()
            // Register the application state with the web application.
            .app_data(shared_data.clone())
            // Configure routes for the web application.
            .configure(general_routes)
            // Register the new course_routes group with the application.
            .configure(course_routes)
    };

    // Initialize the Actix web server with the web
    // application, listen on port 3000, and run the server.
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}