//! Simple mini library made for translations (JSON)
//!
//! Provides system for language translation using serde_json, include_dir and sys_locale
//! The library was made because i couldn't find any simple language library like that
//!
//! The library simply takes a language directory (that is a include_dir), then searched for a file with the same name as the locale
//! The locale file name should be for example `en-us.json` (it has to be lowercase)
//!
//! Example usage:
//! ```rust
//! use include_dir::{Dir, include_dir};
//! use sys_locale::get_locale;
//! use simple_translate_json::Translation;
//!
//! const LANG_DIR: Dir = include_dir!("lang");
//! let result = Translation::new(get_locale(), LANG_DIR);
//! assert_eq!(result.get("title"), "example");
//! ```
//! lang/en-us.json
//! ```json
//! {
//!   "title": "example"
//! }
//! ```

use std::collections::HashMap;
use include_dir::{Dir, File};
use serde_json::from_str;

/// Struct holding the translation
pub struct Translation {
    translation: HashMap<String, String>,
}

impl Translation {
    /// Create a new translation instance
    pub fn new(language: Option<String>, language_dir: Dir) -> Self {
        if language.is_none() {
            return Translation::default(language_dir)
        }

        let language = language.unwrap();
        let file = language_dir.get_file(format!("{language}.json"));

        if file.is_none() {
            println!("Unsupported language: {language}");
            return Translation::default(language_dir)
        }

        Translation {
            translation: json_file_into_hashmap(file.unwrap()),
        }
    }

    /// Get the translation from key
    pub fn get(&self, key: &str) -> String {
        self.translation.get(key).unwrap_or(&key.to_string()).to_string()
    }

    fn default(language_dir: Dir) -> Self {
        let file = language_dir.get_file("en-us.json").unwrap();
        Translation {
            translation: json_file_into_hashmap(file),
        }
    }
}
fn json_file_into_hashmap(file: &File) -> HashMap<String, String> {
    let contents = file.contents_utf8().unwrap();
    from_str::<HashMap<String, String>>(contents).unwrap()
}
