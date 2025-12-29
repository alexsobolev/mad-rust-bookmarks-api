use crate::error::AppError;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use futures::future::{FutureExt, LocalBoxFuture};
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T> FromRequest for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate + 'static,
{
    type Error = AppError;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_fut = web::Json::<T>::from_request(req, payload);

        async move {
            let json = json_fut
                .await
                .map_err(|e| AppError::ValidationError(e.to_string()))?;

            json.0
                .validate()
                .map_err(|e| AppError::ValidationError(format_validation_errors(e)))?;

            Ok(ValidatedJson(json.0))
        }
        .boxed_local()
    }
}

fn format_validation_errors(errors: validator::ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |e| {
                format!(
                    "{}: {}",
                    field,
                    e.message.as_ref().map(|m| m.as_ref()).unwrap_or("invalid")
                )
            })
        })
        .collect::<Vec<_>>()
        .join(", ")
}
