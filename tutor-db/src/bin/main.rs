use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../db_access.rs"]
mod db_access;
#[path = "../iter4/errors.rs"]
mod errors;
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

    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");

    // Start the Actix web server, load the constructed Actix web application
    // instance, and bind the server running on localhost to port 3000. The
    // await keyword indicates the asynchronous nature of the Actix web server.
    HttpServer::new(app).bind(&host_port)?.run().await
}

// // The hello handler function can return one of two values:
// // HTTPResponse in the case of a successful computation,
// // or an Actix Error type in the case of failure.
// async fn hello() -> Result<HttpResponse, Error> {
//     // The handler function returns
//     // an HTTPResponse encapsulated
//     // in the Ok() enum variant.
//     // Ok(HttpResponse::Ok().body("Hello there!"));
//     // Try to open a nonexistent file in the handler function. The
//     // ? operator propagates the error to the calling function
//     // (which is the Actix web server itself, in this case).
//     let _ = File::open("fictionalfile.txt")?;
//     // If the file open is successful, return an
//     // HTTP response message with the success
//     // status code and a text message.
//     Ok(HttpResponse::Ok().body("File read successfully"))
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
//         .bind("127.0.0.1:3000")?
//         .run()
//         .await
// }