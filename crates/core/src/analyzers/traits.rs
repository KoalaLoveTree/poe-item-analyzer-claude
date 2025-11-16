//! Analyzer traits

use std::cmp::Ordering;

use crate::error::AnalysisError;
use crate::items::AnalyzableItem;

/// Generic analyzer trait for item analysis
pub trait Analyzer<T: AnalyzableItem>: Send + Sync {
    /// Configuration type for this analyzer
    type Config;

    /// Result type produced by analysis
    type Result;

    /// Analyze a single item
    fn analyze(&self, item: &T, config: &Self::Config) -> Result<Self::Result, AnalysisError>;

    /// Analyze a batch of items and return ranked results
    fn analyze_batch(
        &self,
        items: &[T],
        config: &Self::Config,
    ) -> Result<Vec<RankedResult<Self::Result>>, AnalysisError> {
        let mut results = Vec::new();

        for item in items {
            let result = self.analyze(item, config)?;
            results.push(result);
        }

        // Sort by comparison function
        results.sort_by(|a, b| self.compare_results(a, b));

        // Add ranking
        Ok(results
            .into_iter()
            .enumerate()
            .map(|(index, result)| RankedResult {
                rank: index + 1,
                result,
            })
            .collect())
    }

    /// Compare two results for ranking (higher is better)
    /// Return Ordering::Greater if 'a' should rank higher than 'b'
    fn compare_results(&self, a: &Self::Result, b: &Self::Result) -> Ordering;
}

/// A ranked analysis result
#[derive(Debug, Clone)]
pub struct RankedResult<R> {
    /// Rank (1 = best)
    pub rank: usize,

    /// The analysis result
    pub result: R,
}
