<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isLoading = false;
</script>

<svelte:head>
	<title>글 수정 - Blog</title>
</svelte:head>

<div class="container">
	<h1>글 수정</h1>

	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
		<a href="/">&larr; 목록으로</a>
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
					<label for="title">제목</label>
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
					<label for="content">내용</label>
					<textarea
						id="content"
						name="content"
						rows="12"
						required
						disabled={isLoading}
					>{form?.content ?? data.post.content}</textarea>
				</div>
				<div style="display: flex; gap: 0.75rem; justify-content: flex-end">
					<a href="/posts/{data.post.id}" class="btn btn-outline">취소</a>
					<button type="submit" class="btn" disabled={isLoading}>
						{isLoading ? '수정 중...' : '수정하기'}
					</button>
				</div>
			</form>
		</div>
	{/if}
</div>
