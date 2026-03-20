import { apiPost } from './api';
import type { ReindexRequest, ReindexResponse, UploadResponse } from '$lib/types/api';

// 관리자용 재인덱싱 API
export async function reindexPdfs(payload: ReindexRequest): Promise<ReindexResponse> {
  return apiPost<ReindexResponse>('/api/reindex', payload);
}

export async function uploadFile(file: File): Promise<UploadResponse> {
  const filename = encodeURIComponent(file.name || 'upload.bin');
  return apiPost<UploadResponse>(`/api/reindex/upload?filename=${filename}`, file, {
    headers: { 'Content-Type': 'application/octet-stream' },
  });
}
