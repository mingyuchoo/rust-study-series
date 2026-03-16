import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import type { Plugin } from 'vite';

// Suppress Svelte 5 API missing-export warnings when using Svelte 4
// (@sveltejs/kit 2.x imports untrack/fork/settled which don't exist in Svelte 4)
function suppressSvelteMissingExports(): Plugin {
	return {
		name: 'suppress-svelte-missing-exports',
		options(opts) {
			const original = opts.onwarn;
			opts.onwarn = (warning, warn) => {
				if (warning.code === 'MISSING_EXPORT' && warning.exporter?.includes('svelte')) return;
				if (original) original(warning, warn);
				else warn(warning);
			};
			return opts;
		}
	};
}

export default defineConfig({
	plugins: [sveltekit(), suppressSvelteMissingExports()],
	ssr: {
		// Keep gRPC modules external — do not bundle them.
		external: ['@grpc/grpc-js', '@grpc/proto-loader', 'node:path']
	}
});
