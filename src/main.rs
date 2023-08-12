use std::path::PathBuf;
use tokio::runtime;

mod config;
mod hosts;
mod server;

fn main() {
    config::write_yaml();
    let file_config: config::Config =
        config::read_config(PathBuf::from("config.yaml")).unwrap_or_default();
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create Tokio runtime");

    rt.block_on(async {
        server::start_server(&file_config).await;
    });

    // match add_redirect_rule(hostname) {
    //     Ok(_) => println!("Redirect rule added to /etc/hosts"),
    //     Err(err) => eprintln!("Error: {}", err),
    // }
}
