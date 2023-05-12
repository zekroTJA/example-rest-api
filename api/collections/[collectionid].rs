use example_rest_api::{controller::Controller, expect, get_path_param, method_handlers};
use vercel_runtime::{
    http::{internal_server_error, not_found, ok},
    *,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    method_handlers!(req,
        "GET" => handler_get(req).await,
        "DELETE" => handler_delete(req).await,
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
