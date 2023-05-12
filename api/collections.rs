use example_rest_api::{controller::Controller, expect, method_handlers};
use vercel_runtime::{
    http::{bad_request, internal_server_error},
    *,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    method_handlers!(req,
        "POST" => handler_post(req).await,
    )
}

async fn handler_post(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let req = expect!(serde_json::from_slice(req.body()),
        Err(err) => bad_request(format!("failed decoding request body: {err}")));

    let res = expect!(controller.create_collection(req).await,
        Err(err) => internal_server_error(format!("failed creating collection: {err}")));

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::Text(serde_json::to_string(&res).unwrap()))?)
}
