use axum::extract::State;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use reqwest::{Client, Error as ReqwestError, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Configuration constants
const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const DEFAULT_MODEL: &str = "phi4";
const WEB_PORT: u16 = 3000;

/// System prompts for each stage of the QA pipeline
mod prompts {
    pub const INTENT: &str = "You are an AI assistant specialized in identifying the user's intent from their question. 
Your task is to analyze the question and provide a clear, detailed description of what the user is asking for. 
Consider the subject matter, the type of information requested, and any specific constraints or preferences mentioned. 
Your response should be comprehensive enough to guide the next stages of analysis.";

    pub const ANALYSIS: &str = "You are an AI assistant specialized in analyzing user intents. 
Your task is to take the identified intent and provide a detailed analysis of what information would be needed to answer the question comprehensively. 
Break down the question into its key components, identify relevant topics, concepts, and potential sources of information. 
Your analysis should be structured and thorough, preparing the groundwork for generating a complete answer.";

    pub const ANSWER: &str = "You are a knowledgeable AI assistant tasked with providing comprehensive answers. 
Based on the provided intent and analysis, deliver a detailed, accurate, and well-structured response to the user's question. 
Include relevant facts, examples, and explanations. 
Ensure your answer directly addresses all aspects identified in the intent and analysis. 
Your response should be authoritative and educational while remaining accessible.";

    pub const SUMMARY: &str = "You are an AI assistant specialized in creating concise summaries. 
Your task is to distill the detailed answer into a clear, structured summary that captures all key points. 
Highlight the most important information, organize it logically, and ensure it directly answers the original question. 
Your summary should be comprehensive yet concise, allowing the user to quickly grasp the essential information without reading the full detailed answer.";
}

// Custom error type for the application
#[derive(Debug)]
enum AppError {
    Network(ReqwestError),
    Api { status: StatusCode, message: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | Self::Network(err) => write!(f, "Network error: {}", err),
            | Self::Api {
                status,
                message,
            } => write!(f, "API error ({}): {}", status, message),
        }
    }
}

impl Error for AppError {}

impl From<ReqwestError> for AppError {
    fn from(err: ReqwestError) -> Self { Self::Network(err) }
}

type Result<T> = std::result::Result<T, AppError>;

/// Request body for Ollama API generation endpoint
#[derive(Serialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

/// Response body from Ollama API generation endpoint
#[derive(Deserialize, Debug)]
struct GenerateResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
}

/// Represents the identified intent of a user question
#[derive(Debug, Clone)]
struct Intent {
    description: String,
}

/// Represents the analysis of an intent
#[derive(Debug, Clone)]
struct Analysis {
    details: String,
}

/// Represents the answer to a user question
#[derive(Debug, Clone)]
struct Answer {
    content: String,
}

/// Represents a summary of an answer
#[derive(Debug, Clone)]
struct Summary {
    content: String,
}

/// Web form for submitting a question
#[derive(Deserialize)]
struct QuestionForm {
    question: String,
}

/// Response for the web API
#[derive(Serialize)]
struct ApiResponse {
    intent: String,
    analysis: String,
    answer: String,
    summary: String,
    step: String, // Current processing step
    done: bool,   // Whether processing is complete
}

/// Ollama API client for making requests to the Ollama API
#[derive(Clone)]
struct OllamaClient {
    client: Client,
    api_url: String,
    model: String,
}

impl OllamaClient {
    /// Create a new Ollama API client
    fn new(model: &str) -> Self {
        Self {
            client: Client::new(),
            api_url: OLLAMA_API_URL.to_string(),
            model: model.to_string(),
        }
    }

    /// Send a prompt to the Ollama API and get a response
    async fn generate(&self, prompt: &str) -> Result<String> {
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response = self.client.post(&self.api_url).json(&request).send().await?;

        if response.status().is_success() {
            let response_body = response.json::<GenerateResponse>().await?;
            Ok(response_body.response)
        } else {
            let status = response.status();
            let error_body = response.text().await?;
            Err(AppError::Api {
                status,
                message: error_body,
            })
        }
    }
}

/// QA Pipeline for processing user questions through multiple stages
#[derive(Clone)]
struct QaPipeline {
    ollama: OllamaClient,
}

impl QaPipeline {
    /// Create a new QA pipeline with the specified model
    fn new(model: &str) -> Self {
        Self {
            ollama: OllamaClient::new(model),
        }
    }

    /// Identify the intent of a user question
    async fn identify_intent(&self, question: &str) -> Result<Intent> {
        let prompt = format!("[SYSTEM]\n{}\n[QUESTION]\n{}", prompts::INTENT, question);
        let response = self.ollama.generate(&prompt).await?;

        Ok(Intent {
            description: response,
        })
    }

    /// Analyze the identified intent
    async fn analyze_intent(&self, intent: &Intent, question: &str) -> Result<Analysis> {
        let prompt = format!("[SYSTEM]\n{}\n[INTENT]\n{}\n[QUESTION]\n{}", prompts::ANALYSIS, intent.description, question);
        let response = self.ollama.generate(&prompt).await?;

        Ok(Analysis {
            details: response,
        })
    }

    /// Generate an answer based on the intent and analysis
    async fn generate_answer(&self, intent: &Intent, analysis: &Analysis, question: &str) -> Result<Answer> {
        let prompt = format!(
            "[SYSTEM]\n{}\n[INTENT]\n{}\n[ANALYSIS]\n{}\n[QUESTION]\n{}",
            prompts::ANSWER,
            intent.description,
            analysis.details,
            question
        );

        let response = self.ollama.generate(&prompt).await?;

        Ok(Answer {
            content: response,
        })
    }

    /// Summarize the answer
    async fn summarize_answer(&self, intent: &Intent, analysis: &Analysis, answer: &Answer, question: &str) -> Result<Summary> {
        let prompt = format!(
            "[SYSTEM]\n{}\n[INTENT]\n{}\n[ANALYSIS]\n{}\n[DETAILED_ANSWER]\n{}\n[QUESTION]\n{}",
            prompts::SUMMARY,
            intent.description,
            analysis.details,
            answer.content,
            question
        );

        let response = self.ollama.generate(&prompt).await?;

        Ok(Summary {
            content: response,
        })
    }

    /// Process a user question through the entire QA pipeline
    async fn process_question(&self, question: &str) -> Result<(Answer, Summary)> {
        let intent = self.identify_intent(question).await?;
        let analysis = self.analyze_intent(&intent, question).await?;
        let answer = self.generate_answer(&intent, &analysis, question).await?;
        let summary = self.summarize_answer(&intent, &analysis, &answer, question).await?;

        Ok((answer, summary))
    }
}

/// HTML template for the index page
const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ollama QA Pipeline</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            color: #333;
        }
        h1 {
            color: #2c3e50;
            text-align: center;
        }
        form {
            background: #f9f9f9;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 20px;
        }
        textarea {
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            min-height: 100px;
            font-family: inherit;
            margin-bottom: 10px;
        }
        button {
            background: #3498db;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 4px;
            cursor: pointer;
            font-size: 16px;
        }
        button:hover {
            background: #2980b9;
        }
        .result {
            display: none;
            background: #f9f9f9;
            padding: 20px;
            border-radius: 8px;
            margin-top: 30px;
        }
        
        .pipeline-step {
            background-color: #f9f9f9;
            padding: 20px;
            border-radius: 5px;
            margin-bottom: 20px;
            border-left: 4px solid #ccc;
        }
        
        .pipeline-step.active {
            border-left-color: #3498db;
        }
        
        .pipeline-step.completed {
            border-left-color: #2ecc71;
        }
        
        .pipeline-step.error {
            border-left-color: #e74c3c;
        }
        
        .status {
            font-weight: bold;
            margin-bottom: 10px;
            color: #666;
        }
        
        .status.processing {
            color: #3498db;
        }
        
        .status.completed {
            color: #2ecc71;
        }
        
        .status.error {
            color: #e74c3c;
        }
        
        .content {
            display: none;
            margin-top: 10px;
            padding-top: 10px;
            border-top: 1px solid #eee;
        }
        
        .loading {
            text-align: center;
            display: none;
            margin: 20px 0;
        }
        
        /* Markdown styling */
        .content {
            line-height: 1.6;
        }
        
        .content h1, .content h2, .content h3 {
            margin-top: 1em;
            margin-bottom: 0.5em;
        }
        
        .content p {
            margin-bottom: 1em;
        }
        
        .content pre {
            background-color: #f0f0f0;
            padding: 10px;
            border-radius: 4px;
            overflow-x: auto;
        }
        
        .content code {
            background-color: #f0f0f0;
            padding: 2px 4px;
            border-radius: 3px;
            font-family: monospace;
        }
        
        .content ul, .content ol {
            padding-left: 2em;
            margin-bottom: 1em;
        }
        
        .content blockquote {
            border-left: 4px solid #ddd;
            padding-left: 1em;
            margin-left: 0;
            color: #666;
        }
    </style>
</head>
<body>
    <h1>Ollama QA Pipeline</h1>
    <form id="questionForm">
        <div>
            <label for="question">Enter your question:</label>
            <textarea id="question" name="question" placeholder="Type your question here..." required></textarea>
        </div>
        <button type="submit">Submit Question</button>
    </form>
    
    <div class="loading" id="loading">
        <p>Processing your question... This may take a few moments.</p>
    </div>
    
    <div class="result" id="result">
        <div class="pipeline-step" id="intentStep">
            <h2>Step 1: Intent Identification</h2>
            <div class="status" id="intentStatus">Waiting...</div>
            <div id="intentContent" class="content"></div>
        </div>
        <div class="pipeline-step" id="analysisStep">
            <h2>Step 2: Analysis</h2>
            <div class="status" id="analysisStatus">Waiting...</div>
            <div id="analysisContent" class="content"></div>
        </div>
        <div class="pipeline-step" id="answerStep">
            <h2>Step 3: Detailed Answer</h2>
            <div class="status" id="answerStatus">Waiting...</div>
            <div id="answerContent" class="content"></div>
        </div>
        <div class="pipeline-step" id="summaryStep">
            <h2>Step 4: Summary</h2>
            <div class="status" id="summaryStatus">Waiting...</div>
            <div id="summaryContent" class="content"></div>
        </div>
    </div>
    
    <!-- Include marked.js for Markdown rendering -->
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
    <script>
        document.getElementById('questionForm').addEventListener('submit', async function(e) {
            e.preventDefault();
            
            const question = document.getElementById('question').value;
            if (!question.trim()) return;
            
            // Reset UI state
            resetUI();
            
            // Show loading indicator and result container
            document.getElementById('loading').style.display = 'block';
            document.getElementById('result').style.display = 'block';
            
            try {
                // Start the streaming process
                await processQuestionStream(question);
            } catch (error) {
                alert('Error: ' + error.message);
                console.error(error);
            } finally {
                document.getElementById('loading').style.display = 'none';
            }
        });
        
        // Reset the UI state for a new question
        function resetUI() {
            // Reset all step statuses
            document.querySelectorAll('.pipeline-step').forEach(step => {
                step.className = 'pipeline-step';
            });
            
            document.querySelectorAll('.status').forEach(status => {
                status.className = 'status';
                status.textContent = 'Waiting...';
            });
            
            document.querySelectorAll('.content').forEach(content => {
                content.style.display = 'none';
                content.innerHTML = '';
            });
        }
        
        // Process a question through the streaming API
        async function processQuestionStream(question) {
            // Step 1: Intent Identification
            updateStepStatus('intent', 'processing', 'Processing...');
            
            const response = await fetch('/api/question/stream', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ question })
            });
            
            if (!response.ok) {
                throw new Error('Failed to process question');
            }
            
            const data = await response.json();
            
            // Display each step's result
            if (data.intent) {
                updateStepStatus('intent', 'completed', 'Completed');
                document.getElementById('intentContent').innerHTML = marked.parse(data.intent);
                document.getElementById('intentContent').style.display = 'block';
            }
            
            if (data.analysis) {
                updateStepStatus('analysis', 'completed', 'Completed');
                document.getElementById('analysisContent').innerHTML = marked.parse(data.analysis);
                document.getElementById('analysisContent').style.display = 'block';
            }
            
            if (data.answer) {
                updateStepStatus('answer', 'completed', 'Completed');
                document.getElementById('answerContent').innerHTML = marked.parse(data.answer);
                document.getElementById('answerContent').style.display = 'block';
            }
            
            if (data.summary) {
                updateStepStatus('summary', 'completed', 'Completed');
                document.getElementById('summaryContent').innerHTML = marked.parse(data.summary);
                document.getElementById('summaryContent').style.display = 'block';
            }
            
            // Handle errors or incomplete processing
            if (data.step === 'error') {
                const stepId = getErrorStep(data);
                updateStepStatus(stepId, 'error', 'Error');
            }
        }
        
        // Update the status of a pipeline step
        function updateStepStatus(stepId, statusClass, statusText) {
            const step = document.getElementById(stepId + 'Step');
            const status = document.getElementById(stepId + 'Status');
            
            // Remove all classes and add the new one
            step.className = 'pipeline-step ' + statusClass;
            status.className = 'status ' + statusClass;
            status.textContent = statusText;
        }
        
        // Determine which step had an error based on available data
        function getErrorStep(data) {
            if (!data.intent) return 'intent';
            if (!data.analysis) return 'analysis';
            if (!data.answer) return 'answer';
            if (!data.summary) return 'summary';
            return 'unknown';
        }
    </script>
</body>
</html>"#;

/// Handler for the root path
async fn index() -> Html<&'static str> { Html(INDEX_HTML) }

/// Handler for the API endpoint that processes the entire pipeline at once
async fn process_question(State(pipeline): State<QaPipeline>, Json(form): Json<QuestionForm>) -> Json<ApiResponse> {
    match pipeline.process_question(&form.question).await {
        | Ok((answer, summary)) => Json(ApiResponse {
            intent: "".to_string(),
            analysis: "".to_string(),
            answer: answer.content,
            summary: summary.content,
            step: "complete".to_string(),
            done: true,
        }),
        | Err(e) => {
            // In a real application, you'd want to handle errors more gracefully
            eprintln!("Error processing question: {}", e);
            Json(ApiResponse {
                intent: "".to_string(),
                analysis: "".to_string(),
                answer: format!("An error occurred: {}", e),
                summary: "Error processing your question".to_string(),
                step: "error".to_string(),
                done: true,
            })
        },
    }
}

/// Handler for the streaming API endpoint that processes the pipeline step by
/// step
async fn process_question_stream(State(pipeline): State<QaPipeline>, Json(form): Json<QuestionForm>) -> Json<ApiResponse> {
    // Step 1: Identify intent
    let intent = match pipeline.identify_intent(&form.question).await {
        | Ok(intent) => intent,
        | Err(e) => {
            eprintln!("Error identifying intent: {}", e);
            return Json(ApiResponse {
                intent: "".to_string(),
                analysis: "".to_string(),
                answer: format!("Error identifying intent: {}", e),
                summary: "Error processing your question".to_string(),
                step: "intent".to_string(),
                done: true,
            });
        },
    };

    // Step 2: Analyze intent
    let analysis = match pipeline.analyze_intent(&intent, &form.question).await {
        | Ok(analysis) => analysis,
        | Err(e) => {
            eprintln!("Error analyzing intent: {}", e);
            return Json(ApiResponse {
                intent: intent.description,
                analysis: "".to_string(),
                answer: format!("Error analyzing intent: {}", e),
                summary: "Error processing your question".to_string(),
                step: "analysis".to_string(),
                done: true,
            });
        },
    };

    // Step 3: Generate answer
    let answer = match pipeline.generate_answer(&intent, &analysis, &form.question).await {
        | Ok(answer) => answer,
        | Err(e) => {
            eprintln!("Error generating answer: {}", e);
            return Json(ApiResponse {
                intent: intent.description,
                analysis: analysis.details,
                answer: format!("Error generating answer: {}", e),
                summary: "Error processing your question".to_string(),
                step: "answer".to_string(),
                done: true,
            });
        },
    };

    // Step 4: Summarize answer
    let summary = match pipeline.summarize_answer(&intent, &analysis, &answer, &form.question).await {
        | Ok(summary) => summary,
        | Err(e) => {
            eprintln!("Error summarizing answer: {}", e);
            return Json(ApiResponse {
                intent: intent.description,
                analysis: analysis.details,
                answer: answer.content,
                summary: format!("Error summarizing answer: {}", e),
                step: "summary".to_string(),
                done: true,
            });
        },
    };

    // Return the complete response
    Json(ApiResponse {
        intent: intent.description,
        analysis: analysis.details,
        answer: answer.content,
        summary: summary.content,
        step: "complete".to_string(),
        done: true,
    })
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the QA pipeline with the default model
    let pipeline = QaPipeline::new(DEFAULT_MODEL);

    // Build our application with routes
    let app = Router::new()
        .route("/", get(index))
        .route("/api/question", post(process_question))
        .route("/api/question/stream", post(process_question_stream))
        .with_state(pipeline);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], WEB_PORT));
    tracing::info!("listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
