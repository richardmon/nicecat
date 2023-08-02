mod hosts;
use crate::hosts::add_redirect_rule;


#[tokio::main]
async fn main() {
        let hostname = "example.com";

    match add_redirect_rule(hostname) {
        Ok(_) => println!("Redirect rule added to /etc/hosts"),
        Err(err) => eprintln!("Error: {}", err),
    }
}
