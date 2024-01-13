# simple-translate-json
Simple mini library made for translations (JSON)

Provides system for language translation using serde_json, include_dir and sys_locale
The library was made because I couldn't find any simple language library like that

The library simply takes a language directory (that is a include_dir), then searched for a file with the same name as the locale file name should be for example `en-us.json` (it has to be lowercase)

Example usage:
```rust
use include_dir::{Dir, include_dir};
use sys_locale::get_locale;
use simple_translate_json::Translation;

const LANG_DIR: Dir = include_dir!("lang");
let result = Translation::new(get_locale(), LANG_DIR);
assert_eq!(result.get("title"), "example");
```

lang/en-us.json
```json
{
  "title": "example"
}
```
