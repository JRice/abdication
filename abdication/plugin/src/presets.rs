//! Oscillator presets (Nov 2026): each engine (Dual, Wavetable, Ray,
//! Orthogonal, Windows) keeps a collection of its own settings, shared by
//! both slots and every project, as JSON files in
//! `%APPDATA%\Abdication\osc-presets\<Engine>\` (on other systems
//! `~/.config/Abdication/osc-presets/<Engine>/`). A preset holds only the
//! engine's own controls ("wt_1.wave_3" saved as "wave_3"), never the
//! modulation depths on them: loading one leaves those as they are.
//!
//! No UI here, so it can be tested.

use std::path::{Path, PathBuf};

/// Where an engine's presets live.
pub fn folder(engine: &str) -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA").map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("Abdication").join("osc-presets").join(engine))
}

/// An engine's presets, by name (alphabetical), with their files.
pub fn list(engine: &str) -> Vec<(String, PathBuf)> {
    let Some(dir) = folder(engine) else { return Vec::new() };
    list_in(&dir)
}

fn list_in(dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = std::fs::read_dir(dir) else { return Vec::new() };
    let mut presets: Vec<(String, PathBuf)> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .filter_map(|p| Some((p.file_stem()?.to_string_lossy().into_owned(), p)))
        .collect();
    presets.sort_by_key(|(name, _)| name.to_lowercase());
    presets
}

/// A name made safe for a file: what Windows won't take in one becomes "-".
pub fn file_name(name: &str) -> String {
    let cleaned: String = name.trim().chars()
        .map(|c| if "<>:\"/\\|?*".contains(c) || c.is_control() { '-' } else { c })
        .collect();
    let cleaned = cleaned.trim_end_matches(['.', ' ']).to_string();
    if cleaned.is_empty() { "Untitled".into() } else { cleaned }
}

/// Save `values` (each control's id without its slot's prefix, and its plain
/// value) as preset `name` of `engine`, replacing one of that name.
pub fn save(engine: &str, name: &str, values: &[(String, f32)]) -> std::io::Result<PathBuf> {
    let dir = folder(engine).ok_or_else(|| std::io::Error::other("no folder for presets"))?;
    save_in(&dir, engine, name, values)
}

fn save_in(dir: &Path, engine: &str, name: &str, values: &[(String, f32)]) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let map: serde_json::Map<String, serde_json::Value> = values.iter()
        .map(|(id, v)| (id.clone(), serde_json::json!(v)))
        .collect();
    let doc = serde_json::json!({ "engine": engine, "values": map });
    let path = dir.join(format!("{}.json", file_name(name)));
    std::fs::write(&path, serde_json::to_string_pretty(&doc).map_err(std::io::Error::other)?)?;
    Ok(path)
}

/// A preset's values (ids without the slot's prefix).
pub fn load(path: &Path) -> Option<Vec<(String, f32)>> {
    let doc: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
    let values = doc.get("values")?.as_object()?;
    Some(values.iter().filter_map(|(k, v)| Some((k.clone(), v.as_f64()? as f32))).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_round_trip() {
        let dir = std::env::temp_dir().join(format!("abdication-presets-{}", std::process::id()));
        let values = vec![("oscmod_depth".to_string(), 0.25), ("wave_1".to_string(), 4.0)];
        let path = save_in(&dir, "Wavetable", "Glass: Choir?", &values).unwrap();
        assert_eq!(path.file_stem().unwrap(), "Glass- Choir-");
        let mut back = load(&path).unwrap();
        back.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(back, values);
        save_in(&dir, "Wavetable", "another", &values).unwrap();
        let names: Vec<String> = list_in(&dir).into_iter().map(|(n, _)| n).collect();
        assert_eq!(names, ["another", "Glass- Choir-"]);
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(file_name("  "), "Untitled");
    }
}
