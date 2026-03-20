// OpenAPI 명세 기반 타입 정의

export type LoginRequest = {
  email: string;
  password: string;
};

export type LoginResponse = {
  access_token: string;
  refresh_token: string;
  user_id: string;
  expires_in: number;
};

export type MessageResponse = {
  message: string;
};

export type MeResponse = {
  email: string;
};

export type RefreshResponse = {
  access_token: string;
  expires_in: number;
};

export type ChatAskRequest = {
  query: string;
  context?: any;
  conversation_id?: string | null;
  options?: any;
};

export type SourceItem = {
  type: string;
  content: string;
  score: number;
  metadata: any;
};

export type GraphPathItem = {
  path: string;
  nodes: any;
  relationships: any;
};

export type ChatAskResponse = {
  conversation_id?: string | null;
  response: string;
  sources: SourceItem[];
  graph_paths: GraphPathItem[];
  query_time: number;
  tokens_used: number;
};

export type VectorSearchItem = {
  id: string;
  content: string;
  score: number;
  metadata: any;
};

export type VectorSearchRequest = {
  query: string;
  filters?: any;
  threshold?: number;
  top_k?: number;
};

export type VectorSearchResponse = {
  results: VectorSearchItem[];
  total: number;
  query_time: number;
};

// 그래프 검색
export type GraphSearchRequest = {
  query: string;
  top_k?: number;
  max_hops?: number;
};

export type GraphSearchResponse = {
  paths: GraphPathItem[];
  total: number;
  query_time: number;
};

export type HealthResponse = {
  status: string;
  timestamp: string;
  services: any;
  version: string;
};

// 인덱싱 생성
export type IndexChunkInput = {
  content: string;
  metadata?: any;
};

export type IndexCreateRequest = {
  document_id?: string | null;
  title?: string | null;
  chunks: IndexChunkInput[];
};

export type IndexCreateResponse = {
  document_id: string;
  chunks_indexed: number;
  elapsed: number;
};

// 관리자 재인덱싱 요청/응답 타입
export type ReindexRequest = {
  pdf_paths: string[];
  clear_existing?: boolean;
};

export type ReindexItemResult = {
  pdf_path: string;
  document_id?: string | null;
  chunks_indexed: number;
  error?: string | null;
};

export type ReindexResponse = {
  results: ReindexItemResult[];
  elapsed: number;
};

// 파일 업로드 응답
export type UploadResponse = {
  path: string;
  size: number;
};
