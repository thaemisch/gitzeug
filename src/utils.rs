use anyhow::Ok;
use std::path::Path;
use reqwest::Client;

use crate::model::TreeNode;

pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), anyhow::Error> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) -> Result<(), anyhow::Error> {
    if !node.path.as_os_str().is_empty() {
        let marker = if is_last { "└──" } else { "├──" };
        let name = node
            .path
            .file_name()
            .unwrap_or(node.path.as_os_str())
            .to_string_lossy();
        println!("{}{} {}", prefix, marker, name);
    }

    let new_prefix = if node.path.as_os_str().is_empty() {
        "".to_string()
    } else if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    for (i, child) in node.children.iter().enumerate() {
        let is_last_child = i == node.children.len() - 1;
        print_tree(child, &new_prefix, is_last_child)?;
    }
    Ok(())
}

pub fn initialize_config() -> Result<(), anyhow::Error> {
    let config_path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("gitzeug");

    if !config_path.exists() {
        std::fs::create_dir_all(&config_path)?;
    }

    let config_file = config_path.join("config.toml");
    if !config_file.exists() {
        std::fs::write(&config_file, "[ai_cmsg]\nkey = \"\"\nmodel = \"\"\nmsg = \"Generate a short, concise commit message for the following diff. Your answer should contain the commit message only, no further output.\"")?;
        println!("Config file created at: {}", config_file.display());
    } else {
        println!("Config file already exists at: {}", config_file.display());
    }

    Ok(())
}

pub fn get_config() -> Result<toml::Value, anyhow::Error> {
    let config_path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("gitzeug")
        .join("config.toml");

    if !config_path.exists() {
        return Err(anyhow::anyhow!("Config file does not exist at: {}", config_path.display()));
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: toml::Value = toml::from_str(&content)?;
    Ok(config)
}

pub async fn openrouter(key: &str, model: &str, msg: &str) -> Result<String, anyhow::Error> {
    let client = Client::new();
    let url = "https://openrouter.ai/api/v1/chat/completions";


    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", key))
        .json(&serde_json::json!({
            "model": model,
            "messages": [
                {"role": "user", "content": msg}
            ]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to get response from OpenRouter: {}", response.status()));
    }

    let json: serde_json::Value = response.json().await?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
        .to_string();

    Ok(content)
}

pub async fn openrouter_check_limits(key: &str) -> Result<String, anyhow::Error> {
    let client = Client::new();
    let url = "https://openrouter.ai/api/v1/auth/key";

    let response = client.get(url)
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to check usage limits: {}", response.status()));
    }

    let output = response.text().await?;

    Ok(output)
}