use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub fn get_markdowns(target_path: &str) -> Vec<PathBuf> {
    fs::read_dir(target_path)
        .map(|read_dir| {
            read_dir
                .flatten()
                .filter(|entry| is_markdown(entry))
                .map(|entry| entry.path())
                .collect()
        })
        .unwrap_or_else(|_| Vec::new())
}

pub fn is_markdown(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map(|ext| ext == "md" || ext == "markdown")
        .unwrap_or(false)
}
