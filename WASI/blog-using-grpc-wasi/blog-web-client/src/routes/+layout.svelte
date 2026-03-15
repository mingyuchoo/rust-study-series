<script lang="ts">
	import '../app.css';
	import { enhance } from '$app/forms';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<nav>
	<a href="/" class="logo">Blog gRPC WASI</a>
	<div class="nav-links">
		{#if data.user}
			<a href="/profile" class="profile-link">
				{data.user.username}
				{#if data.user.role === 'admin'}
					<span style="color: #f59e0b; font-size: 0.75rem">[admin]</span>
				{/if}
			</a>
			<a href="/posts/new">새 글 작성</a>
			{#if data.user.role === 'admin'}
				<a href="/admin" style="color: #f59e0b">관리</a>
			{/if}
			<form method="POST" action="/login?/logout" use:enhance>
				<button type="submit" class="btn-outline btn-sm">로그아웃</button>
			</form>
		{:else}
			<a href="/login">로그인</a>
			<a href="/register">회원가입</a>
		{/if}
	</div>
</nav>

<slot />

<style>
	.profile-link {
		color: #e2e8f0;
		font-size: 0.875rem;
		text-decoration: none;
		padding: 0.25rem 0.5rem;
		border-radius: 0.375rem;
		transition: background 0.15s;
	}
	.profile-link:hover {
		background: rgba(56, 189, 248, 0.1);
		color: #38bdf8;
	}
</style>
