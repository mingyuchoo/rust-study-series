import api from './api';
import { VectorSearchRequest, VectorSearchResponse } from '@/types/api';

// 한국어 주석: 벡터 검색 API
export async function vectorSearch(req: VectorSearchRequest): Promise<VectorSearchResponse> {
  const { data } = await api.post<VectorSearchResponse>('/api/search/vector', req);
  return data;
}
