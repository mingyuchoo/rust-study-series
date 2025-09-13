# Design Document

## Overview

The Svelte frontend will be a single-page application (SPA) that provides an intuitive interface for users to upload PDF documents and perform AI-powered searches. The application will be built using Svelte 5 with TypeScript, SvelteUI for styling, and will communicate with the existing Rust backend through RESTful APIs. The design emphasizes user experience, accessibility, and responsive design principles.

## Architecture

### High-Level Architecture

```mermaid
graph TB
    subgraph "Frontend (Svelte)"
        A[App.svelte] --> B[Router]
        B --> C[Upload Page]
        B --> D[Search Page]
        B --> E[Dashboard Page]
        
        C --> F[FileUpload Component]
        D --> G[SearchForm Component]
        D --> H[ResultsDisplay Component]
        E --> I[HealthStatus Component]
        
        F --> J[API Service]
        G --> J
        I --> J
    end
    
    subgraph "Backend (Rust)"
        J --> K[/upload endpoint]
        J --> L[/query endpoint]
        J --> M[/health endpoint]
        
        K --> N[Document Service]
        L --> O[RAG Service]
        M --> P[Health Service]
    end
    
    subgraph "External Services"
        N --> Q[Qdrant Vector DB]
        O --> R[Azure OpenAI]
    end
```

### Component Hierarchy

```
App.svelte
├── Navigation.svelte
├── Router.svelte
│   ├── UploadPage.svelte
│   │   ├── FileUpload.svelte
│   │   ├── UploadProgress.svelte
│   │   └── UploadResult.svelte
│   ├── SearchPage.svelte
│   │   ├── SearchForm.svelte
│   │   ├── SearchConfig.svelte
│   │   ├── LoadingSpinner.svelte
│   │   └── SearchResults.svelte
│   │       ├── AnswerDisplay.svelte
│   │       └── SourceReferences.svelte
│   └── DashboardPage.svelte
│       ├── HealthStatus.svelte
│       └── SystemMetrics.svelte
├── ErrorBoundary.svelte
└── Toast.svelte (for notifications)
```

## Components and Interfaces

### Core Services

#### API Service (`src/lib/services/api.ts`)
```typescript
interface ApiService {
  uploadDocument(file: File): Promise<UploadResponse>
  queryDocuments(query: QueryRequest): Promise<RAGResponse>
  getHealth(): Promise<HealthResponse>
}

interface QueryRequest {
  question: string
  config?: QueryConfig
}

interface QueryConfig {
  max_chunks?: number
  similarity_threshold?: number
  max_response_tokens?: number
  temperature?: number
  include_low_confidence?: boolean
}
```

#### State Management (`src/lib/stores/`)
```typescript
// app.store.ts
interface AppState {
  isLoading: boolean
  error: string | null
  currentPage: string
}

// upload.store.ts
interface UploadState {
  uploadProgress: number
  isUploading: boolean
  uploadResult: UploadResponse | null
}

// search.store.ts
interface SearchState {
  query: string
  results: RAGResponse | null
  isSearching: boolean
  searchConfig: QueryConfig
}

// health.store.ts
interface HealthState {
  status: HealthResponse | null
  lastChecked: Date | null
  isChecking: boolean
}
```

### UI Components

#### FileUpload Component
```typescript
interface FileUploadProps {
  onFileSelect: (file: File) => void
  acceptedTypes: string[]
  maxSize: number
  disabled?: boolean
}
```

#### SearchForm Component
```typescript
interface SearchFormProps {
  onSubmit: (query: string, config?: QueryConfig) => void
  disabled?: boolean
  showAdvanced?: boolean
}
```

#### SearchResults Component
```typescript
interface SearchResultsProps {
  results: RAGResponse
  onSourceClick: (source: SourceReference) => void
}
```

## Data Models

### Frontend Data Models

```typescript
// API Response Types
interface UploadResponse {
  document_id: string
  filename: string
  chunks_created: number
  processing_time_ms: number
  status: 'success' | 'failure'
  message: string
  timestamp: string
}

interface RAGResponse {
  answer: string
  sources: SourceReference[]
  confidence: number
  query: string
  response_time_ms: number
  timestamp: string
}

interface SourceReference {
  document_id: string
  chunk_id: string
  relevance_score: number
  snippet: string
  source_file: string
  chunk_index: number
  headers: string[]
}

interface HealthResponse {
  status: 'healthy' | 'unhealthy'
  timestamp: string
  services: {
    qdrant: boolean
    azure_openai: boolean
  }
  uptime_seconds: number
}

// UI State Types
interface ToastMessage {
  id: string
  type: 'success' | 'error' | 'warning' | 'info'
  message: string
  duration?: number
}

interface ValidationError {
  field: string
  message: string
}
```

## Error Handling

### Error Handling Strategy

1. **Network Errors**: Implement retry logic with exponential backoff
2. **Validation Errors**: Display inline validation messages
3. **API Errors**: Parse backend error responses and show user-friendly messages
4. **Unexpected Errors**: Use error boundaries to catch and display fallback UI

### Error Types

```typescript
enum ErrorType {
  NETWORK_ERROR = 'network_error',
  VALIDATION_ERROR = 'validation_error',
  API_ERROR = 'api_error',
  UPLOAD_ERROR = 'upload_error',
  SEARCH_ERROR = 'search_error'
}

interface AppError {
  type: ErrorType
  message: string
  details?: any
  retryable: boolean
}
```

### Error Handling Components

```typescript
// ErrorBoundary.svelte
interface ErrorBoundaryProps {
  fallback?: ComponentType
  onError?: (error: Error) => void
}

// Toast.svelte for error notifications
interface ToastProps {
  messages: ToastMessage[]
  position: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left'
}
```

## Testing Strategy

### Unit Testing
- **Components**: Test component rendering, props, and user interactions
- **Services**: Test API calls, error handling, and data transformation
- **Stores**: Test state management and reactive updates
- **Utilities**: Test helper functions and validation logic

### Integration Testing
- **API Integration**: Test communication with backend endpoints
- **User Flows**: Test complete user journeys (upload → search → results)
- **Error Scenarios**: Test error handling and recovery

### E2E Testing
- **Critical Paths**: Upload document and perform search
- **Cross-browser**: Test on major browsers (Chrome, Firefox, Safari)
- **Responsive**: Test on different screen sizes

### Testing Tools
- **Vitest**: Unit and integration testing
- **Testing Library**: Component testing utilities
- **Playwright**: End-to-end testing
- **MSW**: API mocking for tests

## Technology Stack

### Core Technologies
- **Svelte 5**: Frontend framework with runes for reactivity
- **TypeScript**: Type safety and better developer experience
- **Vite**: Build tool and development server
- **SvelteUI**: Component library for consistent styling

### Additional Libraries
- **@svelteui/core**: Core UI components
- **@svelteui/dates**: Date picker components
- **@svelteui/notifications**: Toast notifications
- **@svelteui/modals**: Modal dialogs
- **lucide-svelte**: Icon library
- **zod**: Runtime type validation
- **@tanstack/svelte-query**: Data fetching and caching

### Development Tools
- **pnpm**: Package manager
- **ESLint**: Code linting
- **Prettier**: Code formatting
- **Husky**: Git hooks
- **lint-staged**: Pre-commit linting

## Responsive Design

### Breakpoints
```css
/* Mobile First Approach */
:root {
  --mobile: 320px;
  --tablet: 768px;
  --desktop: 1024px;
  --large: 1440px;
}
```

### Layout Strategy
- **Mobile (320px-767px)**: Single column, stacked navigation
- **Tablet (768px-1023px)**: Two column where appropriate, collapsible sidebar
- **Desktop (1024px+)**: Multi-column layouts, persistent navigation

### SvelteUI Theme Configuration
```typescript
const theme = {
  colorScheme: 'light',
  colors: {
    primary: ['#e3f2fd', '#bbdefb', '#90caf9', '#64b5f6', '#42a5f5', '#2196f3', '#1e88e5', '#1976d2', '#1565c0', '#0d47a1'],
    secondary: ['#f3e5f5', '#e1bee7', '#ce93d8', '#ba68c8', '#ab47bc', '#9c27b0', '#8e24aa', '#7b1fa2', '#6a1b9a', '#4a148c']
  },
  spacing: {
    xs: '0.5rem',
    sm: '0.75rem',
    md: '1rem',
    lg: '1.5rem',
    xl: '2rem'
  }
}
```

## Performance Considerations

### Optimization Strategies
1. **Code Splitting**: Lazy load pages and heavy components
2. **Bundle Analysis**: Monitor bundle size and optimize imports
3. **Image Optimization**: Compress and serve appropriate formats
4. **Caching**: Implement service worker for offline functionality
5. **Debouncing**: Debounce search inputs to reduce API calls

### Performance Metrics
- **First Contentful Paint (FCP)**: < 1.5s
- **Largest Contentful Paint (LCP)**: < 2.5s
- **Cumulative Layout Shift (CLS)**: < 0.1
- **First Input Delay (FID)**: < 100ms

## Accessibility

### WCAG 2.1 AA Compliance
- **Keyboard Navigation**: Full keyboard accessibility
- **Screen Reader Support**: Proper ARIA labels and semantic HTML
- **Color Contrast**: Minimum 4.5:1 ratio for normal text
- **Focus Management**: Visible focus indicators and logical tab order

### Accessibility Features
```typescript
// Accessibility utilities
interface A11yProps {
  'aria-label'?: string
  'aria-describedby'?: string
  'aria-expanded'?: boolean
  'aria-hidden'?: boolean
  role?: string
}
```

## Security Considerations

### Client-Side Security
1. **Input Validation**: Validate all user inputs before sending to backend
2. **XSS Prevention**: Sanitize any dynamic content
3. **CSRF Protection**: Include CSRF tokens in API requests
4. **Content Security Policy**: Implement strict CSP headers
5. **File Upload Security**: Validate file types and sizes

### API Security
```typescript
// API client with security headers
const apiClient = {
  baseURL: import.meta.env.VITE_API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
    'X-Requested-With': 'XMLHttpRequest'
  }
}
```

## Deployment Strategy

### Build Configuration
```typescript
// vite.config.ts
export default defineConfig({
  plugins: [sveltekit()],
  build: {
    target: 'es2020',
    minify: 'terser',
    sourcemap: true
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true
      }
    }
  }
})
```

### Environment Configuration
```bash
# .env.example
VITE_API_BASE_URL=http://localhost:8080
VITE_APP_NAME=RAG Document Search
VITE_MAX_FILE_SIZE=10485760
VITE_SUPPORTED_FILE_TYPES=.pdf
```

### Production Optimizations
1. **Static Asset Optimization**: Compress images and fonts
2. **CDN Integration**: Serve static assets from CDN
3. **Gzip Compression**: Enable server-side compression
4. **Browser Caching**: Set appropriate cache headers
5. **Progressive Web App**: Add service worker for offline support