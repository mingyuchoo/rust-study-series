<script lang="ts">
	import { enhance } from '$app/forms';
	import { t, type Locale } from '$lib/i18n';
	import type { ActionData } from './$types';
	import type { LayoutData } from '../$types';

	export let form: ActionData;
	export let data: LayoutData;
	let isLoading = false;

	$: locale = (data.locale ?? 'ko') as Locale;
</script>

<svelte:head>
	<title>{t(locale, 'login.title')} - Blog</title>
</svelte:head>

<div class="container" style="max-width: 420px">
	<h1>{t(locale, 'login.title')}</h1>

	{#if form?.error}
		<div class="alert alert-error">{form.error}</div>
	{/if}

	<div class="card">
		<form
			method="POST"
			action="?/login"
			use:enhance={() => {
				isLoading = true;
				return async ({ update }) => { await update(); isLoading = false; };
			}}
		>
			<div class="form-group">
				<label for="email">{t(locale, 'login.email')}</label>
				<input id="email" name="email" type="email" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="password">{t(locale, 'login.password')}</label>
				<input id="password" name="password" type="password" required disabled={isLoading} />
			</div>
			<button type="submit" class="btn" style="width: 100%" disabled={isLoading}>
				{isLoading ? t(locale, 'login.loading') : t(locale, 'login.submit')}
			</button>
		</form>
	</div>

	<p class="meta" style="text-align: center">
		{t(locale, 'login.noAccount')} <a href="/register">{t(locale, 'nav.register')}</a>
	</p>
</div>
