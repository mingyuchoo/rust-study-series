// 한국어 주석: OpenAPI 명세 기반 타입 정의

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

export type HealthResponse = {
  status: string;
  timestamp: string;
  services: any;
  version: string;
};
