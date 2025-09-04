use super::models::Course;
use super::errors::EzyTutorError;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32
    // ) -> Vec<Course> {
    // The function returns a Result<T> type
    // representing two possible outcomes:
    // Vec<Course> in the case of success, or
    // the EzyTutorError error type on failure.
) -> Result<Vec<Course>, EzyTutorError> {
    // Prepare the SQL statement for
    // retrieving query results using the
    // query! macro from the sqlx crate.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1",
        tutor_id)
        // Execute the query.
        .fetch_all(pool)
        // Replace await.unwrap() with await?. This converts
        // the sqlx error to an EzyTutorError and
        // propagates it to the calling web handler function.
        .await?;
    // .unwrap();
    // Convert the query results into a Rust vector,
    // which is returned from the function.
    // course_rows
    //     .iter()
    //     .map(|course_row| Course {
    //         course_id: course_row.course_id,
    //         tutor_id: course_row.tutor_id,
    //         course_name: course_row.course_name.clone(),
    //         posted_time: Some(chrono::NaiveDateTime::from(
    //             course_row.posted_time.unwrap())),
    //     })
    //     .collect()

    let courses: Vec<Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
        .collect();
    // If there are no query results for the tutor_id, return an error of
    // type EzyTutorError, which will generate a message for the user
    match courses.len() {
        0 => Err(EzyTutorError::NotFound("Courses not found for tutor".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32
    // ) -> Course {
    // The function returns a Result type such
    // that a course is returned from the function
    // on success, and an error of type
    // EzyTutorError is returned on failure.
) -> Result<Course, EzyTutorError> {
    // Prepare the query for execution.
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1 and course_id = $2",
        tutor_id, course_id)
        // Execute the query.
        .fetch_one(pool)
        .await;
    // .unwrap();
    // Return a Rust Course data structure from the function.
    // Course {
    //     course_id: course_row.course_id,
    //     tutor_id: course_row.tutor_id,
    //     course_name: course_row.course_name.clone(),
    //     posted_time: Some(chrono::NaiveDateTime::from(
    //         course_row.posted_time.unwrap())),
    // }

    // If the specified course_id is not
    // available in the database, it returns
    // a custom error message.
    if let Ok(course_row) = course_row {
        // Execute query
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: Course
    // ) -> Course {
    // The function returns a Result type, where in a
    // successful insert into the database returns the new
    // course details or an error is returned on failure.
) -> Result<Course, EzyTutorError> {
    let course_row = sqlx::query!(
        "insert into ezy_course (course_id, tutor_id, course_name) values ($1,$2,$3) returning tutor_id, course_id,course_name, posted_time",
        // Prepare the query to insert a new course into the database table.
        new_course.course_id, new_course.tutor_id, new_course.course_name)
        // After inserting, fetch the inserted course.
        .fetch_one(pool)
        // Note the use of ? to convert sqlx errors into
        // EzyTutorError types and propagate them
        // back to the calling handler function.
        .await?;
    // .await.unwrap();
    // Return a Rust Course data structure from the function.
    // Course {
    //     course_id: course_row.course_id,
    //     tutor_id: course_row.tutor_id,
    //     course_name: course_row.course_name.clone(),
    //     posted_time: Some(chrono::NaiveDateTime::from(
    //         course_row.posted_time.unwrap())),
    // }

    //Retrieve result
    Ok(Course {
        // Return a Result type with Ok(<Course>).
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(course_row.posted_time.unwrap()),
    })
}