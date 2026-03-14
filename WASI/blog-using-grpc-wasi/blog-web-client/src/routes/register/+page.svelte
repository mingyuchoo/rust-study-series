<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let isLoading = false;
</script>

<svelte:head>
	<title>회원가입 - Blog</title>
</svelte:head>

<div class="container" style="max-width: 420px">
	<h1>회원가입</h1>

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
				<label for="username">사용자명</label>
				<input id="username" name="username" type="text" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="email">이메일</label>
				<input id="email" name="email" type="email" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="password">비밀번호 (8자 이상)</label>
				<input id="password" name="password" type="password" minlength="8" required disabled={isLoading} />
			</div>
			<button type="submit" class="btn" style="width: 100%" disabled={isLoading}>
				{isLoading ? '가입 처리 중...' : '회원가입'}
			</button>
		</form>
	</div>

	<p class="meta" style="text-align: center">
		이미 계정이 있으신가요? <a href="/login">로그인</a>
	</p>
</div>
