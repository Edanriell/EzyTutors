use crate::dbaccess::course::*;
use crate::errors::EzyTutorError;
use crate::models::course::{CreateCourse, UpdateCourse};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    // params: web::Path<(i32,)>,
    path: web::Path<i32>,
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
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        // If the database call is successful, the map logic is processed and the list of query results is returned.
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    // params: web::Path<(i32, i32)>,
    path: web::Path<(i32, i32)>
    // ) -> HttpResponse {
    // Change the handler function signature to return a Result type.
) -> Result<HttpResponse, EzyTutorError> {
    // let tuple = params;
    // let tutor_id: i32 = i32::try_from(tuple.0).unwrap();
    // In the get_course_details() handler function, retrieve
    // values for these two path parameters from the HTTP
    // request: tutor-id and course-id.
    // let course_id: i32 = i32::try_from(tuple.1).unwrap();
    // let (tutor_id, course_id) = (params.0,params.1);
    // let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    // HttpResponse::Ok().json(course)
    let (tutor_id, course_id) = path.into_inner();
    get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        // Invoke the database access function to retrieve
        // the course details. If it’s successful, return the
        // course details in the body of the HTTP response.
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
    // ) -> HttpResponse {
    // Change the return value of the handler function into a Result type.
) -> Result<HttpResponse, EzyTutorError> {
    // let course = post_new_course_db(&app_state.db, new_course.into()).await;
    // HttpResponse::Ok().json(course)
    post_new_course_db(&app_state.db, new_course.into()?)
        .await
        // If the call to the database access function is
        // successful, return the new course details. On failure,
        // propagate errors to the Actix Web framework.
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorError> {
    let (tutor_id, course_id) = path.into_inner();
    delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    // Module imports
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        // Read database access credentials from the .env file.
        // Retrieve the DATABASE_URL from the environment
        // variable.If the variable is not set, the code will panic
        // with an error message.
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
        // let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let tutor_id: web::Path<i32> = web::Path::from(1);
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
    async fn get_course_detail_success_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        // let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        // Construct path parameters representing tutor_id and course_id.
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 2));
        // Note the addition of .unwrap() in the call to the database access
        // function to extract the HTTP Response from the Result type.
        let resp = get_course_details(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        // The call to the handler function, which returns a Result<T,E> type.
        let resp = get_course_details(app_state, parameters).await;
        // We use the match clause to check if the handler function returns
        // successfully or returns an Error. In this case, we are trying to retrieve details
        // for a non-existent course-id, so we’re expecting an error to be returned.
        match resp {
            Ok(_) => println!("Something wrong"),
            // We are asserting that the error status code returned from the handler function
            // is of type StatusCode::NOT_FOUND.
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
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
        // Construct a data structure representing the attributes of the course to be created.
        let new_course_msg = CreateCourse {
            tutor_id: 1,
            course_name: "Third course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("English".into()),
            course_structure: None,
        };
        // Encapsulate the constructed CreateCourse struct in a
        // web::Json object to simulate what happens in a client API call.
        let course_param = web::Json(new_course_msg);
        // let resp = post_new_course(course_param, app_state).await;
        // Add unwrap() on the result value returned by the handler
        // to extract the HTTP response from the Result type returned
        // by the post_new_course() database access function.
        let resp = post_new_course(course_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        // Similar to the CreateCourse struct in the previous test
        // case, we are using the UpdateCourse struct to
        // provide the data elements to modify the Course record
        // in the database.
        let update_course_msg = UpdateCourse {
            course_name: Some("Course name changed".into()),
            course_description: Some("This is yet another test course".into()),
            course_format: None,
            course_level: Some("Intermediate".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("German".into()),
            course_structure: None,
        };
        // Simulate the URL path parameters to uniquely identify
        // a course record in the database using tutor_id and course_id.
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let update_param = web::Json(update_course_msg);
        let resp = update_course_details(app_state, update_param, parameters)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_test_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        // Ensure that a valid tutor-id and course-id are provided in the URL path
        // parameters before invoking this test case.
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 5));
        // let parameters: web::Path<(i32, i32)> = web::Path::from((3, 5));
        let resp = delete_course(app_state, parameters).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_test_failure() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        // Provide an invalid course-id or tutor-id in the path parameters.
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = delete_course(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            // Expect an error to be returned from the handler
            // function, and compare the error status code
            // returned by the handler with the expected value.
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}