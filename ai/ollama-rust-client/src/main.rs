use reqwest::{Client, Error as ReqwestError, StatusCode};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::{self, Write};

// Custom error type for the application
#[derive(Debug)]
enum AppError {
    Network(ReqwestError),
    Api { status: StatusCode, message: String },
    Io(io::Error),
    // Removed unused variant: Other(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            | Self::Network(err) => write!(f, "Network error: {}", err),
            | Self::Api {
                status,
                message,
            } => write!(f, "API error ({}): {}", status, message),
            | Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl Error for AppError {}

impl From<ReqwestError> for AppError {
    fn from(err: ReqwestError) -> Self { Self::Network(err) }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self { Self::Io(err) }
}

type Result<T> = std::result::Result<T, AppError>;

/// Request body for Ollama API generation endpoint
#[derive(Serialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    // Additional options can be added here as needed
    // options: Option<serde_json::Value>,
}

/// Response body from Ollama API generation endpoint
#[derive(Deserialize, Debug)]
struct GenerateResponse {
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    created_at: String,
    response: String,
    #[allow(dead_code)]
    done: bool,
    // Additional fields can be added here as needed
    // context: Vec<i32>,
    // total_duration: u64,
}

/// Represents the identified intent of a user question
#[derive(Debug)]
struct Intent {
    description: String,
}

/// Represents the analysis of an intent
#[derive(Debug)]
struct Analysis {
    details: String,
}

/// Represents the answer to a user question
#[derive(Debug)]
struct Answer {
    content: String,
}

/// Represents a summary of an answer
#[derive(Debug)]
struct Summary {
    content: String,
}

/// Configuration constants
const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const DEFAULT_MODEL: &str = "phi4";

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

/// Ollama API client for making requests to the Ollama API
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
        println!("\n-----------------------------------------------------");
        println!("\n1. Identifying intent...");
        println!("\n-----------------------------------------------------");

        let prompt = format!("[SYSTEM]\n{}\n[QUESTION]\n{}", prompts::INTENT, question);

        println!("Sending intent identification request to Ollama...");
        let response = self.ollama.generate(&prompt).await?;

        println!("✓ Intent identified: {}", response);
        Ok(Intent {
            description: response,
        })
    }

    /// Analyze the identified intent
    async fn analyze_intent(&self, intent: &Intent, question: &str) -> Result<Analysis> {
        println!("\n-----------------------------------------------------");
        println!("\n2. Analyzing intent...");
        println!("\n-----------------------------------------------------");

        let prompt = format!("[SYSTEM]\n{}\n[INTENT]\n{}\n[QUESTION]\n{}", prompts::ANALYSIS, intent.description, question);

        println!("Sending intent analysis request to Ollama...");
        let response = self.ollama.generate(&prompt).await?;

        println!("✓ Analysis completed: {}", response);
        Ok(Analysis {
            details: response,
        })
    }

    /// Generate an answer based on the intent and analysis
    async fn generate_answer(&self, intent: &Intent, analysis: &Analysis, question: &str) -> Result<Answer> {
        println!("\n-----------------------------------------------------");
        println!("\n3. Inferring answer using Ollama API...");
        println!("\n-----------------------------------------------------");

        let prompt = format!(
            "[SYSTEM]\n{}\n[INTENT]\n{}\n[ANALYSIS]\n{}\n[QUESTION]\n{}",
            prompts::ANSWER,
            intent.description,
            analysis.details,
            question
        );

        println!("Sending answer inference request to Ollama...");
        let response = self.ollama.generate(&prompt).await?;

        println!("✓ Answer generated successfully");
        Ok(Answer {
            content: response,
        })
    }

    /// Summarize the answer
    async fn summarize_answer(&self, intent: &Intent, analysis: &Analysis, answer: &Answer, question: &str) -> Result<Summary> {
        println!("\n-----------------------------------------------------");
        println!("\n4. Summarizing result...");
        println!("\n-----------------------------------------------------");

        let prompt = format!(
            "[SYSTEM]\n{}\n[INTENT]\n{}\n[ANALYSIS]\n{}\n[DETAILED_ANSWER]\n{}\n[QUESTION]\n{}",
            prompts::SUMMARY,
            intent.description,
            analysis.details,
            answer.content,
            question
        );

        println!("Sending summary request to Ollama...");
        let response = self.ollama.generate(&prompt).await?;

        println!("✓ Summary created");
        Ok(Summary {
            content: response,
        })
    }

    /// Process a user question through the entire QA pipeline
    async fn process_question(&self, question: &str) -> Result<(Answer, Summary)> {
        println!("\n--- QA Pipeline Processing ---");
        println!("Starting QA pipeline for question: {}", question);

        let intent = self.identify_intent(question).await?;
        let analysis = self.analyze_intent(&intent, question).await?;
        let answer = self.generate_answer(&intent, &analysis, question).await?;
        let summary = self.summarize_answer(&intent, &analysis, &answer, question).await?;

        Ok((answer, summary))
    }
}

/// Read a line of user input
fn read_user_input() -> Result<String> {
    print!("Enter your question (press Enter to submit): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

/// Display welcome message
fn display_welcome_message() {
    println!("=== Ollama QA Pipeline Client ===");
    println!("Type your question and press Enter to submit.");
    println!("Type /quit or /bye to exit the program.");
    println!("Press Ctrl+C to terminate at any time.\n");
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Initialize the QA pipeline with the default model
    let pipeline = QaPipeline::new(DEFAULT_MODEL);

    // Display welcome message
    display_welcome_message();

    // Main input loop
    loop {
        // Read user input
        let user_question = match read_user_input() {
            | Ok(input) => input,
            | Err(e) => {
                eprintln!("Error reading input: {}", e);
                continue;
            },
        };

        // Check for exit commands
        if user_question == "/quit" || user_question == "/bye" {
            println!("Exiting program. Goodbye!");
            break;
        }

        // Check for empty input
        if user_question.is_empty() {
            println!("Question cannot be empty. Please try again.\n");
            continue;
        }

        // Process the question through the QA pipeline
        match pipeline.process_question(&user_question).await {
            | Ok((answer, summary)) => {
                // Display the results
                println!("\n--- Final Result ---");
                println!("{}", summary.content);
                println!("\n--- Detailed Answer ---");
                println!("{}", answer.content);
                println!("\n-----------------------\n");
            },
            | Err(e) => {
                eprintln!("Error processing question: {}", e);
                println!("Please try again with a different question.\n");
            },
        }
    }

    Ok(())
}
