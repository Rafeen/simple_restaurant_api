use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::{response, Request};

#[derive(Debug)]
pub struct ApiResponse {
    pub(crate) data: Value,
    pub(crate) status: Status,
}

// Implement `Responder` for `ApiResponse`
impl<'r> Responder<'r, 'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        rocket::Response::build_from(self.data.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

/// A function for 400 response
pub fn to_bad_request_response(message: String) -> ApiResponse {
    ApiResponse {
        status: Status::BadRequest,
        data: json!({ "message": message }),
    }
}

/// A function for 500 response
pub fn to_internal_server_error_response() -> ApiResponse {
    ApiResponse {
        status: Status::InternalServerError,
        data: json!({  "message": "something went wrong" }),
    }
}

/// A function for 404 response
pub fn to_resource_not_found_response(message: &str) -> ApiResponse {
    ApiResponse {
        status: Status::NotFound,
        data: json!({ "message": message }),
    }
}
