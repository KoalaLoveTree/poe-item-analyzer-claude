//! Data manifest models for tracking and updating game data

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Main data manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataManifest {
    /// Version identifier (e.g., git commit hash)
    pub data_version: String,

    /// Current PoE league
    pub poe_league: String,

    /// Last update timestamp
    pub last_updated: String,

    /// Data source information
    pub source: DataSource,

    /// List of data files
    pub files: Vec<DataFile>,
}

impl DataManifest {
    /// Load manifest from JSON file
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })
    }

    /// Save manifest to JSON file
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)
    }

    /// Get all required files
    pub fn required_files(&self) -> Vec<&DataFile> {
        self.files.iter().filter(|f| f.required).collect()
    }

    /// Find a file by name
    pub fn find_file(&self, name: &str) -> Option<&DataFile> {
        self.files.iter().find(|f| f.name == name)
    }
}

/// Data source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Source type (github, url, etc.)
    #[serde(rename = "type")]
    pub source_type: String,

    /// GitHub repository (if type is "github")
    pub repo: String,

    /// Git branch
    pub branch: String,

    /// Path within repository
    pub path: String,

    /// Full URL to repository
    pub url: String,
}

impl DataSource {
    /// Get API URL for checking commits
    pub fn commits_api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/commits?path={}&per_page=1",
            self.repo, self.path
        )
    }

    /// Get API URL for a specific file
    pub fn file_api_url(&self, filename: &str) -> String {
        format!(
            "https://api.github.com/repos/{}/contents/{}/{}?ref={}",
            self.repo, self.path, filename, self.branch
        )
    }
}

/// Individual data file metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    /// File name
    pub name: String,

    /// Download URL
    pub url: String,

    /// SHA256 checksum (for validation)
    pub sha256: String,

    /// GitHub SHA (for tracking updates)
    pub github_sha: String,

    /// File size in bytes
    pub size: u64,

    /// Whether this file is required
    pub required: bool,

    /// Human-readable description
    pub description: String,
}

impl DataFile {
    /// Check if file has valid checksum info
    pub fn has_checksum(&self) -> bool {
        !self.sha256.is_empty()
    }

    /// Check if file has GitHub SHA
    pub fn has_github_sha(&self) -> bool {
        !self.github_sha.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_source_api_urls() {
        let source = DataSource {
            source_type: "github".to_string(),
            repo: "PathOfBuildingCommunity/PathOfBuilding".to_string(),
            branch: "master".to_string(),
            path: "src/Data/TimelessJewelData".to_string(),
            url: "https://github.com/PathOfBuildingCommunity/PathOfBuilding".to_string(),
        };

        let commits_url = source.commits_api_url();
        assert!(commits_url.contains("api.github.com"));
        assert!(commits_url.contains("commits"));
        assert!(commits_url.contains("src/Data/TimelessJewelData"));

        let file_url = source.file_api_url("test.zip");
        assert!(file_url.contains("api.github.com"));
        assert!(file_url.contains("contents"));
        assert!(file_url.contains("test.zip"));
    }

    #[test]
    fn test_manifest_required_files() {
        let manifest = DataManifest {
            data_version: "test".to_string(),
            poe_league: "Test".to_string(),
            last_updated: "2025-01-01T00:00:00Z".to_string(),
            source: DataSource {
                source_type: "github".to_string(),
                repo: "test/test".to_string(),
                branch: "master".to_string(),
                path: "data".to_string(),
                url: "https://github.com/test/test".to_string(),
            },
            files: vec![
                DataFile {
                    name: "required.zip".to_string(),
                    url: "https://example.com/required.zip".to_string(),
                    sha256: String::new(),
                    github_sha: String::new(),
                    size: 0,
                    required: true,
                    description: "Required file".to_string(),
                },
                DataFile {
                    name: "optional.zip".to_string(),
                    url: "https://example.com/optional.zip".to_string(),
                    sha256: String::new(),
                    github_sha: String::new(),
                    size: 0,
                    required: false,
                    description: "Optional file".to_string(),
                },
            ],
        };

        let required = manifest.required_files();
        assert_eq!(required.len(), 1);
        assert_eq!(required[0].name, "required.zip");
    }

    #[test]
    fn test_data_file_checksum_checks() {
        let file_with_checksum = DataFile {
            name: "test.zip".to_string(),
            url: "https://example.com/test.zip".to_string(),
            sha256: "abc123".to_string(),
            github_sha: "def456".to_string(),
            size: 1000,
            required: true,
            description: "Test".to_string(),
        };

        assert!(file_with_checksum.has_checksum());
        assert!(file_with_checksum.has_github_sha());

        let file_without = DataFile {
            name: "test2.zip".to_string(),
            url: "https://example.com/test2.zip".to_string(),
            sha256: String::new(),
            github_sha: String::new(),
            size: 0,
            required: true,
            description: "Test".to_string(),
        };

        assert!(!file_without.has_checksum());
        assert!(!file_without.has_github_sha());
    }
}
