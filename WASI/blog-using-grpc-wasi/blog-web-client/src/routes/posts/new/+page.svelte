<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isLoading = false;
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
				<label for="content">내용</label>
				<textarea id="content" name="content" rows="12" required disabled={isLoading}></textarea>
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
