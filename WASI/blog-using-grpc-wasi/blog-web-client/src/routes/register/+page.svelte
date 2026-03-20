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
	<title>{t(locale, 'register.title')} - Blog</title>
</svelte:head>

<div class="container" style="max-width: 420px">
	<h1>{t(locale, 'register.title')}</h1>

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
				<label for="username">{t(locale, 'register.username')}</label>
				<input id="username" name="username" type="text" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="email">{t(locale, 'register.email')}</label>
				<input id="email" name="email" type="email" required disabled={isLoading} />
			</div>
			<div class="form-group">
				<label for="password">{t(locale, 'register.password')}</label>
				<input id="password" name="password" type="password" minlength="8" required disabled={isLoading} />
			</div>
			<button type="submit" class="btn" style="width: 100%" disabled={isLoading}>
				{isLoading ? t(locale, 'register.loading') : t(locale, 'register.submit')}
			</button>
		</form>
	</div>

	<p class="meta" style="text-align: center">
		{t(locale, 'register.hasAccount')} <a href="/login">{t(locale, 'nav.login')}</a>
	</p>
</div>
