import api from './api';
import { ChatAskRequest, ChatAskResponse } from '@/types/api';

// 한국어 주석: 통합 질의응답 API
export async function chatAsk(req: ChatAskRequest): Promise<ChatAskResponse> {
  const { data } = await api.post<ChatAskResponse>('/api/chat/ask', req);
  return data;
}
