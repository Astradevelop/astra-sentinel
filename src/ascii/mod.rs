use std::path::Path;
use std::fs;

pub fn load_ascii_template(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}
