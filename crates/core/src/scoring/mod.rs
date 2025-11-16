//! Scoring systems for item analysis

pub mod weighted;

#[cfg(test)]
mod tests;

pub use weighted::WeightedScorer;
