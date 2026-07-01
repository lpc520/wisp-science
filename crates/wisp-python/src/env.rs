//! uv-managed Python environment provisioning.

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A uv-created virtualenv that hosts the Wisp kernel worker.
pub struct PythonEnv {
    pub venv: PathBuf,
}

impl PythonEnv {
    /// Locate `uv` on PATH (or via `UV_PATH` env).
    pub fn find_uv() -> Option<PathBuf> {
        if let Ok(p) = std::env::var("UV_PATH") {
            return Some(PathBuf::from(p));
        }
        which::which("uv").ok()
    }

    /// Python interpreter inside the venv (`Scripts\python.exe` on Windows).
    pub fn python(&self) -> PathBuf {
        if cfg!(target_os = "windows") {
            self.venv.join("Scripts").join("python.exe")
        } else {
            self.venv.join("bin").join("python")
        }
    }

    /// Ensure a venv exists at `<root>/.wisp/python/.venv`, creating it with
    /// `uv venv` if missing. Returns the env handle.
    pub fn ensure(root: &Path) -> Result<Self> {
        let venv = root.join(".wisp").join("python").join(".venv");
        if venv.join(if cfg!(target_os = "windows") { "Scripts\\python.exe" } else { "bin/python" }).exists() {
            return Ok(Self { venv });
        }
        let uv = Self::find_uv().ok_or_else(|| anyhow!("uv not found on PATH; install uv or set UV_PATH"))?;
        std::fs::create_dir_all(venv.parent().unwrap_or(Path::new(".")))?;
        let out = Command::new(&uv).arg("venv").arg(&venv).output()?;
        if !out.status.success() {
            return Err(anyhow!("uv venv failed: {}", String::from_utf8_lossy(&out.stderr)));
        }
        Ok(Self { venv })
    }
}

/// Path to the kernel worker bundled inside the Wisp source tree
/// (`wisp/python/kernel_worker.py`), resolved from this crate's manifest dir.
pub fn bundled_worker_path() -> Option<PathBuf> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("python").join("kernel_worker.py");
    if p.is_file() { Some(p) } else { None }
}
