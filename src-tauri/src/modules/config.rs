use std::fs::{self, create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use dirs;

#[derive(Debug)]
pub struct Config {
    base_path: PathBuf,
    relative_path: String
}

impl Config {
    pub fn new(relative_path: String) -> Self {
        Self {
            base_path: dirs::home_dir().unwrap(),
            relative_path: relative_path
        }
    }

    pub fn get_base_path(&self) -> String {
        self.base_path.to_str().unwrap().to_string()
    }

    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = PathBuf::from(base_path);
    }

    pub fn read_config(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut path = self.base_path.clone();
        path.push(self.relative_path.clone());
    
        let file = File::open(&path);
    
        let result = match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                Ok(contents)
            },
            Err(_) => {
                if let Some(parent_path) = path.parent() {
                    println!("Creating directory at: {:?}", parent_path);
                    create_dir_all(parent_path)?;
                }
                File::create(&path)?;
                Ok("".to_string())
            }
        };
    
        result
    }
    
    pub fn write_config(&self, config: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = self.base_path.clone();
        path.push(self.relative_path.clone());

        if let Some(parent_path) = path.parent() {
            println!("Creating directory at: {:?}", parent_path);
            create_dir_all(parent_path)?;
        }
    
        let mut file = File::create(&path)?;
        file.write_all(config.as_bytes())?;
    
        Ok(())
    }

    pub fn delete_config(&self) {
        let mut path = self.base_path.clone();
        path.push(self.relative_path.clone());

        let _ = fs::remove_file(path.clone());
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_config_when_no_file() {
    const TEST_PATH: &str = ".shortcuts/__test__/test_read_config_when_no_file.json";
    let config = Config::new(TEST_PATH.to_string());
    config.delete_config();
    let result = config.read_config().unwrap();
    assert!(result == "".to_string());
  }

  #[test]
  fn test_write_config_when_no_file() {
    const TEST_CONFIG: &str = r#"{"key": "value"}"#;
    const TEST_PATH: &str = ".shortcuts/__test__/test_write_config_when_no_file.json";

    let config = Config::new(TEST_PATH.to_string());
    config.delete_config();

    config.write_config(&TEST_CONFIG.to_string()).unwrap();
    let config = config.read_config().unwrap();
    assert_eq!(config, TEST_CONFIG.to_string());
  }

  #[test]
  fn test_write_config_when_exist() {
    const TEST_CONFIG: &str = r#"{"key": "value"}"#;
    const TEST_CONFIG_2: &str = r#"{"key": "value2"}"#;
    const TEST_PATH: &str = ".shortcuts/__test__/test_write_config_when_exist.json";

    let config = Config::new(TEST_PATH.to_string());
    config.delete_config();

    config.write_config(&TEST_CONFIG.to_string()).unwrap();

    config.write_config(&TEST_CONFIG_2.to_string()).unwrap();
    let config = config.read_config().unwrap();
    assert_eq!(config, TEST_CONFIG_2.to_string());
  }
}