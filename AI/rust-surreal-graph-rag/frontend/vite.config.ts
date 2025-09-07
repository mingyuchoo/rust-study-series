import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// Vite 설정 파일. React 플러그인과 경로 별칭 설정을 포함한다.
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true,
    // 개발 중 프록시 설정 - 프론트엔드(5173)에서의 "/api" 요청을 백엔드(4000)로 전달
    proxy: {
      '/api': {
        target: 'http://localhost:4000',
        changeOrigin: true,
        secure: false,
      },
    },
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
