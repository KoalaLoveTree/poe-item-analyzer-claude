//! Unit tests for API crate modules

use std::path::PathBuf;

use crate::downloader::DataDownloader;

#[test]
fn test_downloader_creation() {
    let downloader = DataDownloader::new(
        "https://example.com/manifest.json".to_string(),
        PathBuf::from("/tmp/data"),
    );

    // Just verify it can be created without panicking
    let _ = downloader;
}

#[test]
fn test_downloader_with_different_urls() {
    let urls = vec![
        "https://github.com/user/repo/manifest.json",
        "https://example.com/data/manifest.json",
        "file:///local/path/manifest.json",
    ];

    for url in urls {
        let downloader = DataDownloader::new(url.to_string(), PathBuf::from("/tmp"));
        let _ = downloader;
    }
}

#[test]
fn test_downloader_with_different_paths() {
    let paths = vec![
        PathBuf::from("/tmp/data"),
        PathBuf::from("/var/lib/app/data"),
        PathBuf::from("./data"),
    ];

    for path in paths {
        let downloader =
            DataDownloader::new("https://example.com/manifest.json".to_string(), path);
        let _ = downloader;
    }
}
