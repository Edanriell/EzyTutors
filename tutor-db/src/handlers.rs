use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use super::errors::EzyTutorError;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
    // The code for the health_check_handler function
    // keeps track of how many times the handler is
    // invoked and records that in the application state
    // state.rs. It returns the visit count as part of the HTTP response.
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
// ) -> HttpResponse {
    // Change the web handler
    // method signature to
    // return a Result type.
) -> Result<HttpResponse, EzyTutorError> {
    // web::Path is an extractor that allows you to extract
    // typed information from the HTTP request’s path.
    // let tuple = params.0;
    // The data type returned by the web::Path
    // extractor for the get_courses_for_tutor()
    // handler function is <(i32),>.
    // let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
    // let tutor_id: i32 = tuple;
    // Invoke the corresponding database access
    // method to retrieve the list of courses
    // for a tutor, passing in the application
    // state and tutor-id.
    // let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    // In the get_course_details() handler function, retrieve
    // values for these two path parameters from the HTTP
    // request: tutor-id and course-id.
    // HttpResponse::Ok().json(courses)
    let tutor_id = path.into_inner();
    // The call is made to the database access function. Any error
    // returned is propagated by the handler function to the Actix Web
    // framework, which converts it to an HTML response message.
    get_course_for_tutor_db(&app_state.db, tutor_id)
        .await
        // If the database call is successful, the map logic is processed and the list of query results is returned.
        .map(|course| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    // let tuple = params;
    // let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
    // In the get_course_details() handler function, retrieve
    // values for these two path parameters from the HTTP
    // request: tutor-id and course-id.
    // let course_id: i32 = i32::try_from(tuple.1).unwrap();
    let (tutor_id, course_id) = (params.0,params.1);
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;

    HttpResponse::Ok().json(course)
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
        // let resp = get_courses_for_tutor(app_state, tutor_id).await;
        // Note the addition of .unwrap(). A Result type is being returned from
        // the handler method, but we want an HTTP Response, so we have to “unwrap” the result.
        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
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
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
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
            course_id: 3,
            tutor_id: 1,
            course_name: "Third course".into(),
            posted_time: Some(NaiveDate::from_ymd(2020, 12, 18).and_hms(05, 40, 00)),
        };
        let course_param = web::Json(new_course_msg);
        let resp = post_new_course(course_param, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}