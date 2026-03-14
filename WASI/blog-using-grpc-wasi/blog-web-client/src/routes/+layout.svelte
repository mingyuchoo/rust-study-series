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
			<span style="color: #e2e8f0; font-size: 0.875rem">{data.user.username}</span>
			<a href="/posts/new">새 글 작성</a>
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
