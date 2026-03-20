import { apiPost } from './api';
import type { GraphSearchRequest, GraphSearchResponse } from '$lib/types/api';

// 그래프 검색 API
export async function graphSearch(req: GraphSearchRequest): Promise<GraphSearchResponse> {
  return apiPost<GraphSearchResponse>('/api/search/graph', req);
}
