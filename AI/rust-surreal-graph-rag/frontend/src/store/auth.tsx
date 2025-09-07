import React, { createContext, useContext, useEffect, useMemo, useState } from 'react';
import { login as apiLogin, logout as apiLogout, me as apiMe } from '@/services/auth';
import { LoginRequest, LoginResponse, MeResponse } from '@/types/api';

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

// 인증 컨텍스트
interface AuthContextState {
  isAuthenticated: boolean;
  me: MeResponse | null;
  login: (req: LoginRequest) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextState | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [meState, setMeState] = useState<MeResponse | null>(null);

  useEffect(() => {
    // 앱 시작 시 토큰이 있으면 me 호출
    const tokens = getStoredTokens();
    if (tokens?.access_token) {
      apiMe()
        .then(setMeState)
        .catch(() => setMeState(null));
    }
  }, []);

  const value = useMemo<AuthContextState>(
    () => ({
      isAuthenticated: !!meState,
      me: meState,
      login: async (req: LoginRequest) => {
        const res: LoginResponse = await apiLogin(req);
        setStoredTokens({
          access_token: res.access_token,
          refresh_token: res.refresh_token,
          expires_in: Number(res.expires_in),
        });
        const profile = await apiMe();
        setMeState(profile);
      },
      logout: async () => {
        try {
          await apiLogout();
        } catch {
          // ignore
        }
        clearStoredTokens();
        setMeState(null);
      },
    }),
    [meState],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export function useAuth(): AuthContextState {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('AuthProvider 하위에서만 useAuth를 사용할 수 있습니다.');
  return ctx;
}
