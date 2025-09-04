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