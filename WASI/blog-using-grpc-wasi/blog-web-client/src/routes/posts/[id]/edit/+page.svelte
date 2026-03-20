<script lang="ts">
	import { enhance } from '$app/forms';
	import { renderMarkdown } from '$lib/markdown';
	import { t, type Locale } from '$lib/i18n';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isLoading = false;
	let content = form?.content ?? data.post?.content ?? '';
	let showPreview = false;

	$: locale = (data.locale ?? 'ko') as Locale;
	$: preview = renderMarkdown(content);
</script>

<svelte:head>
	<title>{t(locale, 'editPost.title')} - Blog</title>
</svelte:head>

<div class="container">
	<h1>{t(locale, 'editPost.title')}</h1>

	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
		<a href="/">&larr; {t(locale, 'post.backToList')}</a>
	{:else if data.post}
		{#if form?.error}
			<div class="alert alert-error">{form.error}</div>
		{/if}

		<div class="card">
			<form
				method="POST"
				action="?/updatePost"
				use:enhance={() => {
					isLoading = true;
					return async ({ update }) => { await update(); isLoading = false; };
				}}
			>
				<div class="form-group">
					<label for="title">{t(locale, 'newPost.titleLabel')}</label>
					<input
						id="title"
						name="title"
						type="text"
						maxlength="200"
						required
						disabled={isLoading}
						value={form?.title ?? data.post.title}
					/>
				</div>
				<div class="form-group">
					<div class="editor-header">
						<label for="content">{t(locale, 'newPost.content')}</label>
						<div class="editor-tabs">
							<button type="button" class="editor-tab" class:active={!showPreview} on:click={() => showPreview = false}>
								{t(locale, 'newPost.write')}
							</button>
							<button type="button" class="editor-tab" class:active={showPreview} on:click={() => showPreview = true}>
								{t(locale, 'newPost.preview')}
							</button>
						</div>
					</div>
					{#if showPreview}
						<div class="preview-box markdown-body">
							{#if content.trim()}
								{@html preview}
							{:else}
								<p class="meta">{t(locale, 'newPost.noPreview')}</p>
							{/if}
						</div>
					{:else}
						<textarea
							id="content"
							name="content"
							rows="16"
							required
							disabled={isLoading}
							bind:value={content}
						></textarea>
					{/if}
					<input type="hidden" name="content" value={content} />
					<span class="meta">{t(locale, 'newPost.markdownHelp')}</span>
				</div>
				<div class="form-group">
					<label class="checkbox-label">
						<input type="checkbox" name="visibility" disabled={isLoading} checked={data.post.visibility === 'public'} />
						<span>{t(locale, 'newPost.public')}</span>
					</label>
				</div>
				<div style="display: flex; gap: 0.75rem; justify-content: flex-end">
					<a href="/posts/{data.post.id}" class="btn btn-outline">{t(locale, 'newPost.cancel')}</a>
					<button type="submit" class="btn" disabled={isLoading}>
						{isLoading ? t(locale, 'editPost.submitting') : t(locale, 'editPost.submit')}
					</button>
				</div>
			</form>
		</div>
	{/if}
</div>

<style>
	.editor-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.375rem;
	}
	.editor-header label {
		margin-bottom: 0;
	}
	.editor-tabs {
		display: flex;
		gap: 0.25rem;
	}
	.editor-tab {
		padding: 0.25rem 0.625rem;
		font-size: 0.75rem;
		border: 1px solid var(--border);
		border-radius: 0.375rem;
		background: transparent;
		color: var(--text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}
	.editor-tab.active {
		background: var(--accent);
		border-color: var(--accent);
		color: white;
	}
	.preview-box {
		width: 100%;
		min-height: 384px;
		background: var(--bg-input);
		border: 1px solid var(--border);
		border-radius: 0.5rem;
		padding: 0.625rem 0.875rem;
	}
</style>
