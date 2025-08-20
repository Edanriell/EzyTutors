use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

// Define the data structure to represent a course.
#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

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
    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time from
        ezy_course where course_id = $1"#,1
    )
        // Fetch all rows from the table, passing the reference to the database connection pool.
        .fetch_all(&db_pool)
        .await
        .unwrap();

    let mut courses_list = vec![];

    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(
                course_row.posted_time.unwrap())),
        })
    }

    println!("Courses = {:?}", courses_list);

    Ok(())
}