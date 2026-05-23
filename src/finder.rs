use std::{
    fs::{self},
    path::{Path, PathBuf},
};

pub fn get_markdowns(target_path: &str) -> Vec<PathBuf> {
    let path = Path::new(target_path).to_path_buf();

    if path.is_file() && is_markdown(&path) {
        return vec![path];
    }

    fs::read_dir(target_path)
        .map(|read_dir| {
            read_dir
                .flatten()
                .filter(|entry| is_markdown(&entry.path()))
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}

pub fn is_markdown(path: &PathBuf) -> bool {
    path.extension()
        .map(|ext| ext == "md" || ext == "markdown")
        .unwrap_or(false)
}
