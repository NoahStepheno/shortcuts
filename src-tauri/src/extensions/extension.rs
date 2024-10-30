use serde_json::json;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub struct Extension {
  pub name: String,
  pub description: String,
  pub shortcuts: Vec<ShortcutHandler>,
}

pub struct ShortcutHandler {
  pub name: String,
  pub description: String,
  pub handle: fn(),
  pub default_shortcut: Option<Shortcut>,
}

impl ShortcutHandler {
  pub fn new(name: String, description: String, handle: fn()) -> Self {
    ShortcutHandler {
      name,
      description,
      handle,
      default_shortcut: None,
    }
  }
  
  pub fn handle(&self) {
    (self.handle)();
  }

  pub fn name(&self) -> String {
    self.name.to_string()
  }

  pub fn description(&self) -> String {
    self.description.to_string()
  }

  pub fn to_string(&self) -> String {
    json!({
      "name": self.name,
      "description": self.description,
    }).to_string()
  }
}