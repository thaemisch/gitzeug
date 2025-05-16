use anyhow::{Context, Ok, Result};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

use crate::model::build_tree;
use crate::utils::print_tree;

pub fn fetch_files(url: &str, paths: &[&str], output_dir: &Path) -> Result<()> {
    let dir = TempDir::new().context("Failed to create temp dir")?;
    let dir_path = dir.path();
    let status = Command::new("git")
        .args([
            "clone",
            "--no-checkout",
            "--depth",
            "1",
            "--sparse",
            url,
            dir_path.to_str().context("Invalid temp dir path")?,
        ])
        .status()
        .context("Failed to clone repo")?;
    if !status.success() {
        return Err(anyhow::anyhow!("Git clone failed"));
    }
    Command::new("git")
        .current_dir(dir_path)
        .args(["sparse-checkout", "init", "--no-cone"])
        .status()
        .context("Failed to init sparse-checkout")?;

    let patterns = paths.join("\n");
    std::fs::write(dir_path.join(".git/info/sparse-checkout"), patterns)
        .context("Failed to write sparse-checkout file")?;

    Command::new("git")
        .current_dir(dir_path)
        .args(["checkout", "HEAD"])
        .status()
        .context("Failed to checkout files")?;

    for path in paths {
        let src = dir_path.join(path);
        let dest = output_dir.join(path);
        if src.is_dir() {
            std::fs::create_dir_all(dest.parent().context("Invalid destination path")?)
                .context("Failed to create directory")?;
            crate::utils::copy_dir_all(&src, &dest).context("Failed to copy the directory")?;
        } else if src.exists() {
            std::fs::create_dir_all(dest.parent().context("Invalid destination path")?)
                .context("Failed to create directory")?;
            std::fs::copy(&src, &dest).context("Failed to copy file")?;
        }
    }
    Ok(())
}

pub fn vibe_push(msg: &str) {
    let _ = Command::new("git")
        .args(["add", "."])
        .status()
        .context("Failed to add files to index");

    let _ = Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .context("Failed to commit");

    let _ = Command::new("git")
        .args(["push"])
        .status()
        .context("Failed to push");
}

pub fn squash(n: &u32, msg: &str) {
    let head = format!("HEAD~{}", n);
    let _ = Command::new("git")
        .args(["reset", "--soft", &head])
        .status()
        .context("Failed to execute git reset");

    let _ = Command::new("git")
        .args(["commit", "-m", msg])
        .status()
        .context("Failed to execute git commit --amend");

    let _ = Command::new("git")
        .args(["push", "--force"])
        .status()
        .context("Failed to execute git push --force");
}

pub fn tree_structure(url: &str) -> Result<()> {
    fn list_structure(path: &Path) -> Result<Vec<String>> {
        let output = Command::new("git")
            .current_dir(path)
            .args(["ls-tree", "-r", "--name-only", "HEAD"])
            .output()
            .context("Failed to run git ls-tree")?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("Git ls-tree failed"));
        }
        let paths = String::from_utf8(output.stdout)
            .context("Invalid UTF-8 in git ls-tree output")?
            .lines()
            .map(|s| s.to_string())
            .collect();
        Ok(paths)
    }
    let dir = TempDir::new().context("Failed to create temp dir")?;
    let dir_path = dir.path();
    let status = Command::new("git")
        .args([
            "clone",
            "--no-checkout",
            "--depth",
            "1",
            url,
            dir_path.to_str().context("Invalid temp dir path")?,
        ])
        .status()
        .context("Failed to clone repo")?;
    if !status.success() {
        return Err(anyhow::anyhow!("Git clone failed"));
    }

    let paths = list_structure(&dir_path)?;
    if paths.is_empty() {
        println!(".");
        return Ok(());
    }

    let tree = build_tree(&paths);
    print_tree(&tree, "", true)?;
    Ok(())
}
