import api from './api';
import { LoginRequest, LoginResponse, MeResponse, MessageResponse } from '@/types/api';

// 한국어 주석: 인증 관련 API 함수
export async function login(req: LoginRequest): Promise<LoginResponse> {
  const { data } = await api.post<LoginResponse>('/api/auth/login', req);
  return data;
}

export async function logout(): Promise<MessageResponse> {
  const { data } = await api.post<MessageResponse>('/api/auth/logout');
  return data;
}

export async function me(): Promise<MeResponse> {
  const { data } = await api.get<MeResponse>('/api/auth/me');
  return data;
}
