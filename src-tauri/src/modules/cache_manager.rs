use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

use super::config::*;


#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CacheShortcut {
  pub name: String,
  pub shortcut: Shortcut,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CacheExtension {
  pub name: String,
  pub shortcuts: Vec<CacheShortcut>,
  pub enabled: bool
}


#[derive(Debug)]
pub struct CacheManager {
  extensions: Vec<CacheExtension>,
  config: Config,
}


impl CacheManager {
  pub fn new(path: Option<String>) -> Self {
    let config_path = path.unwrap_or_else(|| ".shortcuts/config.json".to_string());
    CacheManager { extensions: Vec::new(), config: Config::new(config_path) }
  }

  pub fn add(&mut self, extension: CacheExtension) {
    self.extensions.push(extension);
  }

  pub fn read_from_string(&mut self, config: &str) -> &Self {
    let extensions: Vec<CacheExtension> = serde_json::from_str(config).unwrap();
    self.extensions.clear();
    self.extensions.extend(extensions);
    self
  }

  pub fn read_from_cache(&mut self) -> &Self {
    let config =  self.config.read_config().unwrap();
    if config.is_empty() {
      return self;
    }
    self.read_from_string(&config);
    self
  }

  pub fn write_to_cache(&self) -> &Self {
    if self.extensions.len() == 0 {
      return self;
    }
    let config = serde_json::to_string(&self.extensions).unwrap();
    self.config.write_config(&config).unwrap();
    self
  }

  pub fn to_map(&self) -> HashMap<String, CacheExtension> {
    let mut map = HashMap::new();
    for extension in &self.extensions {
      map.insert(extension.name.clone(), extension.clone());
    }
    map
  }

  pub fn init(&mut self) {
    self.read_from_cache();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_shortcut() {
    const TEST_FILE_PATH: &str = ".shortcuts/__test__/test_add_shortcut.json";
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyE);
    cache_manager.add(CacheExtension {
      name: "test".to_string(),
      shortcuts: vec![CacheShortcut {
        name: "test".to_string(),
        shortcut: shortcut.clone()
      }],
      enabled: true
    });
    assert_eq!(cache_manager.extensions.len(), 1);
  }

  #[test]
  fn test_read_from_cache_when_empty() {
    const TEST_FILE_PATH: &str = ".shortcuts/__test__/test_read_from_cache_when_empty.json";
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    cache_manager.config.delete_config();
    cache_manager.read_from_cache();
    assert_eq!(cache_manager.extensions.len(), 0);
  }

  #[test]
  fn test_write_to_cache_when_empty() {
    const TEST_FILE_PATH: &str = "..shortcuts/__test__/test_write_to_cache_when_empty.json";
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    cache_manager.config.delete_config();
    cache_manager.write_to_cache();
    let cache_manager = cache_manager.read_from_cache();
    assert!(cache_manager.extensions.len() == 0);
  }

  #[test]
  fn test_read_from_cache_when_exist() {
    const TEST_FILE_PATH: &str = "..shortcuts/__test__/test_read_from_cache_when_exist.json";
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    let shortcut = Shortcut::new(Some(Modifiers::ALT), Code::KeyE);
    let cache_extension = CacheExtension {
      name: "test".to_string(),
      shortcuts: vec![CacheShortcut {
        name: "test".to_string(),
        shortcut: shortcut.clone()
      }],
      enabled: true
    };
    cache_manager.add(cache_extension);
    cache_manager.write_to_cache();
    
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    cache_manager.read_from_cache();
    assert_eq!(cache_manager.extensions.len(), 1);
  }

  #[test]
  fn test_write_to_cache_when_exist() {
    const TEST_FILE_PATH: &str = "..shortcuts/__test__/test_write_to_cache_when_exist.json";
    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    let shortcut1 = Shortcut::new(Some(Modifiers::ALT), Code::KeyE);
    let shortcut2 = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyR);

    let cache_extension1 = CacheExtension {
      name: "test".to_string(),
      shortcuts: vec![CacheShortcut {
        name: "test".to_string(),
        shortcut: shortcut1.clone()
      }],
      enabled: true
    };

    let cache_extension2 = CacheExtension {
      name: "test".to_string(),
      shortcuts: vec![CacheShortcut {
        name: "test".to_string(),
        shortcut: shortcut2.clone()
      }],
      enabled: true
    };

    cache_manager.add(cache_extension1.clone());
    cache_manager.add(cache_extension2.clone());
    cache_manager.write_to_cache();

    let mut cache_manager = CacheManager::new(Some(TEST_FILE_PATH.to_string()));
    cache_manager.read_from_cache();
    assert_eq!(cache_manager.extensions.len(), 2);
    assert_eq!(cache_manager.extensions[0], cache_extension1);
    assert_eq!(cache_manager.extensions[1], cache_extension2);
  }
}