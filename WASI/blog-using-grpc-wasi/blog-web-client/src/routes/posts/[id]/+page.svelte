<script lang="ts">
	import { enhance } from '$app/forms';
	import { renderMarkdown } from '$lib/markdown';
	import { t, dateLocale, type Locale } from '$lib/i18n';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let isLoading = false;
	let isDeleting = false;

	$: locale = (data.locale ?? 'ko') as Locale;
	$: isOwner = data.user && data.post && data.user.id === data.post.author?.id;
	$: isAdmin = data.user?.role === 'admin';
	$: canEdit = isOwner || isAdmin;
	$: renderedContent = data.post ? renderMarkdown(data.post.content) : '';
</script>

<svelte:head>
	<title>{data.post?.title ?? 'Post'} - Blog</title>
</svelte:head>

<div class="container">
	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
		<a href="/">&larr; {t(locale, 'post.backToList')}</a>
	{:else if data.post}
		{#if form?.error}
			<div class="alert alert-error">{form.error}</div>
		{/if}
		<article>
			<a href="/" class="meta" style="display: inline-block; margin-bottom: 1rem">&larr; {t(locale, 'post.backToList')}</a>
			<h1 style="margin-bottom: 0.5rem">{data.post.title}</h1>
			<div class="meta" style="margin-bottom: 1.5rem">
				<span>{data.post.author?.username ?? '?'}</span>
				<span> &middot; </span>
				<span>{new Date(data.post.created_at).toLocaleDateString(dateLocale(locale))}</span>
				{#if data.post.visibility === 'private'}
					<span> &middot; </span>
					<span style="color: #f59e0b">{t(locale, 'post.private')}</span>
				{/if}
			</div>
			{#if canEdit}
				<div style="display: flex; gap: 0.75rem; margin-bottom: 1rem">
					<a href="/posts/{data.post.id}/edit" class="btn btn-sm btn-outline">{t(locale, 'post.edit')}</a>
					<form
						method="POST"
						action="?/deletePost"
						style="margin: 0"
						use:enhance={({ cancel }) => {
							if (!confirm(t(locale, 'post.confirmDelete'))) {
								cancel();
								return;
							}
							isDeleting = true;
							return async ({ update }) => { await update(); isDeleting = false; };
						}}
					>
						<button type="submit" class="btn btn-sm btn-danger" disabled={isDeleting}>
							{isDeleting ? t(locale, 'post.deleting') : t(locale, 'post.delete')}
						</button>
					</form>
				</div>
			{/if}

			<div class="card markdown-body">
				{@html renderedContent}
			</div>
		</article>

		<!-- Comments Section -->
		<section style="margin-top: 2rem">
			<h2>{t(locale, 'post.comments')} ({data.comments.length})</h2>

			{#if data.user}
				<div class="card">
					<form
						method="POST"
						action="?/addComment"
						use:enhance={() => {
							isLoading = true;
							return async ({ update }) => { await update(); isLoading = false; };
						}}
					>
						<div class="form-group" style="margin-bottom: 0.75rem">
							<textarea
								name="content"
								placeholder={t(locale, 'post.commentPlaceholder')}
								rows="3"
								required
								disabled={isLoading}
								style="min-height: 80px"
							></textarea>
						</div>
						<div style="display: flex; gap: 0.75rem; align-items: center">
							<label class="checkbox-label" style="margin: 0">
								<input type="checkbox" name="visibility" disabled={isLoading} />
								<span>{t(locale, 'post.public')}</span>
							</label>
							<button type="submit" class="btn btn-sm" disabled={isLoading}>
								{isLoading ? t(locale, 'post.addingComment') : t(locale, 'post.addComment')}
							</button>
						</div>
					</form>
				</div>
			{:else}
				<div class="card" style="text-align: center; color: var(--text-dim)">
					{@html t(locale, 'post.loginToComment', { link: `<a href="/login">${t(locale, 'nav.login')}</a>` })}
				</div>
			{/if}

			{#each data.comments as comment}
				<div class="card" style="padding: 1rem">
					<div class="meta" style="margin-bottom: 0.5rem; display: flex; justify-content: space-between; align-items: center">
						<span>
							<strong style="color: var(--text)">{comment.author?.username ?? '?'}</strong>
							<span> &middot; </span>
							<span>{new Date(comment.created_at).toLocaleDateString(dateLocale(locale))}</span>
						</span>
						{#if data.user && (data.user.id === comment.author?.id || isAdmin)}
							<form
								method="POST"
								action="?/deleteComment"
								style="margin: 0"
								use:enhance={({ cancel }) => {
									if (!confirm(t(locale, 'post.confirmDeleteComment'))) {
										cancel();
										return;
									}
									return async ({ update }) => { await update(); };
								}}
							>
								<input type="hidden" name="commentId" value={comment.id} />
								<button type="submit" class="btn btn-sm btn-danger">{t(locale, 'post.delete')}</button>
							</form>
						{/if}
					</div>
					<p style="margin: 0; color: var(--text-body); font-size: 0.9rem; white-space: pre-wrap">
						{comment.content}
					</p>
				</div>
			{/each}

			{#if data.comments.length === 0}
				<p class="meta" style="text-align: center">{t(locale, 'post.noComments')}</p>
			{/if}
		</section>
	{/if}
</div>
