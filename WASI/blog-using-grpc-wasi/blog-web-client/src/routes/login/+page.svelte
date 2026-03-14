<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isLoading = false;
</script>

<svelte:head>
	<title>로그인 - Blog</title>
</svelte:head>

<div class="container" style="max-width: 420px">
	<h1>로그인</h1>

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
				<label for="email">이메일</label>
				<input id="email" name="email" type="email" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="password">비밀번호</label>
				<input id="password" name="password" type="password" required disabled={isLoading} />
			</div>
			<button type="submit" class="btn" style="width: 100%" disabled={isLoading}>
				{isLoading ? '로그인 중...' : '로그인'}
			</button>
		</form>
	</div>

	<p class="meta" style="text-align: center">
		계정이 없으신가요? <a href="/register">회원가입</a>
	</p>
</div>
