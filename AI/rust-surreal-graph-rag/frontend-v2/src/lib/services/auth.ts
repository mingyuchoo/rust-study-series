import { apiGet, apiPost } from './api';
import type { LoginRequest, LoginResponse, MeResponse, MessageResponse } from '$lib/types/api';

// 인증 관련 API 함수
export async function login(req: LoginRequest): Promise<LoginResponse> {
  return apiPost<LoginResponse>('/api/auth/login', req);
}

export async function logout(): Promise<MessageResponse> {
  return apiPost<MessageResponse>('/api/auth/logout');
}

export async function me(): Promise<MeResponse> {
  return apiGet<MeResponse>('/api/auth/me');
}
