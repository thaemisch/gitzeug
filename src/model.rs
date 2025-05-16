use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        TreeNode {
            path,
            is_dir,
            children: Vec::new(),
        }
    }
}

pub fn build_tree(paths: &[String]) -> TreeNode {
    let mut root = TreeNode::new(PathBuf::from(""), true);

    let mut dir_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for path in paths {
        let path_buf = PathBuf::from(path);
        let mut parent = path_buf.parent();
        while let Some(p) = parent {
            if !p.as_os_str().is_empty() {
                dir_set.insert(p.to_string_lossy().into_owned());
            }
            parent = p.parent();
        }
    }

    for path in paths {
        let path_buf = PathBuf::from(path);
        let is_dir = dir_set.contains(path) || path_buf.extension().is_none();
        let components: Vec<_> = path_buf.components().collect();
        let mut current = &mut root;

        for (i, component) in components.iter().enumerate() {
            let component_path = current.path.join(component);
            let maybe_node = current
                .children
                .iter()
                .position(|n| n.path == component_path);

            let is_last_component = i == components.len() - 1;
            let node_is_dir = if is_last_component { is_dir } else { true };

            if let Some(index) = maybe_node {
                current = current.children.get_mut(index).unwrap();
            } else {
                let new_node = TreeNode::new(component_path.clone(), node_is_dir);
                current.children.push(new_node);
                current = current.children.last_mut().unwrap();
            }
        }
    }

    fn sort_nodes(node: &mut TreeNode) {
        node.children.sort_by(|a, b| {
            if a.is_dir == b.is_dir {
                a.path.cmp(&b.path)
            } else if a.is_dir {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        for child in &mut node.children {
            sort_nodes(child);
        }
    }
    sort_nodes(&mut root);

    root
}
