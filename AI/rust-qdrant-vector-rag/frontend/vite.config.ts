import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        rewrite: path => path.replace(/^\/api/, ''),
      },
    },
  },
  build: {
    target: 'es2020',
    minify: 'terser',
    sourcemap: true,
  },
  define: {
    // Make environment variables available at build time
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
  },
});
