# Type Definitions

This directory contains comprehensive type definitions for the Svelte RAG frontend application.

## Structure

### Core Type Files

- **`api.ts`** - API request/response types and interfaces
- **`state.ts`** - Application state management types
- **`errors.ts`** - Error handling types and enums
- **`components.ts`** - Component props and event types
- **`utils.ts`** - Utility types and type helpers
- **`constants.ts`** - Application constants and default values

### Validation

- **`schemas/validation.ts`** - Zod validation schemas for runtime type checking

### Documentation

- **`examples.ts`** - Usage examples for the type system
- **`index.ts`** - Central export point for all types

## Key Features

### 1. API Types

Comprehensive type definitions for all backend API interactions:

```typescript
import { type QueryConfig, type RAGResponse } from '$lib/types';

const config: QueryConfig = {
  max_chunks: 10,
  similarity_threshold: 0.8,
  temperature: 0.3
};
```

### 2. State Management

Type-safe state management for Svelte stores:

```typescript
import { type SearchState, type UploadState } from '$lib/types';

const searchState: SearchState = {
  query: '',
  results: null,
  isSearching: false,
  searchConfig: DEFAULT_QUERY_CONFIG,
  searchHistory: []
};
```

### 3. Error Handling

Structured error handling with type safety:

```typescript
import { type AppError, ErrorTypeValues, ErrorSeverityValues } from '$lib/types';

const error: AppError = {
  type: ErrorTypeValues.NETWORK_ERROR,
  message: 'Connection failed',
  retryable: true,
  severity: ErrorSeverityValues.MEDIUM,
  timestamp: new Date()
};
```

### 4. Component Props

Type-safe component interfaces:

```typescript
import { type SearchFormProps } from '$lib/types';

const props: SearchFormProps = {
  onSubmit: (query, config) => { /* handle submit */ },
  disabled: false,
  showAdvanced: true
};
```

### 5. Runtime Validation

Zod schemas for runtime type validation:

```typescript
import { SearchQuerySchema } from '$lib/types';

const result = SearchQuerySchema.safeParse({
  question: 'What is the main topic?',
  config: { max_chunks: 5 }
});

if (result.success) {
  // Type-safe data access
  console.log(result.data.question);
}
```

## Usage Guidelines

### Importing Types

Always import types from the main index file:

```typescript
import { 
  type QueryConfig,
  type SearchState,
  type AppError,
  SearchQuerySchema,
  DEFAULT_QUERY_CONFIG
} from '$lib/types';
```

### Type Safety

- Use branded types for IDs to prevent mixing different ID types
- Leverage utility types for common patterns (Partial, Required, etc.)
- Use validation schemas for runtime type checking
- Prefix unused parameters with underscore to avoid linting errors

### Error Handling

- Use the structured error types for consistent error handling
- Leverage the error recovery system for user-friendly error messages
- Use appropriate error severity levels

### Constants

- Use the predefined constants for consistent values across the app
- Leverage default configurations for sensible defaults
- Use validation limits to ensure data integrity

## Best Practices

1. **Type First**: Define types before implementing functionality
2. **Validation**: Use Zod schemas for runtime validation
3. **Consistency**: Use the provided constants and defaults
4. **Documentation**: Document complex types with JSDoc comments
5. **Testing**: Use the example file patterns for testing type usage

## File Organization

```
types/
├── api.ts              # API types
├── state.ts            # State management types
├── errors.ts           # Error handling types
├── components.ts       # Component types
├── utils.ts            # Utility types
├── constants.ts        # Constants and defaults
├── schemas/
│   └── validation.ts   # Zod validation schemas
├── examples.ts         # Usage examples
├── index.ts           # Main export file
└── README.md          # This file
```

This type system provides a solid foundation for building a type-safe, maintainable Svelte application with comprehensive error handling and validation.