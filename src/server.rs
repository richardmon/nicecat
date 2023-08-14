use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::any,
    Router,
};
use colored::*;

use crate::{config::Config, pprinter::PPrinter};

pub async fn start_server(_config: &Config) {
    let webserver_port = match env::var("PORT") {
        Ok(port) => port.parse().expect("Unable to parse the PORT varible"),
        Err(_) => 8080,
    };
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), webserver_port);
    let app = Router::new().route("/*path", any(default_handler));
    println!(
        "{} {}!",
        "🚀 - Web server runing in port:".green(),
        webserver_port.to_string().red().bold()
    );
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[axum::debug_handler]
async fn default_handler(
    headers: HeaderMap,
    Path(path): Path<String>,
    body: String,
) -> impl IntoResponse {
    PPrinter::pretty_print(path, &headers, body);
    StatusCode::OK
}
