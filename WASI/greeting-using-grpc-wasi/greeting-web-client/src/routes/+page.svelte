<script lang="ts">
	import '../app.css';
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	let isLoading = false;
	let greetHistory: string[] = [];

	$: if (form && 'message' in form && form.message) {
		greetHistory = [form.message as string, ...greetHistory];
	}
</script>

<svelte:head>
	<title>Greeting gRPC WASI Web Client</title>
</svelte:head>

<main>
	<header>
		<div class="header-icon">⚡</div>
		<h1>Greeting gRPC WASI</h1>
		<p class="subtitle">Web Client — TypeScript · Bun · Svelte</p>
	</header>

	{#if data.connectionError}
		<div class="alert alert-error" role="alert" aria-live="assertive">
			<strong>Connection Error</strong>
			<span>{data.connectionError}</span>
			<small>gRPC server at <code>localhost:50051</code> is not reachable. Run <code>make run-server</code> first.</small>
		</div>
	{:else}
		<div class="version-card">
			<span class="version-label">WASI Component Version</span>
			<code class="version-value">{data.version}</code>
		</div>
	{/if}

	<section class="card">
		<h2>Send a Greeting</h2>
		<form
			method="POST"
			action="?/greet"
			use:enhance={() => {
				isLoading = true;
				return async ({ update }) => {
					await update();
					isLoading = false;
				};
			}}
		>
			<div class="input-row">
				<input
					type="text"
					name="name"
					placeholder="Enter a name..."
					aria-label="Name to greet"
					autocomplete="off"
					required
					disabled={isLoading}
				/>
				<button
					type="submit"
					aria-label={isLoading ? 'Sending greeting…' : 'Send greeting'}
					disabled={isLoading || !!data.connectionError}
				>
					{#if isLoading}
						<span class="spinner" aria-hidden="true"></span> Sending…
					{:else}
						Greet
					{/if}
				</button>
			</div>
		</form>

		{#if form && 'error' in form && form.error}
			<p class="form-error" role="alert" aria-live="polite">{form.error}</p>
		{/if}
	</section>

	{#if greetHistory.length > 0}
		<section class="card">
			<h2>Response History</h2>
			<ul class="history-list">
				{#each greetHistory as message, i}
					<li class="history-item" style="animation-delay: {i * 0.05}s">
						<span class="history-icon">💬</span>
						{message}
					</li>
				{/each}
			</ul>
		</section>
	{/if}
</main>

<style>
	main {
		max-width: 640px;
		margin: 0 auto;
		padding: 2rem 1.5rem 4rem;
	}

	header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.header-icon {
		font-size: 3rem;
		line-height: 1;
	}

	h1 {
		font-size: 1.875rem;
		font-weight: 700;
		margin: 0.5rem 0 0.25rem;
		background: linear-gradient(135deg, #38bdf8, #818cf8);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.subtitle {
		color: #64748b;
		font-size: 0.875rem;
		margin: 0;
	}

	.alert {
		border-radius: 0.75rem;
		padding: 1rem 1.25rem;
		margin-bottom: 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.alert-error {
		background: rgba(239, 68, 68, 0.1);
		border: 1px solid rgba(239, 68, 68, 0.3);
		color: #fca5a5;
	}

	.alert-error strong {
		color: #f87171;
		font-weight: 600;
	}

	.alert-error small {
		color: #94a3b8;
		font-size: 0.8rem;
	}

	.alert-error code {
		background: rgba(255, 255, 255, 0.1);
		padding: 0.1rem 0.3rem;
		border-radius: 0.25rem;
		font-size: 0.8rem;
	}

	.version-card {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		background: rgba(56, 189, 248, 0.08);
		border: 1px solid rgba(56, 189, 248, 0.2);
		border-radius: 0.75rem;
		padding: 0.875rem 1.25rem;
		margin-bottom: 1.5rem;
	}

	.version-label {
		color: #64748b;
		font-size: 0.8rem;
		white-space: nowrap;
	}

	.version-value {
		color: #38bdf8;
		font-size: 0.875rem;
		font-family: 'Fira Code', 'Cascadia Code', monospace;
	}

	.card {
		background: #1e293b;
		border: 1px solid #334155;
		border-radius: 0.75rem;
		padding: 1.5rem;
		margin-bottom: 1.5rem;
	}

	h2 {
		font-size: 1rem;
		font-weight: 600;
		color: #94a3b8;
		margin: 0 0 1rem;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.input-row {
		display: flex;
		gap: 0.75rem;
	}

	input {
		flex: 1;
		background: #0f172a;
		border: 1px solid #334155;
		border-radius: 0.5rem;
		padding: 0.625rem 0.875rem;
		color: #e2e8f0;
		font-size: 0.9375rem;
		outline: none;
		transition: border-color 0.15s;
	}

	input:focus {
		border-color: #38bdf8;
	}

	input::placeholder {
		color: #475569;
	}

	input:disabled {
		opacity: 0.5;
	}

	button {
		background: linear-gradient(135deg, #0ea5e9, #6366f1);
		color: white;
		border: none;
		border-radius: 0.5rem;
		padding: 0.625rem 1.25rem;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		transition: opacity 0.15s;
		white-space: nowrap;
	}

	button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.spinner {
		width: 0.875rem;
		height: 0.875rem;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.6s linear infinite;
		display: inline-block;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.form-error {
		color: #f87171;
		font-size: 0.875rem;
		margin: 0.75rem 0 0;
	}

	.history-list {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		flex-direction: column;
		gap: 0.625rem;
	}

	.history-item {
		display: flex;
		align-items: flex-start;
		gap: 0.625rem;
		background: #0f172a;
		border: 1px solid #1e293b;
		border-radius: 0.5rem;
		padding: 0.75rem 1rem;
		font-size: 0.9rem;
		color: #cbd5e1;
		animation: slideIn 0.2s ease-out both;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-6px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	.history-icon {
		flex-shrink: 0;
	}
</style>
