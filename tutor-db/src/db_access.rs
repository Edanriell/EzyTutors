use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // Prepare the SQL statement for
    // retrieving query results using the
    // query! macro from the sqlx crate.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1",
        tutor_id)
        // Execute the query.
        .fetch_all(pool)
        .await
        .unwrap();
    // Convert the query results into a Rust vector,
    // which is returned from the function.
    course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(
                course_row.posted_time.unwrap())),
        })
        .collect()
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    // Prepare the query for execution.
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1 and course_id = $2",
        tutor_id, course_id)
        // Execute the query.
        .fetch_one(pool)
        .await
        .unwrap();
    // Return a Rust Course data structure from the function.
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(
            course_row.posted_time.unwrap())),
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course_row = sqlx::query!(
        "insert into ezy_course (course_id,tutor_id, course_name) values ($1,$2,$3) returning tutor_id, course_id,course_name, posted_time",
        // Prepare the query to insert a new course into the database table.
        new_course.course_id, new_course.tutor_id, new_course.course_name)
        // After inserting, fetch the inserted course.
        .fetch_one(pool)
        .await.unwrap();
    // Return a Rust Course data structure from the function.
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(
            course_row.posted_time.unwrap())),
    }
}