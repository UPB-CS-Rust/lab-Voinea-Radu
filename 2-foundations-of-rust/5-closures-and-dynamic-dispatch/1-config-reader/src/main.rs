use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// An imaginary config file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    port: u16,
    base_url: &'a str,
    s3_path: &'a str,
    database_url: &'a str,
}

#[derive(Debug)]
/// Config deserialization error
pub enum Error {
    /// Something went wrong deserializing JSON
    Json(serde_json::Error),
    /// Something went wrong deserializing YAML
    Yaml(serde_yaml::Error),
}

trait DeserializeConfig {
    /// Deserialize the contents into a `Config`
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error>;
}

struct JsonDeserializer;
struct YamlDeserializer;

impl DeserializeConfig for JsonDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        serde_json::from_str(contents).map_err(|error| Error::Json(error))
    }
}

impl DeserializeConfig for YamlDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        serde_yaml::from_str(contents).map_err(|error| Error::Yaml(error))
    }
}

// TODO add some types that implement `DeserializeConfig`

fn main() {
    let mut args = std::env::args();
    // Unwrapping is OK here, as UTF-8 Strings can always be converted to PathBufs
    let Some(path) = args.nth(1).map(|a| PathBuf::try_from(a).unwrap()) else {
        eprintln!("Please specify the input path");
        return;
    };
    // Unwrapping is Ok as `path` was created from UTF-8 string, and so is the extension
    let extension = path.extension().map(|o| o.to_str().unwrap());
    let file_contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            // `path` was created from an UTF-8 string, so can be converted to one
            eprintln!("Error reading file at path {}: {}", path.to_str().unwrap(), e);
            return;
        }
    };

    let deserializer: Box<dyn DeserializeConfig> = match extension {
        Some("json") => Box::new(JsonDeserializer {}),
        Some("yaml") => Box::new(YamlDeserializer {}),
        Some("yml") => Box::new(YamlDeserializer {}),
        _ => panic!("Unsupported extension")
    };

    let config = deserializer.deserialize(file_contents.as_str());

    println!("Config was: {config:?}");
}
