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
			<span style="color: #e2e8f0; font-size: 0.875rem">
				{data.user.username}
				{#if data.user.role === 'admin'}
					<span style="color: #f59e0b; font-size: 0.75rem">[admin]</span>
				{/if}
			</span>
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
