<script lang="ts">
	import { enhance } from '$app/forms';
	import { invalidateAll } from '$app/navigation';
	import { t, LOCALES, type Locale } from '$lib/i18n';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isProfileLoading = false;
	let isPasswordLoading = false;

	$: locale = (data.locale ?? 'ko') as Locale;

	function applyTheme(newTheme: string) {
		document.documentElement.setAttribute('data-theme', newTheme);
		document.cookie = `theme=${newTheme};path=/;max-age=${60 * 60 * 24 * 365};samesite=strict`;
		invalidateAll();
	}

	function applyLocale(newLocale: string) {
		document.documentElement.lang = newLocale;
		document.cookie = `locale=${newLocale};path=/;max-age=${60 * 60 * 24 * 365};samesite=strict`;
		invalidateAll();
	}
</script>

<svelte:head>
	<title>{t(locale, 'profile.title')} - Blog</title>
</svelte:head>

<div class="container">
	<h1>{t(locale, 'profile.title')}</h1>

	<!-- 프로필 정보 카드 -->
	<div class="card">
		<div class="profile-header">
			<div class="profile-avatar">
				{data.profile.username.charAt(0).toUpperCase()}
			</div>
			<div class="profile-info">
				<h2>{data.profile.username}</h2>
				<span class="meta">{data.profile.email}</span>
				<span class="badge" class:badge-admin={data.profile.role === 'admin'}>
					{data.profile.role}
				</span>
			</div>
		</div>
		<div class="meta" style="margin-top: 0.5rem">
			{t(locale, 'profile.joinDate')}: {new Date(data.profile.created_at).toLocaleDateString(locale === 'en' ? 'en-US' : locale === 'ja' ? 'ja-JP' : 'ko-KR')}
		</div>
	</div>

	<!-- 프로필 설정 -->
	<h2 style="margin-top: 2rem">{t(locale, 'profile.settings')}</h2>

	{#if form?.success}
		<div class="alert alert-success">{form.success}</div>
	{/if}
	{#if form?.error}
		<div class="alert alert-error">{form.error}</div>
	{/if}

	<div class="card">
		<form
			method="POST"
			action="?/updateProfile"
			use:enhance={() => {
				isProfileLoading = true;
				return async ({ update }) => { await update(); isProfileLoading = false; };
			}}
		>
			<div class="form-group">
				<label for="bio">{t(locale, 'profile.bio')}</label>
				<textarea
					id="bio"
					name="bio"
					rows="4"
					maxlength="500"
					placeholder={t(locale, 'profile.bioPlaceholder')}
					disabled={isProfileLoading}
				>{data.profile.bio ?? ''}</textarea>
				<span class="meta">{t(locale, 'profile.bioHelp')}</span>
			</div>
			<div class="form-group">
				<label for="website">{t(locale, 'profile.website')}</label>
				<input
					id="website"
					name="website"
					type="url"
					maxlength="200"
					placeholder="https://example.com"
					value={data.profile.website ?? ''}
					disabled={isProfileLoading}
				/>
			</div>
			<div class="form-group">
				<label for="theme">{t(locale, 'profile.theme')}</label>
				<div class="theme-selector">
					<label class="theme-option">
						<input
							type="radio"
							name="theme"
							value="dark"
							checked={data.profile.theme !== 'light'}
							disabled={isProfileLoading}
							on:change={() => applyTheme('dark')}
						/>
						<span class="theme-preview theme-preview-dark">
							<span class="theme-icon">🌙</span>
							<span>{t(locale, 'profile.themeDark')}</span>
						</span>
					</label>
					<label class="theme-option">
						<input
							type="radio"
							name="theme"
							value="light"
							checked={data.profile.theme === 'light'}
							disabled={isProfileLoading}
							on:change={() => applyTheme('light')}
						/>
						<span class="theme-preview theme-preview-light">
							<span class="theme-icon">☀️</span>
							<span>{t(locale, 'profile.themeLight')}</span>
						</span>
					</label>
				</div>
				<span class="meta">{t(locale, 'profile.themeHelp')}</span>
			</div>
			<div class="form-group">
				<label for="locale">{t(locale, 'profile.locale')}</label>
				<div class="theme-selector">
					{#each LOCALES as loc}
						<label class="theme-option">
							<input
								type="radio"
								name="locale"
								value={loc.code}
								checked={(data.profile.locale || 'ko') === loc.code}
								disabled={isProfileLoading}
								on:change={() => applyLocale(loc.code)}
							/>
							<span class="theme-preview">
								<span>{loc.label}</span>
							</span>
						</label>
					{/each}
				</div>
				<span class="meta">{t(locale, 'profile.localeHelp')}</span>
			</div>
			<div style="display: flex; justify-content: flex-end">
				<button type="submit" class="btn" disabled={isProfileLoading}>
					{isProfileLoading ? t(locale, 'profile.saving') : t(locale, 'profile.save')}
				</button>
			</div>
		</form>
	</div>

	<!-- 비밀번호 변경 -->
	<h2 style="margin-top: 2rem">{t(locale, 'profile.changePassword')}</h2>

	{#if form?.passwordSuccess}
		<div class="alert alert-success">{form.passwordSuccess}</div>
	{/if}
	{#if form?.passwordError}
		<div class="alert alert-error">{form.passwordError}</div>
	{/if}

	<div class="card">
		<form
			method="POST"
			action="?/changePassword"
			use:enhance={() => {
				isPasswordLoading = true;
				return async ({ update }) => { await update(); isPasswordLoading = false; };
			}}
		>
			<div class="form-group">
				<label for="current_password">{t(locale, 'profile.currentPassword')}</label>
				<input
					id="current_password"
					name="current_password"
					type="password"
					required
					disabled={isPasswordLoading}
				/>
			</div>
			<div class="form-group">
				<label for="new_password">{t(locale, 'profile.newPassword')}</label>
				<input
					id="new_password"
					name="new_password"
					type="password"
					required
					minlength="8"
					disabled={isPasswordLoading}
				/>
				<span class="meta">{t(locale, 'profile.newPasswordHelp')}</span>
			</div>
			<div class="form-group">
				<label for="confirm_password">{t(locale, 'profile.confirmPassword')}</label>
				<input
					id="confirm_password"
					name="confirm_password"
					type="password"
					required
					minlength="8"
					disabled={isPasswordLoading}
				/>
			</div>
			<div style="display: flex; justify-content: flex-end">
				<button type="submit" class="btn" disabled={isPasswordLoading}>
					{isPasswordLoading ? t(locale, 'profile.changingPassword') : t(locale, 'profile.changePasswordSubmit')}
				</button>
			</div>
		</form>
	</div>
</div>

<style>
	.profile-header {
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}
	.profile-avatar {
		width: 64px;
		height: 64px;
		border-radius: 50%;
		background: linear-gradient(135deg, #0ea5e9, #6366f1);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.5rem;
		font-weight: 700;
		color: white;
		flex-shrink: 0;
	}
	.profile-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}
	.profile-info h2 {
		margin: 0;
	}
	.badge {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.75rem;
		font-weight: 600;
		background: var(--border);
		color: var(--text-muted);
		width: fit-content;
	}
	.badge-admin {
		background: rgba(245, 158, 11, 0.15);
		color: #f59e0b;
	}

	/* Theme & Locale selector */
	.theme-selector {
		display: flex;
		gap: 0.75rem;
		margin-top: 0.25rem;
		flex-wrap: wrap;
	}
	.theme-option {
		cursor: pointer;
	}
	.theme-option input[type="radio"] {
		display: none;
	}
	.theme-preview {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1.25rem;
		border-radius: 0.5rem;
		border: 2px solid var(--border);
		font-size: 0.9rem;
		font-weight: 500;
		transition: border-color 0.15s;
	}
	.theme-option input[type="radio"]:checked + .theme-preview {
		border-color: var(--accent);
	}
	.theme-preview-dark {
		background: #1e293b;
		color: var(--text);
	}
	.theme-preview-light {
		background: #f1f5f9;
		color: #0f172a;
	}
	.theme-icon {
		font-size: 1.125rem;
	}
</style>
