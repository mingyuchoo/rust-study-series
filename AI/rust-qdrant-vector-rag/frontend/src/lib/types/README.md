# 타입 정의

Svelte RAG 프론트엔드 애플리케이션의 종합 타입 정의 디렉터리입니다.

## 구조

### 핵심 타입 파일

- **`api.ts`** - API 요청/응답 타입 및 인터페이스
- **`state.ts`** - 애플리케이션 상태 관리 타입
- **`errors.ts`** - 에러 처리 타입 및 열거형
- **`components.ts`** - 컴포넌트 props 및 이벤트 타입
- **`utils.ts`** - 유틸리티 타입 및 타입 헬퍼
- **`constants.ts`** - 애플리케이션 상수 및 기본값

### 유효성 검사

- **`schemas/validation.ts`** - Zod 기반 런타임 타입 검증 스키마

### 문서

- **`examples.ts`** - 타입 시스템 사용 예제
- **`index.ts`** - 모든 타입의 중앙 내보내기 파일

## 주요 기능

### 1. API 타입

백엔드 API 상호작용을 위한 종합 타입 정의:

```typescript
import { type QueryConfig, type RAGResponse } from '$lib/types';

const config: QueryConfig = {
  max_chunks: 10,
  similarity_threshold: 0.8,
  temperature: 0.3
};
```

### 2. 상태 관리

Svelte 스토어를 위한 타입 안전 상태 관리:

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

### 3. 에러 처리

타입 안전성을 갖춘 구조화된 에러 처리:

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

### 4. 컴포넌트 Props

타입 안전 컴포넌트 인터페이스:

```typescript
import { type SearchFormProps } from '$lib/types';

const props: SearchFormProps = {
  onSubmit: (query, config) => { /* handle submit */ },
  disabled: false,
  showAdvanced: true
};
```

### 5. 런타임 유효성 검사

Zod 스키마를 활용한 런타임 타입 검증:

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

## 사용 가이드

### 타입 가져오기

항상 메인 인덱스 파일에서 타입을 가져오세요:

```typescript
import { 
  type QueryConfig,
  type SearchState,
  type AppError,
  SearchQuerySchema,
  DEFAULT_QUERY_CONFIG
} from '$lib/types';
```

### 타입 안전성

- 브랜디드 타입으로 ID 혼용 방지
- 일반적인 패턴에 유틸리티 타입 활용 (Partial, Required 등)
- 런타임 타입 검증에 유효성 검사 스키마 사용
- 사용하지 않는 매개변수는 언더스코어 접두사로 린트 에러 방지

### 에러 처리

- 일관된 에러 처리를 위한 구조화된 에러 타입 사용
- 사용자 친화적 에러 메시지를 위한 에러 복구 시스템 활용
- 적절한 에러 심각도 수준 사용

### 상수

- 앱 전체에서 일관된 값을 위해 사전 정의된 상수 사용
- 합리적인 기본값을 위한 기본 설정 활용
- 데이터 무결성을 위한 유효성 검사 제한값 사용

## 모범 사례

1. **타입 우선**: 기능 구현 전 타입을 먼저 정의
2. **유효성 검사**: 런타임 검증에 Zod 스키마 사용
3. **일관성**: 제공된 상수와 기본값 사용
4. **문서화**: 복잡한 타입은 JSDoc 주석으로 문서화
5. **테스트**: 예제 파일 패턴으로 타입 사용 테스트

## 파일 구성

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

이 타입 시스템은 종합적인 에러 처리와 유효성 검증을 갖춘 타입 안전하고 유지보수 가능한 Svelte 애플리케이션을 구축하기 위한 탄탄한 기반을 제공합니다.