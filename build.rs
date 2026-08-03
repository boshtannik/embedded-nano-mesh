use std::{env, fs, path::Path};

const CONFIG_KEYS: [&str; 3] = [
    "ENM_RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD",
    "ENM_RECEIVER_FILTER_REGISTRATION_SIZE",
    "ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY",
];

fn main() {
    for key in CONFIG_KEYS {
        println!("cargo:rerun-if-env-changed={key}");
    }
    println!("cargo:rerun-if-changed=.env");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let dotenv_path = Path::new(&manifest_dir).join(".env");

    let dotenv_values = fs::read_to_string(&dotenv_path)
        .ok()
        .map(|content| parse_dotenv(&content));

    for key in CONFIG_KEYS {
        // Real environment takes precedence over the `.env` file.
        if let Ok(value) = env::var(key) {
            println!("cargo:rustc-env={key}={value}");
        } else if let Some(values) = &dotenv_values {
            if let Some(value) = values.get(key) {
                println!("cargo:rustc-env={key}={value}");
            }
        }
    }
}

fn parse_dotenv(content: &str) -> std::collections::HashMap<String, String> {
    let mut values = std::collections::HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').to_string();
            if !key.is_empty() {
                values.insert(key, value);
            }
        }
    }
    values
}
