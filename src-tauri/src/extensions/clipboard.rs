use super::extension::{ShortcutHandler, Extension};
use std::sync::LazyLock;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut};

pub static CLIPBOARD: LazyLock<Extension> = LazyLock::new(|| Extension {
  name: "Clipboard".to_string(),
  description: "Clipboard description".to_string(),
  shortcuts: vec![
    ShortcutHandler{
      name: "Copy".to_string(),
      description: "Copy the selected text to the clipboard".to_string(),
      handle: || -> () {
        println!("Copy the selected text to the clipboard");
      },
      default_shortcut: Some(Shortcut::new(Some(Modifiers::META), Code::KeyC)),
    },
    ShortcutHandler{
      name: "Paste".to_string(),
      description: "Paste the selected text to the clipboard".to_string(),
      handle: || -> () {
        println!("Paste the selected text to the clipboard");
      },
      default_shortcut: Some(Shortcut::new(Some(Modifiers::META), Code::KeyV)),
    },
    ShortcutHandler{
      name: "HistoryViewer".to_string(),
      description: "View paste history board".to_string(),
      handle: || -> () {
        println!("View paste history board");
      },
      default_shortcut: Some(Shortcut::new(Some(Modifiers::META | Modifiers::ALT), Code::KeyV)),
    },
  ],
});

