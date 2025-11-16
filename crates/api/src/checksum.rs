//! File checksum utilities for validation

use crate::error::DownloadError;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Calculate SHA256 checksum of a file
pub fn calculate_sha256(path: &Path) -> Result<String, DownloadError> {
    let mut file = File::open(path).map_err(|e| DownloadError::IoError(e))?;

    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 8192]; // 8KB buffer

    loop {
        let bytes_read = file.read(&mut buffer).map_err(|e| DownloadError::IoError(e))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Calculate SHA256 checksum of byte data
pub fn calculate_sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Validate file checksum against expected value
pub fn validate_checksum(path: &Path, expected: &str) -> Result<bool, DownloadError> {
    let actual = calculate_sha256(path)?;
    Ok(actual.eq_ignore_ascii_case(expected))
}

/// Verify file checksum, returning error if mismatch
pub fn verify_checksum(path: &Path, expected: &str) -> Result<(), DownloadError> {
    let actual = calculate_sha256(path)?;

    if !actual.eq_ignore_ascii_case(expected) {
        return Err(DownloadError::ChecksumMismatch {
            expected: expected.to_string(),
            actual,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_calculate_sha256_bytes() {
        let data = b"Hello, World!";
        let checksum = calculate_sha256_bytes(data);

        // Expected SHA256 for "Hello, World!"
        assert_eq!(
            checksum,
            "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
        );
    }

    #[test]
    fn test_calculate_sha256_empty() {
        let data = b"";
        let checksum = calculate_sha256_bytes(data);

        // Expected SHA256 for empty string
        assert_eq!(
            checksum,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_calculate_sha256_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let checksum = calculate_sha256(temp_file.path()).unwrap();

        // Expected SHA256 for "Test data"
        assert_eq!(
            checksum,
            "e27c8214be8b7cf5bccc7c08247e3cb0c1514a48ee1f63197fe4ef3ef51d7e6f"
        );
    }

    #[test]
    fn test_validate_checksum_match() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let expected = "e27c8214be8b7cf5bccc7c08247e3cb0c1514a48ee1f63197fe4ef3ef51d7e6f";
        let result = validate_checksum(temp_file.path(), expected).unwrap();

        assert!(result);
    }

    #[test]
    fn test_validate_checksum_mismatch() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let wrong_checksum = "0000000000000000000000000000000000000000000000000000000000000000";
        let result = validate_checksum(temp_file.path(), wrong_checksum).unwrap();

        assert!(!result);
    }

    #[test]
    fn test_validate_checksum_case_insensitive() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        // Uppercase version
        let expected = "E27C8214BE8B7CF5BCCC7C08247E3CB0C1514A48EE1F63197FE4EF3EF51D7E6F";
        let result = validate_checksum(temp_file.path(), expected).unwrap();

        assert!(result);
    }

    #[test]
    fn test_verify_checksum_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let expected = "e27c8214be8b7cf5bccc7c08247e3cb0c1514a48ee1f63197fe4ef3ef51d7e6f";
        let result = verify_checksum(temp_file.path(), expected);

        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_checksum_failure() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Test data").unwrap();
        temp_file.flush().unwrap();

        let wrong_checksum = "0000000000000000000000000000000000000000000000000000000000000000";
        let result = verify_checksum(temp_file.path(), wrong_checksum);

        assert!(result.is_err());

        if let Err(DownloadError::ChecksumMismatch { expected, actual }) = result {
            assert_eq!(expected, wrong_checksum);
            assert_eq!(
                actual,
                "e27c8214be8b7cf5bccc7c08247e3cb0c1514a48ee1f63197fe4ef3ef51d7e6f"
            );
        } else {
            panic!("Expected ChecksumMismatch error");
        }
    }

    #[test]
    fn test_large_file_checksum() {
        let mut temp_file = NamedTempFile::new().unwrap();

        // Write 1MB of data
        let chunk = vec![0x42; 1024]; // 1KB of 'B'
        for _ in 0..1024 {
            temp_file.write_all(&chunk).unwrap();
        }
        temp_file.flush().unwrap();

        // Should not panic or error on large files
        let checksum = calculate_sha256(temp_file.path());
        assert!(checksum.is_ok());
        assert_eq!(checksum.unwrap().len(), 64); // SHA256 is 64 hex chars
    }
}
