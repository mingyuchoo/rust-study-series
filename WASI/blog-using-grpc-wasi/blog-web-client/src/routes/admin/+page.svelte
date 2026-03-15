<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: tab = data.tab;
</script>

<svelte:head>
	<title>관리자 패널 - Blog</title>
</svelte:head>

<div class="container" style="max-width: 960px">
	<h1>관리자 패널</h1>

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
			사용자 관리 ({data.usersTotal})
		</a>
		<a
			href="/admin?tab=posts"
			class="btn btn-sm"
			style={tab === 'posts' ? '' : 'background: transparent; border: 1px solid #334155; color: #94a3b8'}
		>
			포스트 관리 ({data.postsTotal})
		</a>
		<a
			href="/admin?tab=comments"
			class="btn btn-sm"
			style={tab === 'comments' ? '' : 'background: transparent; border: 1px solid #334155; color: #94a3b8'}
		>
			댓글 관리 ({data.commentsTotal})
		</a>
	</div>

	<!-- Users Tab -->
	{#if tab === 'users'}
		{#if data.users.length === 0}
			<p class="meta" style="text-align: center">사용자가 없습니다.</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">사용자명</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">이메일</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">역할</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작업</th>
					</tr>
				</thead>
				<tbody>
					{#each data.users as user}
						<tr style="border-bottom: 1px solid #1e293b">
							<td style="padding: 0.75rem; color: #e2e8f0">{user.username}</td>
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
											{user.role === 'admin' ? 'user로' : 'admin으로'}
										</button>
									</form>
									<form
										method="POST"
										action="?/deleteUser"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm(`${user.username} 사용자를 삭제하시겠습니까? 관련 포스트와 댓글이 모두 삭제됩니다.`)) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="userId" value={user.id} />
										<button type="submit" class="btn btn-sm btn-danger">삭제</button>
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
			<p class="meta" style="text-align: center">포스트가 없습니다.</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">제목</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작성자</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">공개</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">댓글</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작업</th>
					</tr>
				</thead>
				<tbody>
					{#each data.posts as post}
						<tr style="border-bottom: 1px solid #1e293b">
							<td style="padding: 0.75rem">
								<a href="/posts/{post.id}" style="color: #e2e8f0">{post.title}</a>
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
											{post.visibility === 'public' ? '비공개' : '공개'}
										</button>
									</form>
									<a href="/posts/{post.id}/edit" class="btn btn-sm btn-outline">수정</a>
									<form
										method="POST"
										action="?/deletePost"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm(`"${post.title}" 포스트를 삭제하시겠습니까?`)) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="postId" value={post.id} />
										<button type="submit" class="btn btn-sm btn-danger">삭제</button>
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
			<p class="meta" style="text-align: center">댓글이 없습니다.</p>
		{:else}
			<table style="width: 100%; border-collapse: collapse">
				<thead>
					<tr style="border-bottom: 1px solid #334155; text-align: left">
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">포스트</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작성자</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">내용</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작성일</th>
						<th style="padding: 0.75rem; color: #94a3b8; font-size: 0.8rem">작업</th>
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
								{new Date(comment.created_at).toLocaleDateString('ko-KR')}
							</td>
							<td style="padding: 0.75rem">
								<div style="display: flex; gap: 0.5rem">
									<a href="/posts/{comment.post_id}" class="btn btn-sm btn-outline">보기</a>
									<form
										method="POST"
										action="?/deleteComment"
										style="margin: 0"
										use:enhance={({ cancel }) => {
											if (!confirm('이 댓글을 삭제하시겠습니까?')) {
												cancel();
											}
										}}
									>
										<input type="hidden" name="commentId" value={comment.id} />
										<button type="submit" class="btn btn-sm btn-danger">삭제</button>
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
