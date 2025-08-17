use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// The #derive annotation derives the implementations for four traits: Deserialize,
// Serialize, Debug, and Clone. The first two are part of the serde crate and help to
// convert Rust data structs to and from on-the-wire formats.
// Implementing the Debug trait will help us print the Course
// struct values for debug purposes. The Clone trait
// helps address the Rust ownership
// rules during processing.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<i32>,
    pub course_name: String,
    // NativeDateTime is a chrono data type for storing timestamp information.
    pub posted_time: Option<NaiveDateTime>,
}

// This function will convert data from incoming HTTP requests to Rust structs.
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}