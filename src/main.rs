use clap::{arg, command, value_parser, ArgAction, Command};
use tokio::runtime;

mod config;
mod hosts;
mod pprinter;
mod server;

fn main() {
    let matches = Command::new("NiceCat")
        .version("0.1")
        .author("Richard Montoya")
        .about("Captures requests for other services and displays it nicely in the terminal")
        .arg(arg!(--urls <URLS>).value_delimiter(','))
        .get_matches();

    let urls = matches
        .get_many::<String>("urls")
        .expect("urls set")
        .collect::<Vec<_>>();
    println!("urls: {:?}", urls);
    let urls_owned: Vec<String> = urls.iter().map(|s| (*s).clone()).collect();
    let config = config::Config::new(urls_owned);

    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create Tokio runtime");

    rt.block_on(async {
        server::start_server(&config).await;
    });
}
