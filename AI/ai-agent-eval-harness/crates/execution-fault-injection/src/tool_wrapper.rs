#![allow(dead_code)]
use crate::{failure_modes::FailureModeGenerator,
            models::{FailureMode,
                     InjectedFailure}};
use execution_tools::base::{BaseTool,
                            ToolMetadata};
use rand::Rng;
use std::{collections::HashMap,
          sync::Arc};

pub struct FaultInjectedTool {
    original: Arc<dyn BaseTool>,
    failure_rate: f64,
    failure_mode_dist: Vec<(FailureMode, f64)>,
    pub injected_failures: std::sync::Mutex<Vec<InjectedFailure>>,
    seed: Option<u64>,
}

impl FaultInjectedTool {
    pub fn new(original: Arc<dyn BaseTool>, failure_rate: f64, failure_mode_distribution: &HashMap<String, f64>, seed: Option<u64>) -> Self {
        let dist = vec![
            (FailureMode::Timeout, *failure_mode_distribution.get("timeout").unwrap_or(&0.2)),
            (FailureMode::PartialResult, *failure_mode_distribution.get("partial_result").unwrap_or(&0.25)),
            (FailureMode::IncorrectResult, *failure_mode_distribution.get("incorrect_result").unwrap_or(&0.2)),
            (FailureMode::Exception, *failure_mode_distribution.get("exception").unwrap_or(&0.2)),
            (FailureMode::NetworkError, *failure_mode_distribution.get("network_error").unwrap_or(&0.1)),
            (
                FailureMode::PermissionDenied,
                *failure_mode_distribution.get("permission_denied").unwrap_or(&0.05),
            ),
        ];
        Self {
            original,
            failure_rate,
            failure_mode_dist: dist,
            injected_failures: std::sync::Mutex::new(Vec::new()),
            seed,
        }
    }

    fn sample_failure_mode(&self, rng: &mut impl Rng) -> FailureMode {
        let total: f64 = self.failure_mode_dist.iter().map(|(_, w)| w).sum();
        let mut r = rng.gen::<f64>() * total;
        for (mode, weight) in &self.failure_mode_dist {
            r -= weight;
            if r <= 0.0 {
                return mode.clone();
            }
        }
        FailureMode::Exception
    }
}

impl BaseTool for FaultInjectedTool {
    fn metadata(&self) -> &ToolMetadata { self.original.metadata() }

    fn execute(&self, params: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
        let mut rng = if let Some(seed) = self.seed {
            rand::rngs::StdRng::seed_from_u64(seed)
        } else {
            rand::rngs::StdRng::from_entropy()
        };

        if rng.gen::<f64>() < self.failure_rate {
            let mode = self.sample_failure_mode(&mut rng);
            let result = FailureModeGenerator::generate(&mode, self.original.metadata(), params, &mut rng);
            self.injected_failures.lock().unwrap().push(InjectedFailure {
                tool_name: self.original.metadata().name.clone(),
                failure_mode: mode,
                original_parameters: params.clone(),
                injected_result: result.clone(),
                timestamp: chrono::Utc::now(),
            });
            return result;
        }
        self.original.execute(params)
    }

    fn validate_parameters(&self, params: &HashMap<String, serde_json::Value>) -> bool { self.original.validate_parameters(params) }
}

use rand::SeedableRng;
