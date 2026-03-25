# README

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 프로젝트 개요

Python `chatbot` 프로젝트를 Rust로 포팅한 AI Agent 테스트 방법론 검증용 RAG 챗봇 구현체.
OpenAI API 직접 호출 + 인메모리 벡터스토어 기반 RAG 파이프라인에 다중 평가 도구를 통합.

## 명령어

```bash
# 빌드
cargo build
cargo build --release

# 테스트
cargo test
cargo test -- --nocapture

# 린트
cargo clippy

# 포매팅
cargo fmt

# 실행 (CLI)
cargo run -- --tool llm-judge --save
cargo run -- --tool safety --use-golden-json --save
cargo run -- --tool ragas --save
cargo run -- --tool langfuse --save
cargo run -- --tool all --use-golden-json --save
```

## 아키텍처

```
rust-eval-demo/
├── crates/
│   ├── models/         # 공유 데이터 모델 (EvalSample, EvalResult 등)
│   ├── rag-core/       # RAG 핵심 (OpenAI API, 임베딩, 벡터스토어)
│   ├── eval-runner/    # 평가 도구 (LLM Judge, RAGAS, Safety, Langfuse, Promptfoo)
│   └── client-cli/     # CLI 바이너리 (clap)
├── data/
│   └── golden_dataset.json
└── .env.example
```

**데이터 흐름**: 질문 → 벡터 검색(Top-3) → 문맥 증강 → LLM 응답 → 평가

## 핵심 크레이트

- **models**: `EvalSample`, `EvalResult`, `AdversarialTest`, `SafetyResult`, `RagResponse`, `GoldenDataset` 등
- **rag-core**: `RagChatbot`, `LlmClient`, `EmbeddingClient`, `VectorStore`, `RagConfig`
- **eval-runner**: `run_llm_as_judge()`, `run_ragas_evaluation()`, `run_safety_evaluation()`, `run_langfuse_evaluation()`, `run_promptfoo_evaluation()`
- **client-cli**: clap 기반 CLI 진입점

## 원본 Python 프로젝트와의 차이

| Python (chatbot)              | Rust (rust-eval-demo)                  |
|-------------------------------|----------------------------------------|
| LangChain + ChromaDB          | reqwest + 인메모리 코사인 유사도       |
| Streamlit UI                  | CLI 전용                               |
| RAGAS 라이브러리              | LLM 기반 네이티브 RAGAS 메트릭 계산    |
| DeepEval                      | 미포팅 (Deprecated)                    |
| langfuse Python SDK           | Langfuse REST API 직접 호출            |
| Promptfoo npx                 | subprocess로 npx 호출 (동일)           |

## 백엔드 전환

`USE_AZURE_OPENAI` 환경변수로 제어:
- `true`: Azure OpenAI 사용 (AZURE_* 환경변수 필요)
- `false`: OpenAI API 사용 (OPENAI_API_KEY 필요)
