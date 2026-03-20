import { apiGet } from './api';
import type { HealthResponse } from '$lib/types/api';

// 헬스 체크 API
export async function getHealth(): Promise<HealthResponse> {
  return apiGet<HealthResponse>('/api/health');
}
