use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub prefixes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefixes: vec![
                "feat:".to_string(),
                "fix:".to_string(),
                "docs:".to_string(),
                "style:".to_string(),
                "refactor:".to_string(),
                "test:".to_string(),
                "chore:".to_string(),
            ],
        }
    }
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let package_json_path = Path::new("package.json");
    if package_json_path.exists() {
        let package_json_str = fs::read_to_string(package_json_path)?;
        let package_json: Value = serde_json::from_str(&package_json_str)?;

        if let Some(prefixes) = package_json["commitier-prefixes"].as_array() {
            let prefixes = prefixes.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            return Ok(Config { prefixes });
        }
    }

    Ok(Config::default())
}

pub fn save_prefixes(prefixes: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let package_json_path = Path::new("package.json");
    let package_json_str = if package_json_path.exists() {
        fs::read_to_string(package_json_path)?
    } else {
        "{}".to_string()
    };

    let mut package_json: Value = serde_json::from_str(&package_json_str)?;
    
    package_json["commitier-prefixes"] = serde_json::to_value(prefixes)?;

    let updated_package_json = serde_json::to_string_pretty(&package_json)?;
    let mut file = fs::File::create(package_json_path)?;
    file.write_all(updated_package_json.as_bytes())?;

    Ok(())
}