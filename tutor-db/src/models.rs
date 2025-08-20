use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// The Course data structure contains the course ID, tutor ID, name of the course, and posted time as fields. Of these, the
// field posted_time is type Optional<T> because, for a new course posting, this field will be auto-populated
// by the tutor web service—the user does not need to provide this information.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// he From trait will extract the data payload sent with the POST HTTP
// request (for a new course) and convert it into the Rust Course data structure.
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            course_id: course.course_id,
            tutor_id: course.tutor_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}