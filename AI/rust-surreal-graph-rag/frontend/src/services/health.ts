import api from './api';
import { HealthResponse } from '@/types/api';

// 헬스 체크 API
export async function getHealth(): Promise<HealthResponse> {
  const { data } = await api.get<HealthResponse>('/api/health');
  return data;
}
