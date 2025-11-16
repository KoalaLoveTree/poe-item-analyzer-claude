//! GitHub API client for checking data updates

use crate::error::ApiError;
use crate::manifest::DataSource;
use serde::{Deserialize, Serialize};

/// GitHub commit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubCommit {
    pub sha: String,
    pub commit: CommitInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub message: String,
    pub author: Author,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: String,
}

/// GitHub file content information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubFile {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub download_url: Option<String>,
}

/// GitHub API client
pub struct GitHubClient {
    client: reqwest::Client,
}

impl GitHubClient {
    /// Create a new GitHub API client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("poe-item-analyzer/0.1.0")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }

    /// Get the latest commit for a specific path
    pub async fn get_latest_commit(
        &self,
        repo: &str,
        path: &str,
    ) -> Result<GitHubCommit, ApiError> {
        let url = format!(
            "https://api.github.com/repos/{}/commits?path={}&per_page=1",
            repo, path
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::RequestFailed(e))?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(format!(
                "GitHub API error: {}",
                response.status()
            )));
        }

        let commits: Vec<GitHubCommit> = response
            .json()
            .await
            .map_err(|e| ApiError::InvalidResponse(e.to_string()))?;

        commits
            .into_iter()
            .next()
            .ok_or_else(|| ApiError::InvalidResponse("No commits found".to_string()))
    }

    /// Get file information from GitHub
    pub async fn get_file_info(
        &self,
        repo: &str,
        path: &str,
        branch: &str,
    ) -> Result<GitHubFile, ApiError> {
        let url = format!(
            "https://api.github.com/repos/{}/contents/{}?ref={}",
            repo, path, branch
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| ApiError::RequestFailed(e))?;

        if !response.status().is_success() {
            return Err(ApiError::ApiError(format!(
                "GitHub API error: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| ApiError::InvalidResponse(e.to_string()))
    }

    /// Check if data source has updates available
    pub async fn check_for_updates(
        &self,
        source: &DataSource,
        current_version: &str,
    ) -> Result<Option<String>, ApiError> {
        let latest_commit = self.get_latest_commit(&source.repo, &source.path).await?;

        // If versions differ, update is available
        if latest_commit.sha != current_version && current_version != "pob-unknown" {
            Ok(Some(latest_commit.sha))
        } else {
            Ok(None)
        }
    }
}

impl Default for GitHubClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_client_creation() {
        let _client = GitHubClient::new();
    }

    // Note: These integration tests require network access
    // They are commented out by default to avoid CI failures

    /*
    #[tokio::test]
    async fn test_get_latest_commit() {
        let client = GitHubClient::new();
        let result = client
            .get_latest_commit(
                "PathOfBuildingCommunity/PathOfBuilding",
                "src/Data/TimelessJewelData",
            )
            .await;

        assert!(result.is_ok());
        let commit = result.unwrap();
        assert!(!commit.sha.is_empty());
    }

    #[tokio::test]
    async fn test_get_file_info() {
        let client = GitHubClient::new();
        let result = client
            .get_file_info(
                "PathOfBuildingCommunity/PathOfBuilding",
                "src/Data/TimelessJewelData/LethalPride.zip",
                "master",
            )
            .await;

        assert!(result.is_ok());
        let file = result.unwrap();
        assert_eq!(file.name, "LethalPride.zip");
    }
    */
}
