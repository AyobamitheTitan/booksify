pub mod error {
    use actix_web::{error::JsonPayloadError, HttpRequest, HttpResponse, Error};
    use serde_json::json;

    pub fn custom_json_error(err: JsonPayloadError, _: &HttpRequest) -> Error {

        let error_response = match &err {
            JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(json!({
                "error":"Invalid Content-Type. Expecting application/json"
            })),
            // JsonPayloadError::Deserialize(err) => HttpResponse::BadRequest().json(json!({
            //     "error":err.to_string()
            // })),
            _ => HttpResponse::BadRequest().json(json!({
                "error":err.to_string()
            })),
        }; 

        actix_web::error::InternalError::from_response(err, error_response).into()
    }
}
