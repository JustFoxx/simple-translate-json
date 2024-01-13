use std::collections::HashMap;
use include_dir::Dir;

pub struct Translation {
    translation: HashMap<String, String>,
}

impl Translation {
    pub fn new(language: Option<String>, dir: Dir) -> Self {
        let language = language.unwrap_or_else(|| "en-US".to_string());
        let file = dir.get_file(format!("{}.json", language)).unwrap_or_else(|| {
            println!("Unsupported language: {language}");
            dir.get_file("en-US.json").unwrap()
        });
        let contents = file.contents_utf8().unwrap();
        let json = serde_json::from_str::<HashMap<String, String>>(contents).unwrap();
        Translation {
            translation: json,
        }
    }

    pub fn get(&self, key: &str) -> String {
        self.translation.get(key).unwrap_or(&key.to_string()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use include_dir::include_dir;
    use sys_locale::get_locale;
    use super::*;

    #[test]
    fn it_works() {
        let result = Translation::new(get_locale(), include_dir!("lang"));
        assert_eq!(result.get("hello"), "Hello");
    }
}
