//! Bundled demo loader — reads the upstream `seed/manifest_*.json` session
//! recordings and presents each as a pre-baked User + Assistant demo the UI
//! can open. The manifest's `{{artifact:VID}}` image markers are cleaned into
//! readable placeholders (the figures themselves live inside `assets_*.tar.gz`
//! and aren't unpacked for the MVP viewer).

use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Bundled demo manifests (`seed/`).
pub fn bundled_dir() -> Option<PathBuf> {
    wisp_paths::seed_dir()
}

#[derive(Serialize, Clone)]
pub struct DemoInfo {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Clone)]
pub struct Demo {
    pub id: String,
    pub title: String,
    pub request: String,
    pub response: String,
    pub thinking: Option<String>,
}

fn clean(text: &str) -> String {
    static IMG: OnceLock<Regex> = OnceLock::new();
    static ART: OnceLock<Regex> = OnceLock::new();
    let img = IMG.get_or_init(|| Regex::new(r"!\[([^\]]*)\]\(\{\{artifact:[^}]+\}\}\)").unwrap());
    let art = ART.get_or_init(|| Regex::new(r"\{\{artifact:[^}]+\}\}").unwrap());
    let s = img.replace_all(text, "[$1 (figure)]").to_string();
    art.replace_all(&s, "(artifact)").to_string()
}

fn read_title(path: &std::path::Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;
    let req = v.pointer("/root_frame/input_data/request").and_then(|x| x.as_str())?;
    let first = req.split('.').next().unwrap_or(req).trim();
    Some(first.chars().take(70).collect())
}

/// Enumerate `manifest_*.json` in the bundled seed dir.
pub fn list_demos() -> Vec<DemoInfo> {
    let Some(dir) = bundled_dir() else { return vec![] };
    let mut out = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
            if !stem.starts_with("manifest_") {
                continue;
            }
            let title = read_title(&p).unwrap_or_else(|| stem.trim_start_matches("manifest_").to_string());
            out.push(DemoInfo { id: stem, title });
        }
    }
    out.sort_by(|a, b| a.title.cmp(&b.title));
    out
}

/// Load one demo by id (the manifest file stem, e.g. `manifest_crispr_screen`).
pub fn load_demo(id: &str) -> Option<Demo> {
    let dir = bundled_dir()?;
    let path = dir.join(format!("{id}.json"));
    let text = std::fs::read_to_string(&path).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;
    let req = v.pointer("/root_frame/input_data/request").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let resp = v.pointer("/root_frame/output_data/response").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let thinking = v.pointer("/root_frame/output_data/thinking").and_then(|x| x.as_str()).map(String::from);
    let title = read_title(&path).unwrap_or_else(|| id.trim_start_matches("manifest_").to_string());
    Some(Demo {
        id: id.to_string(),
        title,
        request: clean(&req),
        response: clean(&resp),
        thinking: thinking.map(|t| clean(&t)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_and_loads_bundled_demos() {
        let demos = list_demos();
        assert!(!demos.is_empty(), "bundled seed dir should ship manifest_*.json");
        let crispr = demos.iter().find(|d| d.id.contains("crispr")).expect("crispr demo present");
        let demo = load_demo(&crispr.id).expect("load crispr demo");
        assert!(!demo.request.is_empty());
        assert!(demo.response.contains("CRISPR") || demo.response.contains("kinome"));
        // image markers must be cleaned out
        assert!(!demo.response.contains("{{artifact:"));
    }
}
