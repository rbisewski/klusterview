extern crate pretty_env_logger;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use std::process::Command;

//
// print_clusters()
//
async fn print_clusters(_: Request<Body>) -> Result<Response<Body>, Infallible> {

    let clusters_cmd_output = Command::new("sh")
                                       .arg("-c")
                                       .arg("kubectl config get-clusters --kubeconfig=clusters.yaml | sort | grep -v NAME")
                                       .output()
                                       .expect("failed to execute process");

    let clusters = match String::from_utf8(clusters_cmd_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let get_pods_cmd_output = Command::new("sh")
                                       .arg("-c")
                                       .arg("kubectl config current-context && echo && kubectl get pods --all-namespaces | grep -v kube-system")
                                       .output()
                                       .expect("failed to execute process");

    let pods = match String::from_utf8(get_pods_cmd_output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let result = [
        "<html>".to_string(),
        "<head>".to_string(),
        "<title>Infrastructure cluster status page</title>".to_string(),
        "</head>".to_string(),
        "<body>".to_string(),
        "<h2>Clusters</h2>".to_string(),
        "<pre>".to_string(),
        clusters,
        "</pre>".to_string(),
        "<br>".to_string(),
        "<pre>".to_string(),
        pods,
        "</pre>".to_string(),
        "</body>".to_string(),
        "</html>".to_string()
    ].concat();

    Ok(Response::new(Body::from(result)))
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

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
