<script lang="ts">
	import { enhance } from '$app/forms';
	import { renderMarkdown } from '$lib/markdown';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isLoading = false;
	let content = '';
	let showPreview = false;

	$: preview = renderMarkdown(content);
</script>

<svelte:head>
	<title>새 글 작성 - Blog</title>
</svelte:head>

<div class="container">
	<h1>새 글 작성</h1>

	{#if form?.error}
		<div class="alert alert-error">{form.error}</div>
	{/if}

	<div class="card">
		<form
			method="POST"
			use:enhance={() => {
				isLoading = true;
				return async ({ update }) => { await update(); isLoading = false; };
			}}
		>
			<div class="form-group">
				<label for="title">제목</label>
				<input id="title" name="title" type="text" maxlength="200" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<div class="editor-header">
					<label for="content">내용</label>
					<div class="editor-tabs">
						<button type="button" class="editor-tab" class:active={!showPreview} on:click={() => showPreview = false}>
							작성
						</button>
						<button type="button" class="editor-tab" class:active={showPreview} on:click={() => showPreview = true}>
							미리보기
						</button>
					</div>
				</div>
				{#if showPreview}
					<div class="preview-box markdown-body">
						{#if content.trim()}
							{@html preview}
						{:else}
							<p class="meta">미리보기할 내용이 없습니다.</p>
						{/if}
					</div>
				{:else}
					<textarea
						id="content"
						name="content"
						rows="16"
						required
						disabled={isLoading}
						placeholder="Markdown 형식으로 작성할 수 있습니다.&#10;&#10;# 제목&#10;## 소제목&#10;**굵게**, *기울임*, `코드`&#10;- 목록&#10;```&#10;코드 블록&#10;```"
						bind:value={content}
					></textarea>
				{/if}
				<input type="hidden" name="content" value={content} />
				<span class="meta">Markdown 문법을 지원합니다.</span>
			</div>
			<div class="form-group">
				<label class="checkbox-label">
					<input type="checkbox" name="visibility" disabled={isLoading} />
					<span>공개</span>
				</label>
			</div>
			<div style="display: flex; gap: 0.75rem; justify-content: flex-end">
				<a href="/" class="btn btn-outline">취소</a>
				<button type="submit" class="btn" disabled={isLoading}>
					{isLoading ? '게시 중...' : '게시하기'}
				</button>
			</div>
		</form>
	</div>
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
