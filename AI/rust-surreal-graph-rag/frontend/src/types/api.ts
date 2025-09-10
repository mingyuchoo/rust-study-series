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
  // 청크 텍스트 내용
  content: string;
  // 선택적 메타데이터(JSON)
  metadata?: any;
};

export type IndexCreateRequest = {
  // 문서 식별자(옵션)
  document_id?: string | null;
  // 문서 제목(옵션)
  title?: string | null;
  // 분할된 청크 목록
  chunks: IndexChunkInput[];
};

export type IndexCreateResponse = {
  // 생성/사용된 문서 식별자
  document_id: string;
  // 인덱싱된 청크 개수
  chunks_indexed: number;
  // 전체 처리 시간(초)
  elapsed: number;
};

// 관리자 재인덱싱 요청/응답 타입
export type ReindexRequest = {
  // 재인덱싱할 PDF 파일 경로 목록(서버 경로)
  pdf_paths: string[];
  // 기존 데이터 정리 여부: true면 동일 source의 기존 데이터 삭제 후 재인덱싱
  clear_existing?: boolean;
};

export type ReindexItemResult = {
  // 입력 PDF 경로(에코)
  pdf_path: string;
  // 생성/사용된 문서 ID
  document_id?: string | null;
  // 인덱싱된 청크 개수
  chunks_indexed: number;
  // 오류 메시지(성공 시 undefined)
  error?: string | null;
};

export type ReindexResponse = {
  results: ReindexItemResult[];
  elapsed: number;
};

// 파일 업로드 응답
export type UploadResponse = {
  // 서버에 저장된 파일의 전체 경로
  path: string;
  // 저장된 파일 크기(바이트)
  size: number;
};
