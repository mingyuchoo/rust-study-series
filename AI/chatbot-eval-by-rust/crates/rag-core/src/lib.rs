//! RAG 챗봇 핵심 모듈
//!
//! - OpenAI / Azure OpenAI API를 사용한 LLM 및 임베딩
//! - 인메모리 코사인 유사도 벡터스토어
//! - 검색 증강 생성(RAG) 파이프라인

#![allow(clippy::doc_markdown)] // OpenAI 등 고유명사 backtick 불필요

mod config;
mod embedding;
mod error;
mod llm;
mod vectorstore;

pub use config::RagConfig;
pub use embedding::EmbeddingClient;
pub use error::RagError;
pub use llm::LlmClient;
use models::{DocumentMeta,
             RagResponse};
pub use vectorstore::VectorStore;

/// RAG 시스템 프롬프트 (가드레일 포함)
const SYSTEM_PROMPT: &str = "당신은 제공된 문서를 기반으로 질문에 답변하는 도움이 되는 AI 어시스턴트입니다.

## 규칙
1. 반드시 제공된 문맥(context)에 기반하여 답변하세요.
2. 문맥에 없는 정보는 \"제공된 문서에서 해당 정보를 찾을 수 없습니다\"라고 답변하세요.
3. 추측하거나 지어내지 마세요 (환각 방지).
4. 개인정보, 비밀번호, API 키 등 민감한 정보는 절대 노출하지 마세요.
5. 불법적이거나 비윤리적인 요청에는 응하지 마세요.";

/// 샘플 문서 (드래곤볼 관련)
pub const SAMPLE_DOCUMENTS: &[&str] = &[
    "토리야마 아키라의 걸작 드래곤볼은 손오공이라는 사이어인 소년을 중심으로 펼쳐지는 거대한 모험과 전투 서사시로, \
     단순한 액션 만화가 아닌 가족, 우정, 라이벌 관계를 통해 인간성을 탐구하는 깊이 있는 이야기다. \
     작품의 심장은 주요 등장인물들인 Z전사들로, 그들은 혈연과 전투를 매개로 얽힌 복잡한 관계망 속에서 끊임없이 성장하며 \
     프리저, 셀, 마인 부우 같은 강적들을 물리친다.",
    "손오공은 모든 연결의 중심에 서 있으며, 탐험가에서 최강의 전사로 변모한 그의 여정은 지구를 지키는 가족애와 무술에 대한 순수한 열정으로 빛난다. \
     그의 오랜 친구 크리링, 야무치, 천진반은 초반부터 함께 모험을 나누며 Z전사의 기반을 마련하고, \
     부르마 같은 과학자 지지자는 기술적 지원으로 그들의 생존을 돕는다.",
    "가족 관계는 드래곤볼 세계관에서 전투력의 원천이자 감정적 동기로 작용한다. \
     손오공은 어린 시절 치치와의 우연한 약속으로 결혼해 손오반과 손오천을 낳았는데, \
     이는 전형적인 연애 과정 없이 순수한 약속에서 비롯된 결합으로 토리야마 특유의 연애 무관심을 드러낸다. \
     베지터는 오공의 숙적에서 부르마와 결합해 트랭크스와 브라를 둔 가장으로 거듭난다.",
    "손오반은 학교 동급생 비델과 결혼해 팡을 낳으며 미스터 사탄과 사돈 관계를 맺고, \
     크리링은 인조인간 18호와의 적대에서 부부로 발전해 딸 마론을 얻는다. \
     이러한 가족들은 단순한 혈연을 넘어 전투 중 서로를 자극하고 지키는 유대로 기능한다.",
    "우정과 라이벌 관계는 드래곤볼의 역동성을 더하는 핵심 요소로, 전투를 통해 형성되는 유대가 반복적으로 나타난다. \
     오공과 베지터의 관계는 전형적인 '악우(惡友)' 패턴으로, 사이어인 왕자로서의 자존심이 오공의 자유로운 강함에 부딪히며 서로를 자극해 초인적 성장을 이끌어낸다. \
     오공-크리링 우정은 작품 최장기간 지속되며 생사를 공유한 동료애를 상징하고, \
     피콜로와 오반의 스승-제자 관계는 나메크성인과 혼혈 사이어인의 부자 같은 유대로 감동을 준다.",
];

/// RAG 챗봇
pub struct RagChatbot {
    llm: LlmClient,
    vectorstore: VectorStore,
    config: RagConfig,
}

impl RagChatbot {
    /// 새 RAG 챗봇을 생성한다.
    ///
    /// # Errors
    ///
    /// API 키/엔드포인트 누락 시 `RagError::Config`를 반환한다.
    pub fn new(config: RagConfig) -> Result<Self, RagError> {
        let llm = LlmClient::new(&config)?;
        let embedding_client = EmbeddingClient::new(&config)?;
        let vectorstore = VectorStore::new(embedding_client);

        Ok(Self {
            llm,
            vectorstore,
            config,
        })
    }

    /// 텍스트 리스트로부터 문서를 로드한다.
    ///
    /// # Errors
    ///
    /// 임베딩 생성 실패 시 에러를 반환한다.
    pub async fn load_from_texts(&mut self, texts: &[&str]) -> Result<(), RagError> {
        let documents: Vec<DocumentMeta> = texts
            .iter()
            .enumerate()
            .map(|(i, text)| DocumentMeta {
                content: (*text).to_string(),
                metadata: std::iter::once(("source".to_string(), format!("sample_doc_{i}"))).collect(),
            })
            .collect();

        self.vectorstore.add_documents(&documents).await?;
        tracing::info!("{}개 문서 로드 완료 (벡터스토어 크기: {})", documents.len(), self.vectorstore.len());
        Ok(())
    }

    /// 질문에 대한 RAG 응답을 생성한다.
    ///
    /// # Errors
    ///
    /// 벡터 검색 또는 LLM 호출 실패 시 에러를 반환한다.
    pub async fn query(&self, question: &str) -> Result<RagResponse, RagError> {
        // Top-K 문맥 검색
        let top_k = self.config.top_k;
        let retrieved = self.vectorstore.search(question, top_k).await?;

        let contexts: Vec<String> = retrieved.iter().map(|d| d.content.clone()).collect();
        let context_text = contexts.join("\n\n");

        // 프롬프트 구성
        let user_prompt = format!("## 문맥\n{context_text}\n\n위 문맥을 참고하여 질문에 답변해주세요.\n\n질문: {question}");

        // LLM 호출
        let answer = self.llm.chat(SYSTEM_PROMPT, &user_prompt).await?;

        Ok(RagResponse {
            question: question.to_string(),
            answer,
            contexts,
            source_documents: retrieved,
        })
    }

    /// 검색 결과만 반환한다 (평가용).
    ///
    /// # Errors
    ///
    /// 벡터 검색 실패 시 에러를 반환한다.
    pub async fn retrieve(&self, question: &str) -> Result<Vec<DocumentMeta>, RagError> { self.vectorstore.search(question, self.config.top_k).await }

    /// 사용 중인 백엔드 정보를 반환한다.
    #[must_use]
    pub const fn backend_info(&self) -> &str { if self.config.use_azure { "Azure OpenAI" } else { "OpenAI API" } }
}

/// 데모용 챗봇을 생성한다.
///
/// # Errors
///
/// API 키/엔드포인트 누락 또는 임베딩 실패 시 에러를 반환한다.
pub async fn create_demo_chatbot() -> Result<RagChatbot, RagError> {
    let config = RagConfig::from_env();
    let mut chatbot = RagChatbot::new(config)?;

    println!("[RagChatbot] 백엔드: {}", chatbot.backend_info());
    chatbot.load_from_texts(SAMPLE_DOCUMENTS).await?;

    Ok(chatbot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_documents_가_5개이다() {
        assert_eq!(SAMPLE_DOCUMENTS.len(), 5);
    }

    #[test]
    fn system_prompt_에_가드레일이_포함된다() {
        assert!(SYSTEM_PROMPT.contains("환각 방지"));
        assert!(SYSTEM_PROMPT.contains("민감한 정보"));
    }
}
