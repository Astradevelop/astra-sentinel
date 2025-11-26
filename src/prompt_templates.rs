use std::path::Path;
use std::fs;

pub fn load_system_prompt(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

pub fn default_system_prompt() -> &'static str {
    ""
}

pub fn default_user_prompt() -> &'static str {
    ""
}
