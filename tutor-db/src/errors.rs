use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
// Data structure to represent three types of errors that can
// occur in the web service: database-related errors, Actix
// server errors, and errors due to invalid client requests
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
// Data structure to display a suitable
// error message to the user or client
// sending the API request.
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for EzyTutorError {
    // Using this method, we can specify the
    // HTTP status code that should be sent
    // as part of the HTTP response message.
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_msg) | EzyTutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    // This method will be used to
    // determine the body of the HTML
    // response in case of error scenarios.
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}


// This enables us to print the EzyTutorError as a string that can be sent to the user.
impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

// This enables Actix Web errors to be converted to EzyTutorError using the question mark (?) operator.
impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}

// This enables database errors from sqlx to be converted to EzyTutorError using the question mark (?) operator.
impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}

// use std::fmt;
// use std::fs::File;
// use std::io::Write;
//
// #[derive(Debug)]
// // Define a custom error enum type
// // containing the set of error variants.
// pub enum MyError {
//     ParseError,
//     IOError,
// }
//
// // By convention, error types in
// // Rust implement the Error trait
// // from the Rust standard library.
// impl std::error::Error for MyError {}
//
// // The Rust Error trait requires the implementation of the
// // Debug and Display traits. The Debug trait is auto-derived. The
// // Display trait is implemented here.
// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             MyError::ParseError => write!(f, "Parse Error"),
//             MyError::IOError => write!(f, "IO Error"),
//         }
//     }
// }
//
// fn main() {
//     let result = square("INVALID");
//     // The square function is called, and the result is
//     // evaluated to print out a suitable message.
//     match result {
//         Ok(res) => println!("Result is {:?}",res),
//         Err(e) => println!("Error in parsing: {:?}",e)
//     };
// }
//
// fn square(val: &str) -> Result<i32, MyError> {
//     // The map_err method transforms parsing, file
//     // open, and file write errors into our MyError type,
//     // which is propagated back to the calling function
//     // through the ? operator
//     let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
//     let mut f = File::open("fictionalfile.txt").map_err(
//         |_| MyError::IOError)?;
//     let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(
//         num, 2));
//     f.write_all(string_to_write.as_bytes())
//         .map_err(|_| MyError::IOError)?;
//     Ok(i32::pow(num, 2))
// }