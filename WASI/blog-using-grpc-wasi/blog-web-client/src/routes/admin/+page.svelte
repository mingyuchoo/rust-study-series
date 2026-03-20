<script lang="ts">
	import { enhance } from '$app/forms';
	import { t, dateLocale, type Locale } from '$lib/i18n';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: tab = data.tab;
	$: locale = (data.locale ?? 'ko') as Locale;
</script>

<svelte:head>
	<title>{t(locale, 'admin.title')} - Blog</title>
</svelte:head>

<div class="container" style="max-width: 960px">
	<h1>{t(locale, 'admin.title')}</h1>

	{#if form?.error}
		<div class="alert alert-error">{form.error}</div>
	{/if}
	{#if data.error}
		<div class="alert alert-error">{data.error}</div>
	{/if}

	<!-- Tabs -->
	<div style="display: flex; gap: 0.5rem; margin-bottom: 1.5rem; border-bottom: 1px solid #334155; padding-bottom: 0.75rem">
		<a
			href="/admin?tab=users"
			class="btn btn-sm"
			style={tab === 'users' ? '' : 'background: transparent; border: 1px solid #334155; color: #94a3b8'}
		>
			{t(locale, 'admin.users')} ({data.usersTotal})
		</a>
		<a
			href="/admin?tab=posts"
			class="btn btn-sm"
			style={tab === 'posts' ? '' : 'background: transparent; border: 1px solid #334155; color: #94a3b8'}
		>
			{t(locale, 'admin.posts')} ({data.postsTotal})
		</a>
		<a
			href="/admin?tab=comments"
			class="btn btn-sm"
			style={tab === 'comments' ? '' : 'background: transparent; border: 1px solid #334155; color: #94a3b8'}
		>
			{t(locale, 'admin.comments')} ({data.commentsTotal})
		</a>
	</div>

	<!-- Users Tab -->
	{#if tab === 'users'}
		{#if data.users.length === 0}
			<p class="meta" style="text-align: center">{t(locale, 'admin.noUsers')}</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.username')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.email')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.role')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.actions')}</th>
					</tr>
				</thead>
				<tbody>
					{#each data.users as user}
						<tr style="border-bottom: 1px solid #1e293b">
							<td style="padding: 0.75rem; color: var(--text)">{user.username}</td>
							<td style="padding: 0.75rem; color: #94a3b8; font-size: 0.9rem">{user.email}</td>
							<td style="padding: 0.75rem">
								<span style="color: {user.role === 'admin' ? '#f59e0b' : '#38bdf8'}; font-size: 0.85rem; font-weight: 600">
									{user.role}
								</span>
							</td>
							<td style="padding: 0.75rem">
								<div style="display: flex; gap: 0.5rem">
									<form method="POST" action="?/updateRole" style="margin: 0" use:enhance>
										<input type="hidden" name="userId" value={user.id} />
										<input type="hidden" name="role" value={user.role === 'admin' ? 'user' : 'admin'} />
										<button type="submit" class="btn btn-sm btn-outline">
											{user.role === 'admin' ? t(locale, 'admin.toUser') : t(locale, 'admin.toAdmin')}
										</button>
									</form>
									<form
										method="POST"
										action="?/deleteUser"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm(t(locale, 'admin.confirmDeleteUser', { username: user.username }))) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="userId" value={user.id} />
										<button type="submit" class="btn btn-sm btn-danger">{t(locale, 'post.delete')}</button>
									</form>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	{/if}

	<!-- Posts Tab -->
	{#if tab === 'posts'}
		{#if data.posts.length === 0}
			<p class="meta" style="text-align: center">{t(locale, 'admin.noPosts')}</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.postTitle')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.author')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.publicLabel')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.commentCount')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.actions')}</th>
					</tr>
				</thead>
				<tbody>
					{#each data.posts as post}
						<tr style="border-bottom: 1px solid #1e293b">
							<td style="padding: 0.75rem">
								<a href="/posts/{post.id}" style="color: var(--text)">{post.title}</a>
							</td>
							<td style="padding: 0.75rem; color: #94a3b8; font-size: 0.9rem">
								{post.author?.username ?? '?'}
							</td>
							<td style="padding: 0.75rem">
								<span style="color: {post.visibility === 'public' ? '#22c55e' : '#f59e0b'}; font-size: 0.85rem">
									{post.visibility}
								</span>
							</td>
							<td style="padding: 0.75rem; color: #94a3b8">{post.comment_count}</td>
							<td style="padding: 0.75rem">
								<div style="display: flex; gap: 0.5rem">
									<form method="POST" action="?/updateVisibility" style="margin: 0" use:enhance>
										<input type="hidden" name="postId" value={post.id} />
										<input type="hidden" name="visibility" value={post.visibility === 'public' ? 'private' : 'public'} />
										<button type="submit" class="btn btn-sm btn-outline">
											{post.visibility === 'public' ? t(locale, 'admin.makePrivate') : t(locale, 'admin.makePublic')}
										</button>
									</form>
									<a href="/posts/{post.id}/edit" class="btn btn-sm btn-outline">{t(locale, 'post.edit')}</a>
									<form
										method="POST"
										action="?/deletePost"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm(t(locale, 'admin.confirmDeletePost', { title: post.title }))) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="postId" value={post.id} />
										<button type="submit" class="btn btn-sm btn-danger">{t(locale, 'post.delete')}</button>
									</form>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	{/if}

	<!-- Comments Tab -->
	{#if tab === 'comments'}
		{#if data.comments.length === 0}
			<p class="meta" style="text-align: center">{t(locale, 'admin.noComments')}</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.posts')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.author')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.content')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.createdAt')}</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">{t(locale, 'admin.actions')}</th>
					</tr>
				</thead>
				<tbody>
					{#each data.comments as comment}
						<tr style="border-bottom: 1px solid #1e293b">
							<td style="padding: 0.75rem">
								<a href="/posts/{comment.post_id}" style="color: #38bdf8; font-size: 0.9rem">
									{comment.post_title}
								</a>
							</td>
							<td style="padding: 0.75rem; color: #94a3b8; font-size: 0.9rem">
								{comment.author?.username ?? '?'}
							</td>
							<td style="padding: 0.75rem; color: #cbd5e1; font-size: 0.9rem; max-width: 300px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
								{comment.content}
							</td>
							<td style="padding: 0.75rem; color: #64748b; font-size: 0.8rem; white-space: nowrap">
								{new Date(comment.created_at).toLocaleDateString(dateLocale(locale))}
							</td>
							<td style="padding: 0.75rem">
								<div style="display: flex; gap: 0.5rem">
									<a href="/posts/{comment.post_id}" class="btn btn-sm btn-outline">{t(locale, 'admin.view')}</a>
									<form
										method="POST"
										action="?/deleteComment"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm(t(locale, 'admin.confirmDeleteComment'))) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="commentId" value={comment.id} />
										<button type="submit" class="btn btn-sm btn-danger">{t(locale, 'post.delete')}</button>
									</form>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
	{/if}
</div>
