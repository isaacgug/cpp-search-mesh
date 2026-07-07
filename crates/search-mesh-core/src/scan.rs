use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanRequest {
    pub target_dirs: Vec<PathBuf>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanMatch {
    pub file: PathBuf,
    pub line: usize,
    pub keyword: String,
    pub match_str: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScanError {
    #[error("at least one target directory is required")]
    MissingTargetDirs,
    #[error("at least one keyword is required")]
    MissingKeywords,
}

pub fn scan_keywords(request: &ScanRequest) -> Result<Vec<ScanMatch>, ScanError> {
    if request.target_dirs.is_empty() {
        return Err(ScanError::MissingTargetDirs);
    }

    if request.keywords.is_empty() {
        return Err(ScanError::MissingKeywords);
    }

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_missing_target_dirs() {
        let request = ScanRequest {
            target_dirs: Vec::new(),
            keywords: vec!["TODO".to_string()],
        };

        assert_eq!(scan_keywords(&request), Err(ScanError::MissingTargetDirs));
    }

    #[test]
    fn rejects_missing_keywords() {
        let request = ScanRequest {
            target_dirs: vec![PathBuf::from("src")],
            keywords: Vec::new(),
        };

        assert_eq!(scan_keywords(&request), Err(ScanError::MissingKeywords));
    }
}
