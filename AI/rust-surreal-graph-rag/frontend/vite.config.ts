import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// 한국어 주석: Vite 설정 파일. React 플러그인과 경로 별칭 설정을 포함한다.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true,
  },
  preview: {
    port: 5173,
    strictPort: true,
  },
  resolve: {
    alias: {
      '@': '/src',
    },
  },
});
