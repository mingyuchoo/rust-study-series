<script lang="ts">
	import { t, type Locale } from '$lib/i18n';
	import type { PageData } from './$types';

	export let data: PageData;

	$: locale = (data.locale ?? 'ko') as Locale;
	$: stats = data.stats;
	$: publicRatio = stats.total_posts > 0
		? Math.round((stats.public_posts / stats.total_posts) * 100)
		: 0;
	$: privateRatio = 100 - publicRatio;
</script>

<svelte:head>
	<title>{t(locale, 'dashboard.title')} - Blog</title>
</svelte:head>

<div class="container" style="max-width: 800px">
	<h1>{t(locale, 'dashboard.title')}</h1>

	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
	{/if}

	<!-- Overview Cards -->
	<h2 style="margin-top: 1.5rem">{t(locale, 'dashboard.overview')}</h2>
	<div class="stats-grid">
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(56, 189, 248, 0.15); color: #38bdf8">
				<span>👥</span>
			</div>
			<div class="stat-body">
				<div class="stat-value">{stats.total_users}<span class="stat-unit">{t(locale, 'dashboard.unit')}</span></div>
				<div class="stat-label">{t(locale, 'dashboard.totalUsers')}</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(129, 140, 248, 0.15); color: #818cf8">
				<span>📝</span>
			</div>
			<div class="stat-body">
				<div class="stat-value">{stats.total_posts}<span class="stat-unit">{t(locale, 'dashboard.unitPost')}</span></div>
				<div class="stat-label">{t(locale, 'dashboard.totalPosts')}</div>
			</div>
		</div>
		<div class="stat-card">
			<div class="stat-icon" style="background: rgba(34, 197, 94, 0.15); color: #22c55e">
				<span>💬</span>
			</div>
			<div class="stat-body">
				<div class="stat-value">{stats.total_comments}<span class="stat-unit">{t(locale, 'dashboard.unitPost')}</span></div>
				<div class="stat-label">{t(locale, 'dashboard.totalComments')}</div>
			</div>
		</div>
	</div>

	<!-- Posts Breakdown -->
	<h2 style="margin-top: 2rem">{t(locale, 'dashboard.postsBreakdown')}</h2>
	<div class="card">
		<div class="breakdown-row">
			<div class="breakdown-label">
				<span class="breakdown-dot" style="background: #22c55e"></span>
				{t(locale, 'dashboard.publicPosts')}
			</div>
			<div class="breakdown-value">{stats.public_posts}{t(locale, 'dashboard.unitPost')}</div>
		</div>
		<div class="breakdown-bar">
			<div class="breakdown-fill breakdown-fill-public" style="width: {publicRatio}%"></div>
			<div class="breakdown-fill breakdown-fill-private" style="width: {privateRatio}%"></div>
		</div>
		<div class="breakdown-row" style="margin-top: 0.75rem">
			<div class="breakdown-label">
				<span class="breakdown-dot" style="background: #f59e0b"></span>
				{t(locale, 'dashboard.privatePosts')}
			</div>
			<div class="breakdown-value">{stats.private_posts}{t(locale, 'dashboard.unitPost')}</div>
		</div>
	</div>

	<!-- System Info -->
	<div class="meta" style="text-align: center; margin-top: 2rem">
		WASM Component v{data.version}
	</div>
</div>

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
	}
	@media (max-width: 640px) {
		.stats-grid {
			grid-template-columns: 1fr;
		}
	}
	.stat-card {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 1.25rem;
		border-radius: 0.75rem;
		border: 1px solid var(--border);
		background: var(--bg-card);
	}
	.stat-icon {
		width: 48px;
		height: 48px;
		border-radius: 0.75rem;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		flex-shrink: 0;
	}
	.stat-body {
		display: flex;
		flex-direction: column;
	}
	.stat-value {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text);
		line-height: 1.2;
	}
	.stat-unit {
		font-size: 0.875rem;
		font-weight: 400;
		color: var(--text-muted);
		margin-left: 0.25rem;
	}
	.stat-label {
		font-size: 0.8rem;
		color: var(--text-dim);
		margin-top: 0.125rem;
	}
	.breakdown-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.breakdown-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-body);
		font-size: 0.9rem;
	}
	.breakdown-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		display: inline-block;
	}
	.breakdown-value {
		font-weight: 600;
		color: var(--text);
		font-size: 0.9rem;
	}
	.breakdown-bar {
		display: flex;
		height: 12px;
		border-radius: 6px;
		overflow: hidden;
		margin-top: 0.75rem;
		background: var(--border);
	}
	.breakdown-fill-public {
		background: #22c55e;
		transition: width 0.3s;
	}
	.breakdown-fill-private {
		background: #f59e0b;
		transition: width 0.3s;
	}
</style>
