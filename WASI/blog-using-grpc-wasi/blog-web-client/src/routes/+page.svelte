<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<svelte:head>
	<title>Blog - Home</title>
</svelte:head>

<div class="container">
	<h1>Blog Posts</h1>

	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
	{/if}

	{#if data.posts.length === 0}
		<div class="card" style="text-align: center; color: #64748b;">
			<p>아직 작성된 포스트가 없습니다.</p>
			<a href="/posts/new" class="btn" style="display: inline-block; margin-top: 0.5rem">첫 글 작성하기</a>
		</div>
	{:else}
		{#each data.posts as post}
			<a href="/posts/{post.id}" style="text-decoration: none; color: inherit">
				<div class="card" style="cursor: pointer; transition: border-color 0.15s">
					<h2 style="color: #e2e8f0; margin-bottom: 0.5rem">{post.title}</h2>
					<p style="color: #94a3b8; font-size: 0.9rem; margin: 0 0 0.75rem; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden">
						{post.content}
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
