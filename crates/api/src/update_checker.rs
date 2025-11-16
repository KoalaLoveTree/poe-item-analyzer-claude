//! Update checker service for data management

use crate::error::DownloadError;
use crate::github::GitHubClient;
use crate::manifest::DataManifest;
use std::path::{Path, PathBuf};

/// Update information
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    /// Whether an update is available
    pub available: bool,

    /// Current version (commit SHA)
    pub current_version: String,

    /// Latest version (commit SHA)
    pub latest_version: Option<String>,

    /// Commit message of the latest version
    pub commit_message: Option<String>,

    /// Date of the latest commit
    pub commit_date: Option<String>,
}

/// Update checker service
pub struct UpdateChecker {
    github_client: GitHubClient,
    manifest_path: PathBuf,
}

impl UpdateChecker {
    /// Create a new update checker
    pub fn new(manifest_path: PathBuf) -> Self {
        Self {
            github_client: GitHubClient::new(),
            manifest_path,
        }
    }

    /// Check if updates are available
    pub async fn check_for_updates(&self) -> Result<UpdateInfo, DownloadError> {
        // Load local manifest
        let manifest = DataManifest::load_from_file(&self.manifest_path)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))?;

        let current_version = manifest.data_version.clone();

        // Check GitHub for latest commit
        let latest_commit = self
            .github_client
            .get_latest_commit(&manifest.source.repo, &manifest.source.path)
            .await
            .map_err(|e| DownloadError::DownloadFailed(e.to_string()))?;

        let latest_version = latest_commit.sha.clone();
        let available = current_version != latest_version && current_version != "pob-unknown";

        Ok(UpdateInfo {
            available,
            current_version,
            latest_version: if available {
                Some(latest_version)
            } else {
                None
            },
            commit_message: Some(latest_commit.commit.message),
            commit_date: Some(latest_commit.commit.author.date),
        })
    }

    /// Get current data version
    pub fn get_current_version(&self) -> Result<String, DownloadError> {
        let manifest = DataManifest::load_from_file(&self.manifest_path)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))?;
        Ok(manifest.data_version)
    }

    /// Check if data files exist locally
    pub fn data_exists(&self, data_dir: &Path) -> Result<bool, DownloadError> {
        let manifest = DataManifest::load_from_file(&self.manifest_path)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))?;

        // Check if all required files exist
        for file in manifest.required_files() {
            let file_path = data_dir.join(&file.name);
            if !file_path.exists() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get list of missing files
    pub fn get_missing_files(&self, data_dir: &Path) -> Result<Vec<String>, DownloadError> {
        let manifest = DataManifest::load_from_file(&self.manifest_path)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))?;

        let mut missing = Vec::new();

        for file in manifest.required_files() {
            let file_path = data_dir.join(&file.name);
            if !file_path.exists() {
                missing.push(file.name.clone());
            }
        }

        Ok(missing)
    }

    /// Update manifest with new version
    pub fn update_manifest_version(&self, new_version: String) -> Result<(), DownloadError> {
        let mut manifest = DataManifest::load_from_file(&self.manifest_path)
            .map_err(|e| DownloadError::InvalidManifest(e.to_string()))?;

        manifest.data_version = new_version;
        manifest.last_updated = chrono::Utc::now().to_rfc3339();

        manifest
            .save_to_file(&self.manifest_path)
            .map_err(|e| DownloadError::DownloadFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{DataFile, DataSource};
    use std::fs;
    use tempfile::TempDir;

    fn create_test_manifest(temp_dir: &TempDir) -> PathBuf {
        let manifest_path = temp_dir.path().join("manifest.json");

        let manifest = DataManifest {
            data_version: "test-version".to_string(),
            poe_league: "Test".to_string(),
            last_updated: "2025-01-01T00:00:00Z".to_string(),
            source: DataSource {
                source_type: "github".to_string(),
                repo: "PathOfBuildingCommunity/PathOfBuilding".to_string(),
                branch: "master".to_string(),
                path: "src/Data/TimelessJewelData".to_string(),
                url: "https://github.com/PathOfBuildingCommunity/PathOfBuilding".to_string(),
            },
            files: vec![
                DataFile {
                    name: "test1.zip".to_string(),
                    url: "https://example.com/test1.zip".to_string(),
                    sha256: String::new(),
                    github_sha: String::new(),
                    size: 0,
                    required: true,
                    description: "Test file 1".to_string(),
                },
                DataFile {
                    name: "test2.zip".to_string(),
                    url: "https://example.com/test2.zip".to_string(),
                    sha256: String::new(),
                    github_sha: String::new(),
                    size: 0,
                    required: true,
                    description: "Test file 2".to_string(),
                },
            ],
        };

        manifest.save_to_file(&manifest_path).unwrap();
        manifest_path
    }

    #[test]
    fn test_update_checker_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let _checker = UpdateChecker::new(manifest_path);
    }

    #[test]
    fn test_get_current_version() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let checker = UpdateChecker::new(manifest_path);
        let version = checker.get_current_version().unwrap();

        assert_eq!(version, "test-version");
    }

    #[test]
    fn test_data_exists_none() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let checker = UpdateChecker::new(manifest_path);
        let data_dir = temp_dir.path().join("data");

        let exists = checker.data_exists(&data_dir).unwrap();
        assert!(!exists);
    }

    #[test]
    fn test_data_exists_partial() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create only one file
        fs::write(data_dir.join("test1.zip"), b"test data").unwrap();

        let checker = UpdateChecker::new(manifest_path);
        let exists = checker.data_exists(&data_dir).unwrap();

        assert!(!exists); // Not all files exist
    }

    #[test]
    fn test_data_exists_all() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create all files
        fs::write(data_dir.join("test1.zip"), b"test data 1").unwrap();
        fs::write(data_dir.join("test2.zip"), b"test data 2").unwrap();

        let checker = UpdateChecker::new(manifest_path);
        let exists = checker.data_exists(&data_dir).unwrap();

        assert!(exists);
    }

    #[test]
    fn test_get_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(&data_dir).unwrap();

        // Create only one file
        fs::write(data_dir.join("test1.zip"), b"test data").unwrap();

        let checker = UpdateChecker::new(manifest_path);
        let missing = checker.get_missing_files(&data_dir).unwrap();

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0], "test2.zip");
    }

    #[test]
    fn test_update_manifest_version() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = create_test_manifest(&temp_dir);

        let checker = UpdateChecker::new(manifest_path.clone());
        checker
            .update_manifest_version("new-version".to_string())
            .unwrap();

        // Verify version was updated
        let updated = checker.get_current_version().unwrap();
        assert_eq!(updated, "new-version");
    }
}
