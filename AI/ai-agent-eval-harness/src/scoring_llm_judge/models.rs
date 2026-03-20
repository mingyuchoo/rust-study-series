#![allow(dead_code)]
use serde::{Deserialize,
            Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricDimension {
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub scale: u32,
    pub criteria: HashMap<u32, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationRubric {
    pub name: String,
    #[serde(default = "default_domain")]
    pub domain: String,
    pub dimensions: Vec<RubricDimension>,
}

fn default_domain() -> String { "general".into() }

impl EvaluationRubric {
    pub fn validate_weights(&self) -> anyhow::Result<()> {
        let total: f64 = self.dimensions.iter().map(|d| d.weight).sum();
        if !(0.99 ..= 1.01).contains(&total) {
            anyhow::bail!("가중치 합이 1.0이어야 합니다 (현재: {:.4})", total);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionScore {
    pub dimension_name: String,
    pub score: u32,
    pub max_score: u32,
    pub normalized_score: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RubricJudgeResult {
    pub rubric_name: String,
    pub dimension_scores: Vec<DimensionScore>,
    pub weighted_score: f64,
    pub overall_reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairwiseComparison {
    pub trajectory_a_id: String,
    pub trajectory_b_id: String,
    pub winner: String,
    pub confidence: f64,
    pub reasoning: String,
}
