// 인덱싱 생성 API 서비스
// 모든 주석은 한국어로 작성합니다.

import api from './api';
import { IndexCreateRequest, IndexCreateResponse } from '@/types/api';

// 인덱싱 생성 호출
export async function createIndexing(req: IndexCreateRequest): Promise<IndexCreateResponse> {
  const { data } = await api.post<IndexCreateResponse>('/api/index/create', req);
  return data;
}
