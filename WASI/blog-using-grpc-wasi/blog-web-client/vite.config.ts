import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	ssr: {
		// Keep gRPC modules external — do not bundle them.
		external: ['@grpc/grpc-js', '@grpc/proto-loader', 'node:path']
	}
});
