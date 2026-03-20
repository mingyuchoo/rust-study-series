<script lang="ts">
	import { renderExcerpt } from '$lib/markdown';
	import type { PageData } from './$types';

	export let data: PageData;

	$: filter = data.filter || 'public';
</script>

<svelte:head>
	<title>Blog - Home</title>
</svelte:head>

<div class="container">
	<h1>Blog Posts</h1>

	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
	{/if}

	<!-- Filter Tabs -->
	<div class="filter-tabs">
		<a href="/?filter=public" class="filter-tab" class:active={filter === 'public'}>
			공개 포스트
		</a>
		{#if data.user}
			<a href="/?filter=mine" class="filter-tab" class:active={filter === 'mine'}>
				내 포스트
			</a>
		{/if}
	</div>

	{#if data.posts.length === 0}
		<div class="card empty-state">
			{#if filter === 'mine'}
				<p>작성한 포스트가 없습니다.</p>
			{:else}
				<p>공개된 포스트가 없습니다.</p>
			{/if}
			<a href="/posts/new" class="btn" style="display: inline-block; margin-top: 0.5rem">첫 글 작성하기</a>
		</div>
	{:else}
		{#each data.posts as post}
			<a href="/posts/{post.id}" class="post-link">
				<div class="card post-card">
					<div class="post-header">
						<h2 class="post-title">{post.title}</h2>
						{#if post.visibility === 'private'}
							<span class="badge-private">비공개</span>
						{/if}
					</div>
					<p class="post-excerpt">
						{renderExcerpt(post.content)}
					</p>
					<div class="meta">
						<span>{post.author?.username ?? '?'}</span>
						<span> &middot; </span>
						<span>{new Date(post.created_at).toLocaleDateString('ko-KR')}</span>
						<span> &middot; </span>
						<span>댓글 {post.comment_count}개</span>
					</div>
				</div>
			</a>
		{/each}

		{#if data.total > data.posts.length}
			<p class="meta" style="text-align: center">총 {data.total}건 중 {data.posts.length}건 표시</p>
		{/if}
	{/if}
</div>

<style>
	.filter-tabs {
		display: flex;
		gap: 0.5rem;
		margin-bottom: 1.5rem;
		border-bottom: 1px solid var(--border);
		padding-bottom: 0.75rem;
	}
	.filter-tab {
		padding: 0.375rem 0.75rem;
		font-size: 0.8rem;
		font-weight: 600;
		border-radius: 0.5rem;
		border: 1px solid var(--border);
		background: transparent;
		color: var(--text-muted);
		text-decoration: none;
		transition: all 0.15s;
	}
	.filter-tab:hover {
		border-color: var(--accent);
		color: var(--text);
		text-decoration: none;
	}
	.filter-tab.active {
		background: var(--btn-gradient);
		border-color: transparent;
		color: white;
	}
	.empty-state {
		text-align: center;
		color: var(--text-dim);
	}
	.post-link {
		text-decoration: none;
		color: inherit;
	}
	.post-link:hover {
		text-decoration: none;
	}
	.post-card {
		cursor: pointer;
		transition: border-color 0.15s;
	}
	.post-card:hover {
		border-color: var(--accent);
	}
	.post-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-bottom: 0.5rem;
	}
	.post-title {
		color: var(--text);
		margin: 0;
		flex: 1;
	}
	.badge-private {
		color: #f59e0b;
		font-size: 0.75rem;
		border: 1px solid #f59e0b;
		border-radius: 0.25rem;
		padding: 0.125rem 0.375rem;
		white-space: nowrap;
	}
	.post-excerpt {
		color: var(--text-muted);
		font-size: 0.9rem;
		margin: 0 0 0.75rem;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
