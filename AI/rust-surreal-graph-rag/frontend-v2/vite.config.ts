import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// Vite 설정 파일. SvelteKit 플러그인과 프록시 설정을 포함한다.
export default defineConfig({
  plugins: [sveltekit()],
  server: {
    port: 5174,
    strictPort: true,
    // 개발 중 프록시 설정 - 프론트엔드(5174)에서의 "/api" 요청을 백엔드(4000)로 전달
    proxy: {
      '/api': {
        target: 'http://localhost:4000',
        changeOrigin: true,
        secure: false,
      },
    },
  },
  preview: {
    port: 5174,
    strictPort: true,
  },
});
