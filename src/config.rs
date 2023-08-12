use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    ANY,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    sites: Vec<Site>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Site {
    base_url: String,
    catch_all: Option<bool>,
    general_response: Option<String>,
    specific_responses: Vec<SpecificReponse>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SpecificReponse {
    path: String,
    response: String,
    http_method: HttpMethod,
}

fn write_to_yaml_file(config: &Config, filename: &str) -> Result<()> {
    let yaml_str = serde_yaml::to_string(config)?;
    let mut file = File::create(filename)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}

pub fn write_yaml() {
    let deep_struct: Config = Config {
        sites: vec![
            Site {
                base_url: "twilio.com".to_string(),
                catch_all: None,
                general_response: None,
                specific_responses: vec![SpecificReponse {
                    path: "/sms/send".to_string(),
                    response: "{'some': 'data'}".to_string(),
                    http_method: HttpMethod::POST,
                }],
            },
            Site {
                base_url: "micro.myapp.com".to_string(),
                catch_all: None,
                general_response: None,
                specific_responses: vec![
                    SpecificReponse {
                        path: "api/users/".to_string(),
                        response: "[{id: 1}, {id: 3}]".to_string(),
                        http_method: HttpMethod::ANY,
                    },
                    SpecificReponse {
                        path: "api/items".to_string(),
                        response: "[{id: 1}, {id: 3}]".to_string(),
                        http_method: HttpMethod::GET,
                    },
                ],
            },
        ],
    };
    let filename = "config.yaml";

    match write_to_yaml_file(&deep_struct, filename) {
        Ok(()) => println!("Successfully wrote to YAML file."),
        Err(err) => eprintln!("Error writing to YAML file: {}", err),
    }
}

pub fn read_config(file_path: PathBuf) -> Result<Config> {
    let mut file = File::open(file_path).expect("Unable to open configuration file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Unable to read configuration file");

    let config: Config = serde_yaml::from_str(&file_contents).expect("Unable to parse YAML");
    Ok(config)
}
