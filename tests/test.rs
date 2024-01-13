use include_dir::include_dir;
use sys_locale::get_locale;
use simple_translate_json::Translation;

#[test]
fn it_works() {
    let result = Translation::new(get_locale(), include_dir!("lang"));
    assert_eq!(result.get("hello"), "Hello");
}