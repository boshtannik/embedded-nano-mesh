use std::{collections::HashMap, env, fs, path::Path};

const CONFIG_KEYS: [&str; 6] = [
    "ENM_RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD",
    "ENM_RECEIVER_FILTER_REGISTRATION_SIZE",
    "ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY",
    "ENM_RECEIVER_FILTER_DUPLICATE_STRATEGY",
    "ENM_PACKET_DATA_LENGTH",
    "ENM_ADDRESS_BYTES",
];

fn main() {
    for key in CONFIG_KEYS {
        println!("cargo:rerun-if-env-changed={key}");
    }
    println!("cargo:rerun-if-changed=.env");
    for cfg in ["enm_address_type_u8", "enm_address_type_u16", "enm_address_type_u32"] {
        println!("cargo:rustc-check-cfg=cfg({cfg})");
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let dotenv_path = Path::new(&manifest_dir).join(".env");

    let dotenv_values = fs::read_to_string(&dotenv_path)
        .ok()
        .map(|content| parse_dotenv(&content));

    let mut merged = HashMap::new();
    for key in CONFIG_KEYS {
        if let Ok(value) = env::var(key) {
            merged.insert(key, value);
        } else if let Some(values) = &dotenv_values {
            if let Some(value) = values.get(key) {
                merged.insert(key, value.clone());
            }
        }
    }

    for key in CONFIG_KEYS {
        if let Some(value) = merged.get(key) {
            println!("cargo:rustc-env={key}={value}");
        }
    }

    emit_address_size_cfg(merged.get("ENM_ADDRESS_BYTES").map(String::as_str));
}

/// Emits the rustc cfg flag that selects the wire type of the device
/// address (and, therefore, of the lifetime field too).
///
/// Supported values:
/// - `1` → `u8`  (255 addresses, max 255 hops)
/// - `2` → `u16` (65535 addresses, max 65535 hops)
/// - `4` → `u32` (4 billion addresses, max 4 billion hops)
fn emit_address_size_cfg(value: Option<&str>) {
    let cfg = match value {
        None => "enm_address_type_u8",
        Some("1") => "enm_address_type_u8",
        Some("2") => "enm_address_type_u16",
        Some("4") => "enm_address_type_u32",
        Some(other) => panic!(
            "Invalid ENM_ADDRESS_BYTES value {:?}: expected 1, 2 or 4",
            other
        ),
    };
    println!("cargo:rustc-cfg={cfg}");
}

fn parse_dotenv(content: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
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
