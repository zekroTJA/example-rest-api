use example_rest_api::{
    controller::{errors::Error as ControllerError, Controller},
    expect, get_path_param, method_handlers,
    utils::response::no_content,
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
        "OPTIONS" => no_content(),
    )
}

async fn handler_get(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");

    let res = expect!(controller.get_collection(&collectionid).await,
        Err(err) => internal_server_error(format!("failed getting collection: {err}")));

    match res {
        None => not_found("collection not found"),
        Some(res) => ok(res),
    }
}

async fn handler_put(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");

    let req = expect!(serde_json::from_slice(req.body()),
        Err(err) => bad_request(format!("failed decoding request body: {err}")));

    let res = expect!(controller.update_collection(&collectionid, req).await,
        Err(ControllerError::CollectionNotFound) => not_found("collection not found"),
        Err(err) => internal_server_error(format!("failed updating collection: {err}")));

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::Text(serde_json::to_string(&res).unwrap()))?)
}

async fn handler_delete(req: Request) -> Result<Response<Body>, Error> {
    let controller = expect!(Controller::from_env(),
        Err(err) => internal_server_error(format!("failed initializing controller: {err}")));

    let collectionid = get_path_param!(&req, "collectionid");

    expect!(controller.delete_collection(&collectionid).await,
        Err(err) => internal_server_error(format!("failed deleting collection: {err}")));

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::Empty)?)
}
