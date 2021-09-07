

use std::str::{ParseBoolError};



use actix_web::http::header::ToStrError;
use actix_web::{HttpResponse};
use derive_more::{Display, Error};
use hyper::StatusCode;

use crate::api_response::{APIResponse};
use crate::apierror::APIError;
use crate::error::internal_error::InternalError;
use crate::error::GenericError;
use crate::repository::repo_error::RepositoryError;

#[derive(Debug, Display, Error)]
pub enum RequestError {
    NotAuthorized,
    InvalidLogin,
    NotFound,
    BadRequest(GenericError),
    MismatchingPasswords,
    AlreadyExists,
    MissingArgument(GenericError),
    UnInstalled,
    InternalError(InternalError),

}

impl RequestError {
    pub fn json_error(&self) -> HttpResponse {
        match self {
            RequestError::NotAuthorized => {
                let response = APIResponse {
                    success: false,
                    data: Some(self.to_string()),
                    status_code: Some(401),
                };
                let result = HttpResponse::Ok()
                    .status(StatusCode::UNAUTHORIZED)
                    .content_type("application/json")
                    .body(serde_json::to_string(&response).unwrap());
                return result;
            }
            RequestError::BadRequest(error) => {
                let response = APIResponse {
                    success: false,
                    data: Some(error.error.clone()),
                    status_code: Some(401),
                };
                let result = HttpResponse::Ok()
                    .status(StatusCode::BAD_REQUEST)
                    .content_type("application/json")
                    .body(serde_json::to_string(&response).unwrap());
                return result;
            }
            _ => {
                let response = APIResponse {
                    success: false,
                    data: Some(self.to_string()),
                    status_code: Some(200),
                };
                let result = HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .content_type("application/json")
                    .body(serde_json::to_string(&response).unwrap());
                return result;
            }
        }
    }
}

impl From<InternalError> for RequestError {
    fn from(err: InternalError) -> RequestError {
        RequestError::InternalError(err)
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

//from<Error>
impl From<APIError> for RequestError {
    fn from(_err: APIError) -> RequestError {
        panic!("LEGACY CODE FIX IT FUCKER")
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

impl From<tera::Error> for RequestError {
    fn from(err: tera::Error) -> RequestError {
        InternalError::TeraError(err).into()
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

impl From<hyper::Error> for RequestError {
    fn from(err: hyper::Error) -> RequestError {
        InternalError::HyperError(err).into()
    }
}

impl From<RepositoryError> for RequestError {
    fn from(value: RepositoryError) -> RequestError {
        return InternalError::RepoError(value).into();
    }
}

impl From<actix_web::client::HttpError> for RequestError {
    fn from(err: actix_web::client::HttpError) -> RequestError {
        InternalError::Error(GenericError::from(err.to_string())).into()
    }
}

impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> RequestError {
        InternalError::Error(GenericError::from(err.to_string())).into()
    }
}

impl From<ToStrError> for RequestError {
    fn from(err: ToStrError) -> RequestError {
        InternalError::Error(GenericError::from(err.to_string())).into()
    }
}

impl From<String> for RequestError {
    fn from(value: String) -> RequestError {
        let error = GenericError { error: value };
        InternalError::Error(error).into()
    }
}

impl From<&str> for RequestError {
    fn from(value: &str) -> Self {
        let error = GenericError {
            error: value.to_string(),
        };
        InternalError::Error(error).into()
    }
}