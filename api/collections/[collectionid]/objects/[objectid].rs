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
        "PUT" => handler_put(req).await,
        "DELETE" => handler_delete(req).await,
    )
}

async fn handler_get(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");
    let objectid = get_path_param!(&req, "objectid");

    let res = expect!(controller.get_object(&collectionid, &objectid).await,
        Err(err) => match err.kind() {
            ErrorKind::CollectionNotFound => not_found("collection not found"),
            _ => internal_server_error(format!("failed getting object: {err}"))
        }
    );

    match res {
        None => not_found("object not found"),
        Some(res) => ok(res),
    }
}

async fn handler_put(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");
    let objectid = get_path_param!(&req, "objectid");

    let req = expect!(serde_json::from_slice(req.body()),
        Err(err) => bad_request(format!("failed decoding request body: {err}")));

    let res = expect!(controller.set_object(&collectionid, &objectid, req).await,
        Err(err) => match err.kind() {
            ErrorKind::CollectionNotFound => not_found("collection not found"),
            ErrorKind::ObjectNotFound => not_found("object not found"),
            _ => internal_server_error(format!("failed getting object: {err}"))
        }
    );

    ok(res)
}

async fn handler_delete(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");
    let objectid = get_path_param!(&req, "objectid");

    expect!(controller.delete_object(&collectionid, &objectid).await,
        Err(err) => match err.kind() {
            ErrorKind::CollectionNotFound => not_found("collection not found"),
            _ => internal_server_error(format!("failed getting object: {err}"))
        }
    );

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::Empty)?)
}
