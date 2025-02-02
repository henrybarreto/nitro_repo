use std::str::ParseBoolError;

use actix_web::http::header::ToStrError;
use actix_web::HttpResponse;

use crate::api_response::APIResponse;
use crate::error::internal_error::InternalError;
use actix_web::dev::Body;
use actix_web::http::StatusCode;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub enum RequestError {
    /// Not Authorized
    NotAuthorized,
    /// Invalid Login
    InvalidLogin,
    /// Not Found
    NotFound,
    /// Bad Request
    BadRequest(String),
    /// I am a Teapot
    IAmATeapot(String),
    /// Generic Error
    Error(String),
    /// Passwords are not matching
    MismatchingPasswords,
    /// That item already exists
    AlreadyExists,
    /// Missing arguments in your request
    MissingArgument(String),
    /// Nitro_Repo is not installed
    UnInstalled,
    /// Internal Errors that happen due to a library or error outside the users controll
    InternalError(InternalError),

}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for RequestError {}

impl RequestError {
    pub fn json_error(&self) -> HttpResponse {
        let response = self.to_json_response();
        let result = HttpResponse::Ok()
            .status(response.status.clone())
            .content_type("application/json")
            .body(response.value);
        return result;
    }
}

impl From<InternalError> for RequestError {
    fn from(err: InternalError) -> RequestError {
        RequestError::InternalError(err)
    }
}

#[derive(Debug)]
pub struct ErrorResponse {
    pub value: String,
    pub status: StatusCode,
}

impl RequestError {
    pub fn to_json_response(&self) -> ErrorResponse {
        match self {
            RequestError::NotAuthorized => {
                let response = APIResponse {
                    success: false,
                    data: Some(self.to_string()),
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::BadRequest(error) => {
                let response = APIResponse {
                    success: false,
                    data: Some(error.clone()),
                    status_code: Some(400),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::BAD_REQUEST,
                };
            }
            RequestError::MissingArgument(error) => {
                let response = APIResponse {
                    success: false,
                    data: Some(error.clone()),
                    status_code: Some(400),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::BAD_REQUEST,
                };
            }
            RequestError::IAmATeapot(error) => {
                let response = APIResponse {
                    success: false,
                    data: Some(error.clone()),
                    status_code: Some(418),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::IM_A_TEAPOT,
                };
            }
            RequestError::NotFound => {
                let response = APIResponse::<bool> {
                    success: false,
                    data: None,
                    status_code: Some(404),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::NOT_FOUND,
                };
            }

            RequestError::InvalidLogin => {
                let response = APIResponse::<bool> {
                    success: false,
                    data: None,
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::Error(error) => {
                let response = APIResponse {
                    success: false,
                    data: Some(error),
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::MismatchingPasswords => {
                let response = APIResponse::<bool> {
                    success: false,
                    data: None,
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::AlreadyExists => {
                let response = APIResponse::<bool> {
                    success: false,
                    data: None,
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::UnInstalled => {
                let response = APIResponse::<bool> {
                    success: false,
                    data: None,
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
            RequestError::InternalError(s) => {
                let response = APIResponse {
                    success: false,
                    data: Some(s.to_string()),
                    status_code: Some(401),
                };
                return ErrorResponse {
                    value: serde_json::to_string(&response).unwrap(),
                    status: StatusCode::UNAUTHORIZED,
                };
            }
        }
    }
}

impl Into<Body> for RequestError {
    fn into(self) -> Body {
        Body::from(self.to_json_response().value)
    }
}

impl actix_web::error::ResponseError for RequestError {
    fn error_response(&self) -> HttpResponse {
        match self {
            RequestError::InternalError(e) => {
                return e.json_error();
            }
            _ => {
                return self.json_error();
            }
        }
    }
}

impl From<diesel::result::Error> for RequestError {
    fn from(err: diesel::result::Error) -> RequestError {
        InternalError::DBError(err).into()
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> RequestError {
        InternalError::JSONError(err).into()
    }
}

impl From<actix_web::Error> for RequestError {
    fn from(err: actix_web::Error) -> RequestError {
        InternalError::ActixWebError(err).into()
    }
}

impl From<r2d2::Error> for RequestError {
    fn from(err: r2d2::Error) -> RequestError {
        InternalError::R2D2Error(err).into()
    }
}

impl From<lettre::transport::smtp::Error> for RequestError {
    fn from(err: lettre::transport::smtp::Error) -> RequestError {
        InternalError::SMTPTransportError(err).into()
    }
}

impl From<ParseBoolError> for RequestError {
    fn from(err: ParseBoolError) -> RequestError {
        InternalError::BooleanParseError(err).into()
    }
}


impl From<actix_web::client::HttpError> for RequestError {
    fn from(err: actix_web::client::HttpError) -> RequestError {
        InternalError::Error(err.to_string()).into()
    }
}

impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> RequestError {
        InternalError::Error(err.to_string()).into()
    }
}

impl From<ToStrError> for RequestError {
    fn from(err: ToStrError) -> RequestError {
        InternalError::Error(err.to_string()).into()
    }
}

impl From<String> for RequestError {
    fn from(value: String) -> RequestError {
        InternalError::Error(value).into()
    }
}

impl From<&str> for RequestError {
    fn from(value: &str) -> Self {
        InternalError::Error(value.to_string()).into()
    }
}
