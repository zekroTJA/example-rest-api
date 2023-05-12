use example_rest_api::{
    controller::{errors::ErrorKind, Controller},
    expect, get_path_param, method_handlers,
};
use vercel_runtime::{
    http::{bad_request, internal_server_error, not_found, ok},
    *,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    method_handlers!(req,
        "GET" => handler_get(req).await,
        "POST" => handler_post(req).await,
    )
}

async fn handler_get(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");

    let res = expect!(controller.list_objects(&collectionid).await,
        Err(err) => match err.kind() {
            ErrorKind::CollectionNotFound => not_found("collection not found"),
            _ => internal_server_error(format!("failed listing objects: {err}"))
        }
    );

    ok(res)
}

async fn handler_post(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");

    let req = expect!(serde_json::from_slice(req.body()),
        Err(err) => bad_request(format!("failed decoding request body: {err}")));

    let res = expect!(controller.create_object(&collectionid, req).await,
        Err(err) => match err.kind() {
            ErrorKind::CollectionNotFound => not_found("collection not found"),
            _ => internal_server_error(format!("failed creating object: {err}"))
        }
    );

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .body(Body::Text(serde_json::to_string(&res).unwrap()))?)
}
