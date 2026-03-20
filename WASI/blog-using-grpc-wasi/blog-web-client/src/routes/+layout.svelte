<script lang="ts">
	import '../app.css';
	import { enhance } from '$app/forms';
	import { browser } from '$app/environment';
	import { invalidateAll } from '$app/navigation';
	import { t, LOCALES, type Locale } from '$lib/i18n';
	import type { LayoutData } from './$types';

	export let data: LayoutData;

	$: theme = data.theme ?? 'dark';
	$: locale = (data.locale ?? 'ko') as Locale;

	function toggleTheme() {
		const newTheme = theme === 'dark' ? 'light' : 'dark';
		document.documentElement.setAttribute('data-theme', newTheme);
		document.cookie = `theme=${newTheme};path=/;max-age=${60 * 60 * 24 * 365};samesite=strict`;
		invalidateAll();
	}

	function changeLocale(event: Event) {
		const newLocale = (event.target as HTMLSelectElement).value;
		document.documentElement.lang = newLocale;
		document.cookie = `locale=${newLocale};path=/;max-age=${60 * 60 * 24 * 365};samesite=strict`;
		invalidateAll();
	}

	$: if (browser) {
		document.documentElement.setAttribute('data-theme', theme);
		document.documentElement.lang = locale;
	}
</script>

<nav>
	<a href="/" class="logo">Blog gRPC WASI</a>
	<div class="nav-links">
		<select class="locale-select" value={locale} on:change={changeLocale} title="Language">
			{#each LOCALES as loc}
				<option value={loc.code}>{loc.label}</option>
			{/each}
		</select>
		<button class="theme-toggle" on:click={toggleTheme} title={theme === 'dark' ? t(locale, 'nav.lightMode') : t(locale, 'nav.darkMode')}>
			{theme === 'dark' ? '☀️' : '🌙'}
		</button>
		{#if data.user}
			<a href="/profile" class="profile-link">
				{data.user.username}
				{#if data.user.role === 'admin'}
					<span style="color: #f59e0b; font-size: 0.75rem">[admin]</span>
				{/if}
			</a>
			<a href="/posts/new">{t(locale, 'nav.newPost')}</a>
			{#if data.user.role === 'admin'}
				<a href="/dashboard" style="color: #22c55e">{t(locale, 'nav.dashboard')}</a>
				<a href="/admin" style="color: #f59e0b">{t(locale, 'nav.admin')}</a>
			{/if}
			<form method="POST" action="/login?/logout" use:enhance>
				<button type="submit" class="btn-outline btn-sm">{t(locale, 'nav.logout')}</button>
			</form>
		{:else}
			<a href="/login">{t(locale, 'nav.login')}</a>
			<a href="/register">{t(locale, 'nav.register')}</a>
		{/if}
	</div>
</nav>

<slot />

<style>
	.profile-link {
		color: var(--text);
		font-size: 0.875rem;
		text-decoration: none;
		padding: 0.25rem 0.5rem;
		border-radius: 0.375rem;
		transition: background 0.15s;
	}
	.profile-link:hover {
		background: rgba(56, 189, 248, 0.1);
		color: var(--accent);
	}
	.locale-select {
		background: transparent;
		border: 1px solid var(--border);
		color: var(--text-muted);
		border-radius: 0.375rem;
		padding: 0.25rem 0.375rem;
		font-size: 0.8rem;
		cursor: pointer;
		transition: border-color 0.15s, color 0.15s;
	}
	.locale-select:hover {
		border-color: var(--accent);
		color: var(--text);
	}
	.locale-select option {
		background: var(--bg-card);
		color: var(--text);
	}
</style>
