/// <reference types="vite/client" />

// Vite 환경 변수 타입 선언
interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
