use serde::{Deserialize, Serialize};

/// Foundry 모델 열거형
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundryModel {
    GptOss20b,
    Phi4MiniInstruct,
    Qwen25_7bInstruct,
    Custom(String),
}

impl FoundryModel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GptOss20b => "gpt-oss-20b-cuda-gpu:1",
            Self::Phi4MiniInstruct => "Phi-4-mini-instruct-cuda-gpu:4",
            Self::Qwen25_7bInstruct => "qwen2.5-7b-instruct-cuda-gpu:3",
            Self::Custom(s) => s.as_str(),
        }
    }
}

impl From<&str> for FoundryModel {
    fn from(s: &str) -> Self {
        match s {
            "gpt-oss-20b-cuda-gpu:1" => Self::GptOss20b,
            "Phi-4-mini-instruct-cuda-gpu:4" => Self::Phi4MiniInstruct,
            "qwen2.5-7b-instruct-cuda-gpu:3" => Self::Qwen25_7bInstruct,
            _ => Self::Custom(s.to_string()),
        }
    }
}

/// 채팅 메시지 역할
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

/// 채팅 메시지
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }
}

/// 채팅 완성 요청
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// 채팅 완성 응답의 선택지
#[derive(Debug, Clone, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
    pub index: u32,
}

/// 채팅 완성 응답
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionResponse {
    pub choices: Vec<ChatChoice>,
}

impl ChatCompletionResponse {
    pub fn first_content(&self) -> Option<&str> {
        self.choices
            .first()
            .map(|choice| choice.message.content.as_str())
    }
}

/// 빌더 패턴을 사용한 채팅 완성 요청 생성
pub struct ChatCompletionRequestBuilder {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

impl ChatCompletionRequestBuilder {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
        }
    }

    pub fn message(mut self, message: ChatMessage) -> Self {
        self.messages.push(message);
        self
    }

    pub fn messages(mut self, messages: Vec<ChatMessage>) -> Self {
        self.messages = messages;
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn build(self) -> ChatCompletionRequest {
        ChatCompletionRequest {
            model: self.model,
            messages: self.messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }
}
