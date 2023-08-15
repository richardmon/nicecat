use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    ANY,
}

//TODO: This structure is meant to represent a configuration that not only matches
// the main domain of a website but also specific paths and possible responses from does paths.
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    // pub sites: Vec<Site>,
    pub sites: Vec<String>
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new(sites: Vec<String>) -> Self {
        Config { sites }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub base_url: String,
    pub catch_all: Option<bool>,
    pub general_response: Option<String>,
    pub specific_responses: Vec<SpecificReponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecificReponse {
    pub path: String,
    pub response: String,
    pub http_method: HttpMethod,
}

#[allow(dead_code)]
fn write_to_yaml_file(config: &Config, filename: &str) -> Result<()> {
    let yaml_str = serde_yaml::to_string(config)?;
    let mut file = File::create(filename)?;
    file.write_all(yaml_str.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
pub fn read_config(file_path: PathBuf) -> Result<Config> {
    let mut file = File::open(file_path).expect("Unable to open configuration file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Unable to read configuration file");

    let config: Config = serde_yaml::from_str(&file_contents).expect("Unable to parse YAML");
    Ok(config)
}
