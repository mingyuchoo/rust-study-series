import { apiPost } from './api';
import type { ChatAskRequest, ChatAskResponse } from '$lib/types/api';

// 통합 질의응답 API
export async function chatAsk(req: ChatAskRequest): Promise<ChatAskResponse> {
  return apiPost<ChatAskResponse>('/api/chat/ask', req);
}
