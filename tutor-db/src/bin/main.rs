use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../db_access.rs"]
mod db_access;
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

// Used to run an asynchronous Actix web server, and to connect to the database using sqlx
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load the environment variables into memory.
    dotenv().ok();

    // Retrieve the value of the DATABASE_URL environment variable.
    // This value should be set either using a shell prompt or in an .env file.
    let database_url = env::var("DATABASE_URL").expect(
        "DATABASE_URL is not set in .env file");
    // Create a database connection pool with sqlx. This helps to manage the number of database
    // connections efficiently across multiple threads spawned by the Actix Web framework.
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // Define the query to be executed.
    // let course_rows = sqlx::query!(
    //     r#"select course_id, tutor_id, course_name, posted_time from
    //     ezy_course where course_id = $1"#,1
    // )
        // Fetch all rows from the table, passing the reference to the database connection pool.
        // .fetch_all(&db_pool)
        // .await
        // .unwrap();

    // let mut courses_list = vec![];

    // for course_row in course_rows {
    //     courses_list.push(Course {
    //         course_id: course_row.course_id,
    //         tutor_id: course_row.tutor_id,
    //         course_name: course_row.course_name,
    //         posted_time: Some(chrono::NaiveDateTime::from(
    //             course_row.posted_time.unwrap())),
    //     })
    // }

    // println!("Courses = {:?}", courses_list);

    // Ok(())

    // Construct AppState. Note that we are storing the connection pool as
    // part of the application state in the db field.
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    // Construct app and configure routes
    let app = move || {
        App::new()
            // Inject the connection pool into the Actix web application instance as a
            // cross-application dependency. This will be made available to the handler
            // functions by the Actix Web framework.
            // Inject the app state into the application instance.
            .app_data(shared_data.clone())
            // Configure the routes.
            .configure(general_routes)
            .configure(course_routes)
    };

    // Start the Actix web server, load the constructed Actix web application
    // instance, and bind the server running on localhost to port 3000. The
    // await keyword indicates the asynchronous nature of the Actix web server.
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}