extern crate pretty_env_logger;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use std::process::Command;

//
// print_clusters()
//
async fn print_clusters(_: Request<Body>) -> Result<Response<Body>, Infallible> {

    let output = Command::new("sh")
                         .arg("-c")
                         .arg("kubectl config current-context && echo && kubectl get pods --all-namespaces | grep -v kube-system")
                         .output()
                         .expect("failed to execute process");

    Ok(Response::new(Body::from(output.stdout)))
}

//
// MAIN
//
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(print_clusters)) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
