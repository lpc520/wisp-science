//! Resolve bundled asset directories in dev (repo root) and release (Tauri resources).

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static RESOURCE_ROOT: OnceLock<PathBuf> = OnceLock::new();

/// Set the install resource root (Tauri `resource_dir` in release builds).
pub fn set_resource_root(root: PathBuf) {
    let _ = RESOURCE_ROOT.set(root);
}

fn dev_repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

/// Root directory containing bundled `skills/`, `python/`, etc.
pub fn resource_root() -> PathBuf {
    RESOURCE_ROOT.get().cloned().unwrap_or_else(dev_repo_root)
}

fn existing_dir(base: &Path, rel: &str) -> Option<PathBuf> {
    let p = base.join(rel);
    p.is_dir().then_some(p)
}

pub fn skills_dir() -> Option<PathBuf> {
    existing_dir(&resource_root(), "skills")
}

pub fn python_dir() -> Option<PathBuf> {
    existing_dir(&resource_root(), "python")
}

pub fn bio_tools_dir() -> Option<PathBuf> {
    existing_dir(&resource_root(), "mcp-servers/bio-tools")
}

pub fn seed_dir() -> Option<PathBuf> {
    existing_dir(&resource_root(), "seed")
}

pub fn kernel_worker_path() -> Option<PathBuf> {
    python_dir().map(|d| d.join("kernel_worker.py")).filter(|p| p.is_file())
}

pub fn mcp_requirements_path() -> Option<PathBuf> {
    python_dir()
        .map(|d| d.join("requirements-mcp.txt"))
        .filter(|p| p.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev_tree_has_bundled_assets() {
        assert!(skills_dir().is_some());
        assert!(python_dir().is_some());
        assert!(bio_tools_dir().is_some());
        assert!(seed_dir().is_some());
    }
}
