import { apiPost } from './api';
import type { VectorSearchRequest, VectorSearchResponse } from '$lib/types/api';

// 벡터 검색 API
export async function vectorSearch(req: VectorSearchRequest): Promise<VectorSearchResponse> {
  return apiPost<VectorSearchResponse>('/api/search/vector', req);
}
