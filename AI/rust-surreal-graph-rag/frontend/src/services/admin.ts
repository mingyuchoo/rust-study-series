// 관리자용 API 호출 유틸리티
// - 재인덱싱: /api/reindex
// 모든 주석은 한국어로 작성합니다.

import api from './api';
import type { ReindexRequest, ReindexResponse, UploadResponse } from '@/types/api';

export async function reindexPdfs(payload: ReindexRequest): Promise<ReindexResponse> {
  const { data } = await api.post<ReindexResponse>('/api/reindex', payload);
  return data;
}

export async function uploadFile(file: File): Promise<UploadResponse> {
  const filename = encodeURIComponent(file.name || 'upload.bin');
  const { data } = await api.post<UploadResponse>(
    `/api/admin/upload?filename=${filename}`,
    file,
    { headers: { 'Content-Type': 'application/octet-stream' } },
  );
  return data;
}
