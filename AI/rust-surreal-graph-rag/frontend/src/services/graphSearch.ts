import api from './api';
import { GraphSearchRequest, GraphSearchResponse } from '@/types/api';

// 그래프 검색 API
export async function graphSearch(req: GraphSearchRequest): Promise<GraphSearchResponse> {
  const { data } = await api.post<GraphSearchResponse>('/api/search/graph', req);
  return data;
}
