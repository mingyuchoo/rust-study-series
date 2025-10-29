// Cargo.toml 의존성:
// [dependencies]
// tower-lsp = "0.20"
// tokio = { version = "1", features = ["full"] }
// serde_json = "1.0"
// regex = "1.10"

use regex::Regex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

// 한국어 키워드 정의
struct KoreanKeyword {
    label: &'static str,
    kind: CompletionItemKind,
    detail: &'static str,
    documentation: &'static str,
}

const KEYWORDS: &[KoreanKeyword] = &[
    KoreanKeyword {
        label: "함수",
        kind: CompletionItemKind::KEYWORD,
        detail: "함수 선언",
        documentation: "함수 이름(매개변수) {\n  // 코드\n}",
    },
    KoreanKeyword {
        label: "변수",
        kind: CompletionItemKind::KEYWORD,
        detail: "변수 선언",
        documentation: "변수 이름 = 값",
    },
    KoreanKeyword {
        label: "만약",
        kind: CompletionItemKind::KEYWORD,
        detail: "조건문",
        documentation: "만약 (조건) {\n  // 코드\n}",
    },
    KoreanKeyword {
        label: "아니면",
        kind: CompletionItemKind::KEYWORD,
        detail: "else 조건",
        documentation: "아니면 {\n  // 코드\n}",
    },
    KoreanKeyword {
        label: "반복",
        kind: CompletionItemKind::KEYWORD,
        detail: "반복문",
        documentation: "반복 (조건) {\n  // 코드\n}",
    },
    KoreanKeyword {
        label: "출력",
        kind: CompletionItemKind::FUNCTION,
        detail: "콘솔 출력",
        documentation: "출력(\"메시지\")",
    },
    KoreanKeyword {
        label: "참",
        kind: CompletionItemKind::VALUE,
        detail: "true",
        documentation: "불린 참 값",
    },
    KoreanKeyword {
        label: "거짓",
        kind: CompletionItemKind::VALUE,
        detail: "false",
        documentation: "불린 거짓 값",
    },
];

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "한국어 Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) { self.client.log_message(MessageType::INFO, "한국어 Language Server 초기화 완료").await; }

    async fn shutdown(&self) -> Result<()> { Ok(()) }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let items: Vec<CompletionItem> = KEYWORDS
            .iter()
            .map(|kw| CompletionItem {
                label: kw.label.to_string(),
                kind: Some(kw.kind),
                detail: Some(kw.detail.to_string()),
                documentation: Some(Documentation::String(kw.documentation.to_string())),
                ..Default::default()
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn completion_resolve(&self, mut item: CompletionItem) -> Result<CompletionItem> {
        if let Some(keyword) = KEYWORDS.iter().find(|kw| kw.label == item.label) {
            item.detail = Some(keyword.detail.to_string());
            item.documentation = Some(Documentation::String(keyword.documentation.to_string()));
        }
        Ok(item)
    }

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        // 실제 구현에서는 문서 내용을 저장하고 관리해야 합니다
        // 여기서는 간단한 예시로 첫 번째 키워드 정보를 보여줍니다

        if let Some(keyword) = KEYWORDS.first() {
            // 간단한 호버 정보 제공 (실제로는 문서의 텍스트를 파싱해야 함)
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**{}**\n\n{}", keyword.label, keyword.detail),
                }),
                range: None,
            }));
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client.log_message(MessageType::INFO, "파일 열림").await;

        self.validate_document(params.text_document.uri, params.text_document.text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            self.validate_document(params.text_document.uri, change.text.clone()).await;
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) { self.client.log_message(MessageType::INFO, "파일 저장됨").await; }

    async fn did_close(&self, _: DidCloseTextDocumentParams) { self.client.log_message(MessageType::INFO, "파일 닫힘").await; }
}

impl Backend {
    async fn validate_document(&self, uri: Url, text: String) {
        let mut diagnostics = Vec::new();

        // 예시: "변수" 키워드 다음에 이름이 없는 경우 경고
        let var_pattern = Regex::new(r"변수\s*$").unwrap();

        for (line_num, line) in text.lines().enumerate() {
            if let Some(mat) = var_pattern.find(line) {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: mat.start() as u32,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: mat.end() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some("korean-lang".to_string()),
                    message: "변수 이름이 필요합니다".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
