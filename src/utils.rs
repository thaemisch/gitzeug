use anyhow::Ok;
use std::path::Path;

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
