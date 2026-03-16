use std::collections::HashMap;
use std::path::Path;

use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetEntry {
    pub id: String,
    pub name: String,
    pub version: String,
    pub priority: u32,
    pub description: String,
    pub installed_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct PresetRegistry {
    entries: HashMap<String, PresetEntry>,
}

impl PresetRegistry {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        if content.trim().is_empty() || content.trim() == "{}" {
            return Ok(Self::default());
        }
        let entries: HashMap<String, PresetEntry> = match serde_json::from_str(&content) {
            Ok(e) => e,
            Err(e) => {
                log::warn!("Preset registry corrupted, starting fresh: {e}");
                HashMap::new()
            }
        };
        Ok(Self { entries })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add(&mut self, entry: PresetEntry) -> Result<()> {
        if self.entries.contains_key(&entry.id) {
            bail!(
                "Preset '{}' is already installed. Remove it first.",
                entry.id
            );
        }
        self.entries.insert(entry.id.clone(), entry);
        Ok(())
    }

    pub fn remove(&mut self, id: &str) -> Result<PresetEntry> {
        self.entries
            .remove(id)
            .ok_or_else(|| anyhow::anyhow!("Preset '{}' is not installed.", id))
    }

    pub fn get(&self, id: &str) -> Option<PresetEntry> {
        self.entries.get(id).cloned() // deep copy
    }

    pub fn list(&self) -> Vec<PresetEntry> {
        let mut entries: Vec<_> = self.entries.values().cloned().collect();
        entries.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.id.cmp(&b.id)));
        entries
    }

    /// Return (id, priority) pairs sorted by priority for template resolution.
    pub fn sorted_priorities(&self) -> Vec<(String, u32)> {
        let mut pairs: Vec<_> = self
            .entries
            .values()
            .map(|e| (e.id.clone(), e.priority))
            .collect();
        pairs.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        pairs
    }

    pub fn search(&self, query: &str) -> Vec<PresetEntry> {
        let q = query.to_lowercase();
        self.entries
            .values()
            .filter(|e| {
                e.id.to_lowercase().contains(&q)
                    || e.name.to_lowercase().contains(&q)
                    || e.description.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_entry(id: &str, priority: u32) -> PresetEntry {
        PresetEntry {
            id: id.into(),
            name: format!("Preset {id}"),
            version: "1.0.0".into(),
            priority,
            description: format!("Description for {id}"),
            installed_at: "2026-03-14T00:00:00Z".into(),
        }
    }

    #[test]
    fn add_appears_in_registry() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("my-preset", 10)).unwrap();
        assert!(reg.get("my-preset").is_some());
        assert_eq!(reg.get("my-preset").unwrap().priority, 10);
    }

    #[test]
    fn duplicate_id_errors() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("my-preset", 10)).unwrap();
        assert!(reg.add(sample_entry("my-preset", 5)).is_err());
    }

    #[test]
    fn remove_gone_from_registry() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("my-preset", 10)).unwrap();
        reg.remove("my-preset").unwrap();
        assert!(reg.get("my-preset").is_none());
    }

    #[test]
    fn remove_nonexistent_errors() {
        let mut reg = PresetRegistry::default();
        assert!(reg.remove("nonexistent").is_err());
    }

    #[test]
    fn list_sorted_by_priority() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("low-priority", 10)).unwrap();
        reg.add(sample_entry("high-priority", 1)).unwrap();
        reg.add(sample_entry("mid-priority", 5)).unwrap();

        let list = reg.list();
        assert_eq!(list[0].id, "high-priority");
        assert_eq!(list[1].id, "mid-priority");
        assert_eq!(list[2].id, "low-priority");
    }

    #[test]
    fn sorted_priorities_for_resolver() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("b", 10)).unwrap();
        reg.add(sample_entry("a", 1)).unwrap();

        let priorities = reg.sorted_priorities();
        assert_eq!(priorities[0], ("a".into(), 1));
        assert_eq!(priorities[1], ("b".into(), 10));
    }

    #[test]
    fn save_and_reload() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");

        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("test", 5)).unwrap();
        reg.save(&path).unwrap();

        let loaded = PresetRegistry::load(&path).unwrap();
        assert_eq!(loaded.get("test").unwrap().priority, 5);
    }

    #[test]
    fn load_empty_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");
        std::fs::write(&path, "{}").unwrap();
        let reg = PresetRegistry::load(&path).unwrap();
        assert!(reg.entries.is_empty());
    }

    #[test]
    fn load_missing_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".registry");
        let reg = PresetRegistry::load(&path).unwrap();
        assert!(reg.entries.is_empty());
    }

    #[test]
    fn search_by_name() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("testing-preset", 1)).unwrap();
        reg.add(sample_entry("other", 2)).unwrap();

        let results = reg.search("testing");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "testing-preset");
    }

    #[test]
    fn get_returns_deep_copy() {
        let mut reg = PresetRegistry::default();
        reg.add(sample_entry("test", 5)).unwrap();

        let entry = reg.get("test").unwrap();
        // Mutation of copy shouldn't affect registry
        drop(entry);
        assert_eq!(reg.get("test").unwrap().priority, 5);
    }
}
