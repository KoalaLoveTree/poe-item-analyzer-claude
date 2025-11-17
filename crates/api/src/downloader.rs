//! Data downloader for LUT files

use std::path::PathBuf;
use reqwest;

use crate::error::DownloadError;

/// Data downloader for managing LUT files
pub struct DataDownloader {
    target_dir: PathBuf,
}

impl DataDownloader {
    /// Create a new data downloader
    pub fn new(target_dir: PathBuf) -> Self {
        Self { target_dir }
    }

    /// Download all required PoB data files
    pub async fn download_pob_data(&self) -> Result<(), DownloadError> {
        // Create target directory if it doesn't exist
        std::fs::create_dir_all(&self.target_dir)
            .map_err(|e| DownloadError::IoError(e))?;

        eprintln!("Downloading PoB data to: {}", self.target_dir.display());

        // Base URL for PoB timeless jewel data
        let base_url = "https://raw.githubusercontent.com/PathOfBuildingCommunity/PathOfBuilding/master/src/Data/TimelessJewelData";

        // List of files to download
        let files = vec![
            "NodeIndexMapping.lua",
            "LegionPassives.lua",
            "LethalPride.zip",
            "BrutalRestraint.zip",
            "ElegantHubris.zip",
            "MilitantFaith.zip",
        ];

        let client = reqwest::Client::new();

        for file_name in files {
            let url = format!("{}/{}", base_url, file_name);
            eprintln!("Downloading: {}", file_name);

            let response = client
                .get(&url)
                .send()
                .await
                .map_err(|e| DownloadError::DownloadFailed(format!("Failed to download {}: {}", file_name, e)))?;

            if !response.status().is_success() {
                return Err(DownloadError::DownloadFailed(format!(
                    "Failed to download {}: HTTP {}",
                    file_name,
                    response.status()
                )));
            }

            let bytes = response
                .bytes()
                .await
                .map_err(|e| DownloadError::DownloadFailed(format!("Failed to read {}: {}", file_name, e)))?;

            let file_path = self.target_dir.join(file_name);
            std::fs::write(&file_path, &bytes)
                .map_err(|e| DownloadError::IoError(e))?;

            eprintln!("  ✓ Saved {} ({} bytes)", file_name, bytes.len());
        }

        eprintln!("Download complete!");
        Ok(())
    }

    /// Get the target directory path
    pub fn target_dir(&self) -> &PathBuf {
        &self.target_dir
    }

    /// Check for updates
    pub async fn check_updates(&self) -> Result<Option<String>, DownloadError> {
        // TODO: Implement update checking
        Ok(None)
    }

    /// Validate downloaded files
    pub async fn validate_files(&self) -> Result<bool, DownloadError> {
        // Check if required files exist
        let required_files = vec![
            "NodeIndexMapping.lua",
            "LegionPassives.lua",
        ];

        for file_name in required_files {
            let file_path = self.target_dir.join(file_name);
            if !file_path.exists() {
                return Ok(false);
            }
        }

        Ok(true)
    }
}
