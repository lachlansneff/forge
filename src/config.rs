use std::fs::File;
use std::io::{Read, Write};
use toml::{from_str, to_string};

#[derive(Deserialize, Serialize)]
pub struct Config {
    target: Option<String>,
}

impl Config {
    pub fn new(target: Option<&str>) -> Config {
        Config {
            target: match target {
                Some(s) => Some(String::from(s)),
                None => None,
            },
        }
    }

    pub fn open(path: &str) -> Result<Config, ()> {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        match from_str(&contents) {
            Ok(c) => Ok(c),
            Err(_) => Err(())
        }
    }

    pub fn write(&self, path: &str) {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => File::create(path).unwrap(),
        };

        let serialized = to_string(self).unwrap();

        file.write_all(serialized.as_bytes()).unwrap();
    }

    pub fn target(&self) -> Option<String> {
        self.target.clone()
    }
}