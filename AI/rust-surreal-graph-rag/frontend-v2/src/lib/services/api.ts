import { getStoredTokens, setStoredTokens, clearStoredTokens } from '$lib/stores/auth';

// fetch 래퍼 - Axios 인터셉터와 동일한 역할 수행
const apiBaseURL = '';

let isRefreshing = false;
let pendingQueue: Array<{
  resolve: (value: any) => void;
  reject: (reason?: any) => void;
}> = [];

function processPendingQueue(error: any = null) {
  pendingQueue.forEach(({ resolve, reject }) => {
    if (error) {
      reject(error);
    } else {
      resolve(null);
    }
  });
  pendingQueue = [];
}

// 인증 헤더가 포함된 fetch 요청
async function apiFetch(url: string, options: RequestInit = {}): Promise<Response> {
  const tokens = getStoredTokens();
  const headers = new Headers(options.headers);

  if (!headers.has('Content-Type') && !(options.body instanceof File)) {
    headers.set('Content-Type', 'application/json');
  }

  if (tokens?.access_token) {
    headers.set('Authorization', `Bearer ${tokens.access_token}`);
  }

  const response = await fetch(`${apiBaseURL}${url}`, {
    ...options,
    headers,
    credentials: 'include',
  });

  // 401 응답 시 토큰 갱신
  if (response.status === 401) {
    if (!isRefreshing) {
      isRefreshing = true;
      try {
        const refreshRes = await fetch(`${apiBaseURL}/api/auth/refresh`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
        });

        if (!refreshRes.ok) throw new Error('토큰 갱신 실패');

        const { access_token, expires_in } = await refreshRes.json();
        const currentTokens = getStoredTokens();
        setStoredTokens({
          access_token,
          refresh_token: currentTokens?.refresh_token ?? '',
          expires_in,
        });

        isRefreshing = false;
        processPendingQueue();

        // 원래 요청 재시도
        headers.set('Authorization', `Bearer ${access_token}`);
        return fetch(`${apiBaseURL}${url}`, {
          ...options,
          headers,
          credentials: 'include',
        });
      } catch (e) {
        isRefreshing = false;
        clearStoredTokens();
        processPendingQueue(e);
        throw e;
      }
    }

    // 이미 갱신 중인 경우 대기열에 추가
    return new Promise((resolve, reject) => {
      pendingQueue.push({
        resolve: () => {
          const newTokens = getStoredTokens();
          if (newTokens?.access_token) {
            headers.set('Authorization', `Bearer ${newTokens.access_token}`);
          }
          resolve(
            fetch(`${apiBaseURL}${url}`, {
              ...options,
              headers,
              credentials: 'include',
            }),
          );
        },
        reject,
      });
    });
  }

  return response;
}

// JSON GET 요청
export async function apiGet<T>(url: string): Promise<T> {
  const res = await apiFetch(url);
  if (!res.ok) throw new Error(`요청 실패: ${res.status}`);
  return res.json();
}

// JSON POST 요청
export async function apiPost<T>(url: string, body?: any, options?: RequestInit): Promise<T> {
  const res = await apiFetch(url, {
    method: 'POST',
    body: body instanceof File ? body : body !== undefined ? JSON.stringify(body) : undefined,
    ...options,
  });
  if (!res.ok) throw new Error(`요청 실패: ${res.status}`);
  return res.json();
}
