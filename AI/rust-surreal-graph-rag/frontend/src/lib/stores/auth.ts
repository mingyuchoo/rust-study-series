import { writable, get } from 'svelte/store';
import { login as apiLogin, logout as apiLogout, me as apiMe } from '$lib/services/auth';
import type { LoginRequest, MeResponse } from '$lib/types/api';

// 토큰 저장/조회 유틸. localStorage 사용
const LS_KEY = 'rag_tokens_v1';

export type StoredTokens = {
  access_token: string;
  refresh_token: string;
  expires_in: number;
};

export function getStoredTokens(): StoredTokens | null {
  try {
    const raw = localStorage.getItem(LS_KEY);
    return raw ? (JSON.parse(raw) as StoredTokens) : null;
  } catch {
    return null;
  }
}

export function setStoredTokens(tokens: StoredTokens) {
  localStorage.setItem(LS_KEY, JSON.stringify(tokens));
}

export function clearStoredTokens() {
  localStorage.removeItem(LS_KEY);
}

// 인증 스토어
export const meStore = writable<MeResponse | null>(null);
export const isAuthenticated = writable(false);

// 앱 시작 시 토큰이 있으면 me 호출
export async function initAuth() {
  const tokens = getStoredTokens();
  if (tokens?.access_token) {
    try {
      const profile = await apiMe();
      meStore.set(profile);
      isAuthenticated.set(true);
    } catch {
      meStore.set(null);
      isAuthenticated.set(false);
    }
  }
}

export async function login(req: LoginRequest) {
  const res = await apiLogin(req);
  setStoredTokens({
    access_token: res.access_token,
    refresh_token: res.refresh_token,
    expires_in: Number(res.expires_in),
  });
  const profile = await apiMe();
  meStore.set(profile);
  isAuthenticated.set(true);
}

export async function logout() {
  try {
    await apiLogout();
  } catch {
    // ignore
  }
  clearStoredTokens();
  meStore.set(null);
  isAuthenticated.set(false);
}
