import axios, { AxiosError, AxiosInstance, AxiosRequestConfig } from 'axios';
import { getStoredTokens, setStoredTokens, clearStoredTokens } from '@/store/auth';

// Axios 인스턴스 생성 및 인터셉터 설정
const apiBaseURL = import.meta.env.VITE_API_BASE_URL || '';

const api: AxiosInstance = axios.create({
  baseURL: apiBaseURL,
  headers: {
    'Content-Type': 'application/json',
  },
  withCredentials: true,
});

// 요청 인터셉터: Access Token 자동 첨부
api.interceptors.request.use((config) => {
  const tokens = getStoredTokens();
  if (tokens?.access_token) {
    config.headers = config.headers || {};
    (config.headers as any)['Authorization'] = `Bearer ${tokens.access_token}`;
  }
  return config;
});

let isRefreshing = false;
let pendingQueue: Array<{
  resolve: (value?: any) => void;
  reject: (reason?: any) => void;
  config: AxiosRequestConfig;
}> = [];

function subscribeTokenRefresh(cb: (token: string) => void) {
  pendingQueue.forEach(({ config, resolve }) => {
    config.headers = config.headers || {};
    const tokens = getStoredTokens();
    if (tokens?.access_token) {
      (config.headers as any)['Authorization'] = `Bearer ${tokens.access_token}`;
    }
    resolve(api(config));
  });
  pendingQueue = [];
}

// 응답 인터셉터: 401 발생 시 토큰 재발급 로직 수행
api.interceptors.response.use(
  (res) => res,
  async (error: AxiosError) => {
    const originalConfig = error.config!;

    if (error.response?.status === 401 && !originalConfig._retry) {
      originalConfig._retry = true as any;

      if (!isRefreshing) {
        isRefreshing = true;
        try {
          const refreshed = await axios.post(
            `${apiBaseURL}/api/auth/refresh`,
            {},
            { headers: { 'Content-Type': 'application/json' } },
          );
          const { access_token, expires_in } = refreshed.data as {
            access_token: string;
            expires_in: number;
          };
          const tokens = getStoredTokens();
          setStoredTokens({
            access_token,
            refresh_token: tokens?.refresh_token ?? '',
            expires_in,
          });
          isRefreshing = false;
          subscribeTokenRefresh((access_token) => access_token);
          return api(originalConfig);
        } catch (e) {
          isRefreshing = false;
          clearStoredTokens();
          return Promise.reject(e);
        }
      }

      // 이미 갱신중인 경우 대기열에 추가
      return new Promise((resolve, reject) => {
        pendingQueue.push({ resolve, reject, config: originalConfig });
      });
    }

    return Promise.reject(error);
  },
);

export default api;
