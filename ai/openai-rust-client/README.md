# OpenAI Rust Chatbot

A web-based chatbot application built with Rust and Axum that uses OpenAI's API to generate responses.

## Features

- Multi-turn conversation with OpenAI's GPT models
- Modern and responsive UI
- Support for Korean language
- Markdown-like formatting in chat messages

## Prerequisites

- Rust and Cargo installed
- An OpenAI API key (Azure OpenAI Service)

## Setup

1. Clone the repository
2. Create a `.env` file in the root directory based on `.env.example`
3. Add your OpenAI API key to the `.env` file

```
AZURE_API_KEY=your_api_key_here
OPENAI_ENDPOINT=https://prototyping-demo-ai.openai.azure.com/openai/deployments/gpt-4o/chat/completions?api-version=2025-01-01-preview
OPENAI_MODEL=gpt-4o
```

## Running the Application

```bash
cargo run
```

The application will be available at http://localhost:8080

## Project Structure

- `src/main.rs` - Main application code
- `static/` - Frontend files
  - `index.html` - HTML structure
  - `styles.css` - CSS styling
  - `script.js` - JavaScript for handling chat functionality

## License

MIT
