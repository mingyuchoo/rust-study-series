<script lang="ts">
	import { enhance } from '$app/forms';
	import { renderMarkdown } from '$lib/markdown';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isLoading = false;
	let isDeleting = false;

	$: isOwner = data.user && data.post && data.user.id === data.post.author?.id;
	$: isAdmin = data.user?.role === 'admin';
	$: canEdit = isOwner || isAdmin;
	$: renderedContent = data.post ? renderMarkdown(data.post.content) : '';
</script>

<svelte:head>
	<title>{data.post?.title ?? 'Post'} - Blog</title>
</svelte:head>

<div class="container">
	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
		<a href="/">&larr; 목록으로</a>
	{:else if data.post}
		{#if form?.error}
			<div class="alert alert-error">{form.error}</div>
		{/if}
		<article>
			<a href="/" class="meta" style="display: inline-block; margin-bottom: 1rem">&larr; 목록으로</a>
			<h1 style="margin-bottom: 0.5rem">{data.post.title}</h1>
			<div class="meta" style="margin-bottom: 1.5rem">
				<span>{data.post.author?.username ?? '?'}</span>
				<span> &middot; </span>
				<span>{new Date(data.post.created_at).toLocaleDateString('ko-KR')}</span>
				{#if data.post.visibility === 'private'}
					<span> &middot; </span>
					<span style="color: #f59e0b">비공개</span>
				{/if}
			</div>
			{#if canEdit}
				<div style="display: flex; gap: 0.75rem; margin-bottom: 1rem">
					<a href="/posts/{data.post.id}/edit" class="btn btn-sm btn-outline">수정</a>
					<form
						method="POST"
						action="?/deletePost"
						style="margin: 0"
						use:enhance={({ cancel }) => {
							if (!confirm('정말로 이 포스트를 삭제하시겠습니까?')) {
								cancel();
								return;
							}
							isDeleting = true;
							return async ({ update }) => { await update(); isDeleting = false; };
						}}
					>
						<button type="submit" class="btn btn-sm btn-danger" disabled={isDeleting}>
							{isDeleting ? '삭제 중...' : '삭제'}
						</button>
					</form>
				</div>
			{/if}

			<div class="card markdown-body">
				{@html renderedContent}
			</div>
		</article>

		<!-- Comments Section -->
		<section style="margin-top: 2rem">
			<h2>댓글 ({data.comments.length})</h2>

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
						<div style="display: flex; gap: 0.75rem; align-items: center">
							<label class="checkbox-label" style="margin: 0">
								<input type="checkbox" name="visibility" disabled={isLoading} />
								<span>공개</span>
							</label>
							<button type="submit" class="btn btn-sm" disabled={isLoading}>
								{isLoading ? '등록 중...' : '댓글 등록'}
							</button>
						</div>
					</form>
				</div>
			{:else}
				<div class="card" style="text-align: center; color: var(--text-dim)">
					댓글을 작성하려면 <a href="/login">로그인</a>해주세요.
				</div>
			{/if}

			{#each data.comments as comment}
				<div class="card" style="padding: 1rem">
					<div class="meta" style="margin-bottom: 0.5rem; display: flex; justify-content: space-between; align-items: center">
						<span>
							<strong style="color: var(--text)">{comment.author?.username ?? '?'}</strong>
							<span> &middot; </span>
							<span>{new Date(comment.created_at).toLocaleDateString('ko-KR')}</span>
						</span>
						{#if data.user && (data.user.id === comment.author?.id || isAdmin)}
							<form
								method="POST"
								action="?/deleteComment"
								style="margin: 0"
								use:enhance={({ cancel }) => {
									if (!confirm('이 댓글을 삭제하시겠습니까?')) {
										cancel();
										return;
									}
									return async ({ update }) => { await update(); };
								}}
							>
								<input type="hidden" name="commentId" value={comment.id} />
								<button type="submit" class="btn btn-sm btn-danger">삭제</button>
							</form>
						{/if}
					</div>
					<p style="margin: 0; color: var(--text-body); font-size: 0.9rem; white-space: pre-wrap">
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
