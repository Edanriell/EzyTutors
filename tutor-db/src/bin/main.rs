use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
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

    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    //Construct app and configure routes
    let app = move || {
        App::new()
            // Inject the connection pool into the Actix web application instance as a
            // cross-application dependency. This will be made available to the handler
            // functions by the Actix Web framework.
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

#[cfg(test)]
mod tests {
    // Module imports
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();

        // Read database access credentials from the .env file.
        let database_url = env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set in .env file");
        // Create a new connection pool to talk to the Postgres database.
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        // Construct the application state that is to be
        // passed as a parameter to the handler function.
        // In an end-to-end test, the application state
        // would be passed by the Actix Web framework to
        // the handler function automatically. Here, in
        // unit-test code, we have to do this step manually.
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        // Construct the HTTP request parameter to pass to the
        // handler function. In an end-to-end test, the Actix
        // framework deserializes the incoming HTTP request
        // parameters and passes them to the handler function.
        // Here, in unit-test code, we have to do this step manually
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        // Invoke the handler function with the application state
        // and HTTP request parameter constructed in the previous steps.
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        // Verify that the returned HTTP response from the handler function shows the success status code.
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let new_course_msg = Course {
            course_id: 1,
            tutor_id: 1,
            course_name: "This is the next course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}