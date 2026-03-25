#![allow(
    clippy::doc_markdown,       // OpenAI 등 고유명사 backtick 불필요
    clippy::cast_precision_loss, // 평가 데이터셋 크기에서 usize→f64 정밀도 손실 무관
    clippy::too_many_lines,      // 평가 함수의 긴 흐름은 자연스러움
)]

//! AI Agent 테스트 평가 모듈
//!
//! 지원 도구:
//! - LLM-as-Judge: GPT-4 기반 자동 평가
//! - RAGAS: RAG 파이프라인 평가 (충실성, 관련성, 문맥 활용도)
//! - Safety: 가드레일 테스트
//! - Langfuse: On-Premise 추적/평가
//! - Promptfoo: 프롬프트 단위 테스트

pub mod config;
pub mod dataset;
pub mod tools;
pub mod utils;

pub use config::{EvalConfig,
                 default_thresholds,
                 default_violation_keywords};
pub use dataset::{legacy_adversarial_prompts,
                  legacy_golden_dataset,
                  load_adversarial_tests,
                  load_golden_dataset,
                  load_test_cases};
pub use tools::{langfuse::run_langfuse_evaluation,
                llm_judge::run_llm_as_judge,
                promptfoo::run_promptfoo_evaluation,
                ragas::run_ragas_evaluation,
                safety::run_safety_evaluation};
pub use utils::save_results;
