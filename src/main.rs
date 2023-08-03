mod config;
mod hosts;

#[tokio::main]
async fn main() {
    config::write_yaml();
    // let hostname = "example.com";

    // match add_redirect_rule(hostname) {
    //     Ok(_) => println!("Redirect rule added to /etc/hosts"),
    //     Err(err) => eprintln!("Error: {}", err),
    // }
}
