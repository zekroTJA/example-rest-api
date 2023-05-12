use vercel_runtime::{Body, Error, Response, StatusCode};

pub fn no_content() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::Empty)?)
}
