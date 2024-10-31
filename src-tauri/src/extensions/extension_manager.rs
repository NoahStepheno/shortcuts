use serde_json::json;
use std::sync::LazyLock;
use crate::modules::cache_manager::{self, CacheManager};

use super::{clipboard::CLIPBOARD, extension::{Extension, ShortcutHandler}};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};
use std::collections::HashMap;

pub struct ExtensionManager<'a> {
  extensions: Vec<&'a Extension>,
  cache_manager: CacheManager,

  handler_mapper: HashMap<String, &'a ShortcutHandler>,
  shortcut_mapper: HashMap<Shortcut, &'a ShortcutHandler>,
}

impl<'a> ExtensionManager<'a> {
  pub fn new() -> Self {
    ExtensionManager {
      extensions: vec![],
      cache_manager: CacheManager::new(None),
      handler_mapper: HashMap::new(),
      shortcut_mapper: HashMap::new(),
    }
  }

  pub fn register(&mut self, extension: &'a Extension) {
    self.extensions.push(extension);
  }

  pub fn to_string(&self) -> String {
    let map: HashMap<String, cache_manager::CacheExtension> = self.cache_manager.to_map();
    let extensions: Vec<_> = self.extensions.iter().map(|ext| {
      
      let cache_extension = map.get(&ext.name).cloned();
      match cache_extension {
        Some(cache) => json!({
          "name": ext.name,
          "description": ext.description,
          "enabled": cache.enabled,
          "shortcuts": ext.shortcuts.iter().map(|shortcut| {

            let cache_shortcut = cache.shortcuts.iter().find(|&s| s.name == shortcut.name);
            if let Some(value) = cache_shortcut {
              return json!({
                "name": shortcut.name,
                "description": shortcut.description,
                "shortcut": value.shortcut.to_string(),
              });
            }

            json!({
              "name": shortcut.name,
              "description": shortcut.description,
              "shortcut": shortcut.default_shortcut.unwrap().to_string(),
            })
          }).collect::<Vec<_>>(),
        }),
        None => json!({
          "name": ext.name,
          "description": ext.description,
          "enabled": false,
          "shortcuts": ext.shortcuts.iter().map(|shortcut| {
            json!({
              "name": shortcut.name,
              "description": shortcut.description,
              "shortcut": shortcut.default_shortcut.unwrap().to_string(),
            })
          }).collect::<Vec<_>>(),
        }),
      }
    }).collect();
    serde_json::to_string(&extensions).unwrap()
  }

  pub fn init(&mut self) {
    self.cache_manager.init();
    self.build_handler_mapper();
    self.build_shortcut_mapper();
  }

  pub fn build_handler_mapper(&mut self) {
    self.handler_mapper.clear();
    self.extensions.clone().iter().for_each(|ext| {
      ext.shortcuts.iter().for_each(|shortcut| {
        self.handler_mapper.insert(self.build_key(ext.name.clone(), shortcut.name.clone()), shortcut);
      });
    });
  }

  pub fn build_shortcut_mapper(&mut self) {
    self.shortcut_mapper.clear();
    let extensions = self.cache_manager.extensions.clone();
    extensions.iter().for_each(|ext| {
      if !ext.enabled {
        return;
      }

      ext.shortcuts.iter().for_each(|shortcut| {
        let key = self.build_key(ext.name.clone(), shortcut.name.clone());
        if let Some(shortcut_instance) = self.handler_mapper.get(&key) {
          self.shortcut_mapper.insert(shortcut.shortcut.clone(), shortcut_instance);
        } 
      });
    });
  }

  fn build_key(&self, extension_name: String, shortcut_name: String) -> String {
    format!("{}-{}", extension_name, shortcut_name)
  }

  pub fn listen(&self, shortcut: &Shortcut) {
    if let Some(shortcut_instance) = self.shortcut_mapper.get(shortcut) {
      shortcut_instance.handle();
    }
  }
}

pub static EXTENSION_MANAGER: LazyLock<ExtensionManager> = LazyLock::new(|| {
  let mut manager: ExtensionManager<'_> = ExtensionManager::new();
  manager.register(&*CLIPBOARD);
  manager
});


#[cfg(test)]
mod tests {
  use super::*;
  use crate::extensions::extension::{Extension, ShortcutHandler};

  #[test]
  fn test_extension_manager_new() {
    let manager: ExtensionManager = ExtensionManager::new();
    assert!(manager.extensions.is_empty());
  }

  #[test]
  fn test_register_extension() {
    let mut manager: ExtensionManager = ExtensionManager::new();
    let extension = Extension {
      name: "Test Extension".to_string(),
      description: "A test extension".to_string(),
      shortcuts: vec![],
    };
    manager.register(&extension);
    assert_eq!(manager.extensions.len(), 1);
    assert_eq!(manager.extensions[0].name, "Test Extension");
  }

  #[test]
  fn test_to_string_when_meta() {
    let mut manager: ExtensionManager = ExtensionManager::new();
    let extension = Extension {
      name: "Test Extension".to_string(),
      description: "A test extension".to_string(),
      shortcuts: vec![
        ShortcutHandler{
          name: "Copy".to_string(),
          description: "Copy the selected text to the clipboard".to_string(),
          handle: || -> () {
            println!("Copy the selected text to the clipboard");
          },
          default_shortcut: Some(Shortcut::new(Some(Modifiers::META), Code::KeyV))
        },
      ],
    };
    manager.register(&extension);
    let json_str = manager.to_string();
    let expected_json = serde_json::json!([{
      "name": "Test Extension",
      "description": "A test extension",
      "enabled": false,
      "shortcuts": [{
        "name": "Copy",
        "description": "Copy the selected text to the clipboard",
        "shortcut": "super+KeyV"
      }]
    }]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&json_str).unwrap(), expected_json);
  }

  #[test]
  fn test_to_string_when_alt() {
    let mut manager: ExtensionManager = ExtensionManager::new();
    let extension = Extension {
      name: "Test Extension".to_string(),
      description: "A test extension".to_string(),
      shortcuts: vec![
        ShortcutHandler{
          name: "Copy".to_string(),
          description: "Copy the selected text to the clipboard".to_string(),
          handle: || -> () {
            println!("Copy the selected text to the clipboard");
          },
          default_shortcut: Some(Shortcut::new(Some(Modifiers::ALT), Code::KeyV))
        },
      ],
    };
    manager.register(&extension);
    let json_str = manager.to_string();
    let expected_json = serde_json::json!([{
      "name": "Test Extension",
      "description": "A test extension",
      "enabled": false,
      "shortcuts": [{
        "name": "Copy",
        "description": "Copy the selected text to the clipboard",
        "shortcut": "alt+KeyV"
      }]
    }]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&json_str).unwrap(), expected_json);
  }

  #[test]
  fn test_to_string_when_ctrl() {
    let mut manager: ExtensionManager = ExtensionManager::new();
    let extension = Extension {
      name: "Test Extension".to_string(),
      description: "A test extension".to_string(),
      shortcuts: vec![
        ShortcutHandler{
          name: "Copy".to_string(),
          description: "Copy the selected text to the clipboard".to_string(),
          handle: || -> () {
            println!("Copy the selected text to the clipboard");
          },
          default_shortcut: Some(Shortcut::new(Some(Modifiers::CONTROL), Code::KeyV))
        },
      ],
    };
    manager.register(&extension);
    let json_str = manager.to_string();
    let expected_json = serde_json::json!([{
      "name": "Test Extension",
      "description": "A test extension",
      "enabled": false,
      "shortcuts": [{
        "name": "Copy",
        "description": "Copy the selected text to the clipboard",
        "shortcut": "control+KeyV"
      }]
    }]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&json_str).unwrap(), expected_json);
  }

  #[test]
  fn test_to_string_when_shift() {
    let mut manager: ExtensionManager = ExtensionManager::new();
    let extension = Extension {
      name: "Test Extension".to_string(),
      description: "A test extension".to_string(),
      shortcuts: vec![
        ShortcutHandler{
          name: "Copy".to_string(),
          description: "Copy the selected text to the clipboard".to_string(),
          handle: || -> () {
            println!("Copy the selected text to the clipboard");
          },
          default_shortcut: Some(Shortcut::new(Some(Modifiers::SHIFT), Code::KeyV))
        },
      ],
    };
    manager.register(&extension);
    let json_str = manager.to_string();
    let expected_json = serde_json::json!([{
      "name": "Test Extension",
      "description": "A test extension",
      "enabled": false,
      "shortcuts": [{
        "name": "Copy",
        "description": "Copy the selected text to the clipboard",
        "shortcut": "shift+KeyV"
      }]
    }]);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&json_str).unwrap(), expected_json);
  }
}

