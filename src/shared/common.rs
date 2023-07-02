use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Format the response as JSON instead of the default text
// actix_web::error::ErrorBadRequest(err)
// ref: https://stackoverflow.com/questions/64291039/how-to-return-the-error-description-in-a-invalid-json-request-body-to-the-client
// also see the following url for solution for all errors:
// ref: https://stackoverflow.com/questions/57878917/why-does-an-actix-web-service-send-text-plain-instead-of-json-for-an-auth-error
// actix_web::error::InternalError::from_response(
//     "",
//     HttpResponse::BadRequest()
//         .content_type("application/json")
//         .body(format!(r#"{{"error":"{}"}}"#, err)),
// )

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = r#"{{"error":"Internal Server Error - {}"}}"#, _0)]
    InternalServerError(String),

    #[display(fmt = r#"{{"error":"{}"}}"#, _0)]
    BadRequest(String),

    // #[display(fmt = r#"{{"error":"Unauthorized"}}"#)]
    // Unauthorized,

    #[display(fmt = r#"{{"error":"{} not Found"}}"#, _0)]
    NotFound(String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError(ref _message) => HttpResponse::InternalServerError()
                .content_type("application/json")
                .body(self.to_string()),
            ServiceError::BadRequest(ref _message) => HttpResponse::BadRequest()
                .content_type("application/json")
                .body(self.to_string()),
            // ServiceError::Unauthorized => HttpResponse::Unauthorized()
            //     .content_type("application/json")
            //     .body(self.to_string()),
            ServiceError::NotFound(ref _message) => HttpResponse::NotFound()
                .content_type("application/json")
                .body(self.to_string()),
        }
    }
}
