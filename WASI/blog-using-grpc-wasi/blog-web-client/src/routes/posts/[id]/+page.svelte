<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isLoading = false;
</script>

<svelte:head>
	<title>{data.post?.title ?? 'Post'} - Blog</title>
</svelte:head>

<div class="container">
	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
		<a href="/">&larr; 목록으로</a>
	{:else if data.post}
		<article>
			<a href="/" class="meta" style="display: inline-block; margin-bottom: 1rem">&larr; 목록으로</a>
			<h1 style="margin-bottom: 0.5rem">{data.post.title}</h1>
			<div class="meta" style="margin-bottom: 1.5rem">
				<span>{data.post.author?.username ?? '?'}</span>
				<span> &middot; </span>
				<span>{new Date(data.post.created_at).toLocaleDateString('ko-KR')}</span>
			</div>
			<div class="card">
				<p style="white-space: pre-wrap; line-height: 1.7; margin: 0; color: #cbd5e1">
					{data.post.content}
				</p>
			</div>
		</article>

		<!-- Comments Section -->
		<section style="margin-top: 2rem">
			<h2>댓글 ({data.comments.length})</h2>

			{#if form?.error}
				<div class="alert alert-error">{form.error}</div>
			{/if}

			{#if data.user}
				<div class="card">
					<form
						method="POST"
						action="?/addComment"
						use:enhance={() => {
							isLoading = true;
							return async ({ update }) => { await update(); isLoading = false; };
						}}
					>
						<div class="form-group" style="margin-bottom: 0.75rem">
							<textarea
								name="content"
								placeholder="댓글을 입력하세요..."
								rows="3"
								required
								disabled={isLoading}
								style="min-height: 80px"
							></textarea>
						</div>
						<button type="submit" class="btn btn-sm" disabled={isLoading}>
							{isLoading ? '등록 중...' : '댓글 등록'}
						</button>
					</form>
				</div>
			{:else}
				<div class="card" style="text-align: center; color: #64748b">
					댓글을 작성하려면 <a href="/login">로그인</a>해주세요.
				</div>
			{/if}

			{#each data.comments as comment}
				<div class="card" style="padding: 1rem">
					<div class="meta" style="margin-bottom: 0.5rem">
						<strong style="color: #e2e8f0">{comment.author?.username ?? '?'}</strong>
						<span> &middot; </span>
						<span>{new Date(comment.created_at).toLocaleDateString('ko-KR')}</span>
					</div>
					<p style="margin: 0; color: #cbd5e1; font-size: 0.9rem; white-space: pre-wrap">
						{comment.content}
					</p>
				</div>
			{/each}

			{#if data.comments.length === 0}
				<p class="meta" style="text-align: center">아직 댓글이 없습니다.</p>
			{/if}
		</section>
	{/if}
</div>
