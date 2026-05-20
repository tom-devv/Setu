use std::path::PathBuf;

use crate::checker::{LinkCheckResult, LinkStatus, LocalLinkStatus};

fn evaluate_local_path(mdx_path: &PathBuf, local_path: &String) -> LinkCheckResult {
    let stripped = match local_path.strip_prefix("./") {
        Some(s) => s,
        None => {
            return LinkCheckResult {
                source_file: mdx_path.clone(),
                raw_link: local_path.to_owned(),
                status: LinkStatus::Local(LocalLinkStatus::InvalidPrefix),
            };
        }
    };

    let base_dir = mdx_path.parent().unwrap_or(std::path::Path::new("."));
    let joined_path = base_dir.join(stripped);

    let status = if joined_path.exists() {
        LinkStatus::Local(LocalLinkStatus::Valid)
    } else {
        LinkStatus::Local(LocalLinkStatus::DoesNotExist)
    };

    LinkCheckResult {
        source_file: mdx_path.clone(),
        raw_link: local_path.to_owned(),
        status: status,
    }
}

pub fn evaluate(mdx_path: &PathBuf, local_paths: Vec<String>) -> Vec<LinkCheckResult> {
    local_paths
        .iter()
        .map(|local| evaluate_local_path(mdx_path, local))
        .collect()
}
