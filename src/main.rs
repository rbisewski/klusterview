extern crate pretty_env_logger;

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use std::process::Command;

//
// print_clusters()
//
async fn print_clusters(request: Request<Body>) -> Result<Response<Body>, Infallible> {

    println!("Request: {:?}", request);
    println!("URI: {}", request.uri());

    let mut body_content: String = String::from("");
    let start_of_page: String;
    let end_of_page: String;

    //
    // SHOW ALL CLUSTERS
    //
    if request.uri() == "/" {
        let clusters_cmd_output = Command::new("sh")
                                           .arg("-c")
                                           .arg("kubectl config get-clusters --kubeconfig=clusters.yaml | sort | grep -v NAME")
                                           .output()
                                           .expect("failed to execute process");

        let clusters = match String::from_utf8(clusters_cmd_output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // start of page
        start_of_page = [
            "<html>\n
                 <head>\n
                     <title>Infrastructure cluster status page</title>\n
                 </head>\n
                 <body>\n
                     <h2>Clusters</h2>\n
                     <pre>\n".to_string(),
            clusters.clone(),
            "        </pre>\n
                     <br>\n".to_string()
        ].concat();

        // end of page
        end_of_page = [
            "</body>\n
             </html>".to_string()
        ].concat();

        let list_of_clusters: Vec<&str> = clusters.trim().split('\n').collect();

        for cluster in list_of_clusters.iter() {

            body_content.push_str(&["<h3 id='",cluster,"'>", cluster, "</h3>\n"].concat());

            let get_pods_cmd_output = Command::new("sh")
                                               .arg("-c")
                                               .arg(["kubectl --kubeconfig=clusters.yaml config use-context ", cluster, " > /dev/null && kubectl --kubeconfig=clusters.yaml get pods --all-namespaces | grep -v kube-system"].concat())
                                               .output()
                                               .expect("failed to execute process");

            let pods = match String::from_utf8(get_pods_cmd_output.stdout) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            body_content.push_str(&pods)
        }

    //
    // DISPLAY THE PODS OF A SINGLE CLUSTER
    //
    } else {

        let cluster = String::from(request.uri().to_string().trim_matches('/'));

        let get_pods_cmd_output = Command::new("sh")
                                           .arg("-c")
                                           .arg(["kubectl --kubeconfig=clusters.yaml config use-context ", &cluster, " > /dev/null && kubectl --kubeconfig=clusters.yaml get pods --all-namespaces | grep -v kube-system"].concat())
                                           .output()
                                           .expect("failed to execute process");

        let pods = match String::from_utf8(get_pods_cmd_output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        body_content.push_str(&pods);

        // start of page
        start_of_page = [
            "<html>\n
                 <head>\n
                     <title>Cluster pods</title>\n
                 </head>\n
                 <body>\n
                     <br>\n".to_string()
        ].concat();

        // end of page
        end_of_page = [
            "</body>\n
             </html>".to_string()
        ].concat();
    }

    let html = [
        start_of_page,
        "<pre>".to_string(),
        body_content,
        "</pre>".to_string(),
        end_of_page,
    ].concat();

    Ok(Response::new(Body::from(html)))
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
