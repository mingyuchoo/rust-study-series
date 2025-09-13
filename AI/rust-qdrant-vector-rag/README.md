# Rust Qdrant Vector RAG

A modern full-stack application featuring a high-performance Rust backend with Azure OpenAI integration and Qdrant vector database, paired with a responsive frontend built with Svelte and TypeScript.

## 🏗️ Architecture Overview

This project consists of three main components:

- **Backend**: Rust-based RAG (Retrieval-Augmented Generation) service using Actix Web
- **Frontend**: Svelte/TypeScript application with Vite build system
- **Docker**: Containerized deployment configuration

```
project-root/
├── backend/           # Rust RAG service
├── frontend/          # Svelte/TypeScript UI
├── docker/            # Docker configuration
└── README.md          # This file
```

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.78+ (for backend)
- **Node.js** 18+ and **pnpm** (for frontend)
- **Docker** and **Docker Compose** (for services)
- **Azure OpenAI** resource with embedding and chat deployments

### 1. Start Infrastructure Services

```bash
cd docker
docker-compose up -d
```

This starts Qdrant vector database and any other required services.

### 2. Configure Environment

Copy environment files and configure:

```bash
# Backend configuration
cp backend/.env.example backend/.env
# Edit backend/.env with your Azure OpenAI credentials

# Frontend configuration  
cp frontend/.env.example frontend/.env
# Edit frontend/.env with API endpoints
```

### 3. Start Backend

```bash
cd backend
cargo run
```

The backend API will be available at `http://localhost:8080`

### 4. Start Frontend

```bash
cd frontend
pnpm install
pnpm dev
```

The frontend will be available at `http://localhost:5173`

## 📁 Project Structure

### Backend (`/backend`)

The Rust backend provides a high-performance RAG service with the following architecture:

```
backend/src/
├── main.rs              # Server entry point and routing
├── app.rs               # DI container and application setup
├── docs.rs              # OpenAPI/Swagger configuration
├── handlers/            # HTTP request handlers
│   ├── health.rs        # Health check endpoints
│   ├── upload.rs        # Document upload handlers
│   ├── query.rs         # Query processing handlers
│   └── monitoring.rs    # Metrics and monitoring
├── services/            # Business logic layer
│   ├── document.rs      # Document processing service
│   ├── chunker.rs       # Text chunking pipeline
│   ├── embedding.rs     # Azure OpenAI embedding service
│   ├── vector_search.rs # Qdrant vector search
│   ├── rag.rs           # RAG orchestration
│   ├── cache.rs         # Caching layer
│   └── resilience.rs    # Retry and circuit breaker
├── clients/             # External service clients
├── repository/          # Data access layer (Qdrant)
├── config/              # Configuration management
├── middleware/          # HTTP middleware
├── monitoring/          # Metrics and performance
└── models/              # Data models and types
```

**Key Features:**
- Document upload and processing (Markdown support)
- Azure OpenAI embedding generation with batch processing
- Qdrant vector storage and similarity search
- RAG pipeline with context retrieval and answer generation
- Comprehensive monitoring with Prometheus metrics
- OpenAPI documentation with Swagger UI
- Resilient design with retry logic and caching

### Frontend (`/frontend`)

Modern Svelte application with TypeScript:

```
frontend/src/
├── app.html             # HTML template
├── app.css              # Global styles
├── main.ts              # Application entry point
├── lib/                 # Reusable components and utilities
├── routes/              # SvelteKit routes (if using SvelteKit)
└── static/              # Static assets
```

**Key Features:**
- Responsive UI built with Svelte and TypeScript
- Tailwind CSS for styling
- Vite for fast development and building
- ESLint and Prettier for code quality
- Vitest for testing

### Docker Configuration (`/docker`)

Containerized deployment setup:

- `docker-compose.yml`: Orchestrates Qdrant and other services
- `.dockerignore`: Optimizes Docker build context

## 🔧 Configuration

### Backend Configuration

Key environment variables in `backend/.env`:

```bash
# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
SERVER_MAX_REQUEST_SIZE=10485760
SERVER_TIMEOUT_SECONDS=30

# Azure OpenAI
AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com/
AZURE_OPENAI_API_KEY=your-api-key
AZURE_OPENAI_EMBEDDING_DEPLOYMENT=text-embedding-3-large
AZURE_OPENAI_CHAT_DEPLOYMENT=gpt-4

# Qdrant Vector Database
QDRANT_URL=http://localhost:6334
QDRANT_COLLECTION_NAME=documents
QDRANT_VECTOR_SIZE=3072

# Performance & Caching
CACHE_TTL_SECONDS=3600
MAX_CONCURRENT_REQUESTS=100
```

### Frontend Configuration

Configure API endpoints in `frontend/.env`:

```bash
VITE_API_BASE_URL=http://localhost:8080/api/v1
VITE_APP_TITLE=RAG Application
```

## 📡 API Endpoints

The backend provides a comprehensive REST API:

### Document Management
- `POST /api/v1/upload` - Upload documents (multipart)
- `POST /api/v1/upload/json` - Upload via JSON

### Query Processing  
- `POST /api/v1/query` - Process RAG queries
- `GET /api/v1/query/{question}` - Simple query via URL

### Monitoring & Management
- `GET /api/v1/health` - Health check
- `GET /api/v1/metrics` - Application metrics
- `GET /api/v1/metrics/prometheus` - Prometheus format metrics
- `GET /api/v1/cache/stats` - Cache statistics
- `POST /api/v1/cache/clear` - Clear cache

### Documentation
- `GET /swagger-ui/` - Interactive API documentation

## 🧪 Testing

### Backend Testing

```bash
cd backend

# Run unit tests
cargo test

# Run with coverage
cargo tarpaulin --ignore-tests

# Run integration tests
cargo test --test integration_tests

# Performance benchmarks
cargo test --release benchmark
```

### Frontend Testing

```bash
cd frontend

# Run unit tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with coverage
pnpm test:coverage

# Type checking
pnpm check
```

## 📊 Monitoring

The application includes comprehensive monitoring:

- **Prometheus Metrics**: Request rates, latencies, error rates
- **Health Checks**: Service availability and dependency status  
- **Performance Monitoring**: Response times and resource usage
- **Cache Statistics**: Hit rates and memory usage
- **Logging**: Structured logging with tracing

Access metrics at:
- Application metrics: `GET /api/v1/metrics`
- Prometheus format: `GET /api/v1/metrics/prometheus`

## 🚀 Deployment

### Development

Use the quick start guide above for local development.

### Production

1. **Build the application:**

```bash
# Build backend
cd backend
cargo build --release

# Build frontend
cd frontend
pnpm build
```

2. **Deploy with Docker:**

```bash
cd docker
docker-compose -f docker-compose.prod.yml up -d
```

## 🛠️ Development Tools

### Backend Tools
- **cargo-make**: Task runner (see `Makefile.toml`)
- **rustfmt**: Code formatting
- **clippy**: Linting
- **tarpaulin**: Coverage reporting

### Frontend Tools
- **ESLint**: Code linting
- **Prettier**: Code formatting  
- **Vitest**: Testing framework
- **TypeScript**: Type checking

## 🔍 Troubleshooting

### Common Issues

**Backend Issues:**
- **Rate limiting**: Reduce request frequency or use batch embedding
- **Vector dimension mismatch**: Ensure embedding model matches `QDRANT_VECTOR_SIZE`
- **Connection errors**: Verify Qdrant is running and accessible

**Frontend Issues:**
- **API connection**: Check `VITE_API_BASE_URL` configuration
- **Build errors**: Ensure Node.js version compatibility
- **Type errors**: Run `pnpm check` for TypeScript validation

### Logs and Debugging

- Backend logs: Structured logging with configurable levels
- Frontend logs: Browser console and network tab
- Service logs: `docker-compose logs <service-name>`

## 📄 License

This project follows the organization's licensing policy. See individual component directories for specific license information.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

For detailed contribution guidelines, see the individual component READMEs in `/backend` and `/frontend` directories.