use anyhow::Result;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn save_to_file(path: &Path, data: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let s = serde_json::to_string_pretty(data)?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
