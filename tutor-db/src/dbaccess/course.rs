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
    // let course_rows = sqlx::query!(
        // "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1",
        // tutor_id)
        // Execute the query.
        // .fetch_all(pool)
        // Replace await.unwrap() with await?. This converts
        // the sqlx error to an EzyTutorError and
        // propagates it to the calling web handler function.
        // .await?;
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

    // let courses: Vec<Course> = course_rows
    //     .iter()
    //     .map(|course_row| Course {
    //         course_id: course_row.course_id,
    //         tutor_id: course_row.tutor_id,
    //         course_name: course_row.course_name.clone(),
    //         posted_time: Some(course_row.posted_time.unwrap()),
    //     })
    //     .collect();
    // If there are no query results for the tutor_id, return an error of
    // type EzyTutorError, which will generate a message for the user
    // match courses.len() {
    //     0 => Err(EzyTutorError::NotFound("Courses not found for tutor".into())),
    //     _ => Ok(courses),
    // }


    // Construct a query using the sqlx query_as!
    let course_rows: Vec<Course> = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1 order by course_id desc",
        tutor_id
    )
        // Execute the SELECT query statement to retrieve
        // all rows that match the selection clause in SQL.
        .fetch_all(pool)
        // Indicates an async function that internally uses Rust futures.
        // In Rust, futures are lazily evaluated, which means that until
        // the .await keyword is called, the query is not executed.
        .await?;

    // The sqlx library automatically converts the database
    // row into a Rust Course data struct, and a vector of
    // these courses is returned from this function.
    Ok(course_rows)
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
    // let course_row = sqlx::query!(
    //     "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course where tutor_id = $1 and course_id = $2",
    //     tutor_id, course_id)
    //     // Execute the query.
    //     .fetch_one(pool)
    //     .await;
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
    // if let Ok(course_row) = course_row {
    //     // Execute query
    //     Ok(Course {
    //         course_id: course_row.course_id,
    //         tutor_id: course_row.tutor_id,
    //         course_name: course_row.course_name.clone(),
    //         posted_time: Some(course_row.posted_time.unwrap()),
    //     })
    // } else {
    //     Err(EzyTutorError::NotFound("Course id not found".into()))
    // }

    // The query_as! macro is used to
    // map the returned database
    // record into a Course struct.
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        // fetch_optional returns an Option type,
        // indicating that there may not be a record in
        // the database for the specified SELECT clause.
        .fetch_optional(pool)
        .await?;


    // If a record is found in the database,
    // return the course details encapsulated
    // in the OK(T) variant of the Result type.
    if let Some(course) = course_row {
        Ok(course)
    } else {
        // If no record is found for the criteria specified, return an Err type with a suitable
        // error message. This error is then propagated back to the calling handler function
        // and sent to the API client as part of an HTTP response message.
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
    // let course_row = sqlx::query!(
    //     "insert into ezy_course (course_id, tutor_id, course_name) values ($1,$2,$3) returning tutor_id, course_id,course_name, posted_time",
    //     // Prepare the query to insert a new course into the database table.
    //     new_course.course_id, new_course.tutor_id, new_course.course_name)
    //     // After inserting, fetch the inserted course.
    //     .fetch_one(pool)
    //     // Note the use of ? to convert sqlx errors into
    //     // EzyTutorError types and propagate them
    //     // back to the calling handler function.
    //     .await?;
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
    // Ok(Course {
    //     // Return a Result type with Ok(<Course>).
    //     course_id: course_row.course_id,
    //     tutor_id: course_row.tutor_id,
    //     course_name: course_row.course_name.clone(),
    //     posted_time: Some(course_row.posted_time.unwrap()),
    // })

    // First, a standard insert SQL statement
    // is constructed using the parameters
    // passed from the handler function.
    let course_row= sqlx::query_as!(Course,"insert into ezy_course (tutor_id, course_name, course_description,course_duration, course_level, course_format, course_language, course_structure, course_price) values ($1,$2,$3,$4,$5,$6,$7,$8,$9) returning tutor_id, course_id,course_name, course_description, course_duration, course_level, course_format, course_language, course_structure, course_price, posted_time",
    new_course.tutor_id, new_course.course_name, new_course.course_description,
    new_course.course_duration, new_course.course_level, new_course.course_format, new_course.course_language, new_course.course_structure, new_course.course_price)
        .fetch_one(pool)
        .await?;

    // After inserting a record, the fetch_one() method is called to
    // return the inserted record. The retrieved database row is
    // automatically converted into the Course data type due to the use
    // of the query_as! macro. The newly created course is returned
    // to the handler function in the form of the Course struct.
    Ok(course_row)
}

pub async fn delete_course_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<String, EzyTutorError> {
    // Prepare SQL statement
    // Construct a SQL query to
    // delete the specified course
    // from the database.
    let rows_deleted = sqlx::query!(
        "DELETE FROM ezy_course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id,
    )
        // Execute the query statement. Note that because
        // this is an async function, the query is actually
        // executed only when .await() is invoked.
        .execute(pool)
        .await?;
    // Return a message confirming deletion.
    Ok(format!("Deleted {:#?} record", rows_deleted))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
    update_course: UpdateCourse,
) -> Result<Course, EzyTutorError> {
    // Construct a SQL query to verify
    // if a record exists in the database
    // for the criteria specified.
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM ezy_course where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
        // Fetch a single record.
        .fetch_one(pool)
        .await
        // If no record is found for the
        // specified tutor_id and course_id,
        // return an error message.
        .map_err(|_err| EzyTutorError::NotFound("Course id not found".into()))?;

    // Construct the values to update the database.
    let name: String = if let Some(name) = update_course.course_name {
        name
    } else {
        current_course_row.course_name
    };
    let description: String = if let Some(desc) = update_course.course_description {
        desc
    } else {
        current_course_row.course_description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.course_format {
        format
    } else {
        current_course_row.course_format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.course_structure {
        structure
    } else {
        current_course_row.course_structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.course_duration {
        duration
    } else {
        current_course_row.course_duration.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.course_level {
        level
    } else {
        current_course_row.course_level.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.course_language {
        language
    } else {
        current_course_row.course_language.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.course_price {
        price
    } else {
        current_course_row.course_price.unwrap_or_default()
    };

    // Construct the query statement to update the database.
    let course_row =
        sqlx::query_as!(
        Course,
        "UPDATE ezy_course set course_name = $1, course_description = $2, course_format = $3,
        course_structure = $4, course_duration = $5, course_price = $6, course_language = $7,
        course_level = $8 where tutor_id = $9 and course_id = $10 returning tutor_id, course_id,
        course_name, course_description, course_duration, course_level, course_format,
        course_language, course_structure, course_price, posted_time ", name, description, format,
        structure, duration, price, language,level, tutor_id, course_id
    )
            // Retrieve the updated record.
            .fetch_one(pool)
            .await;
    // Verify whether the update is successful.
    // If it is, return the updated course
    // record to the calling handler function.
    if let Ok(course) = course_row {
        Ok(course)
    } else {
        // If the update operation fails,
        // return an error message.
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}
