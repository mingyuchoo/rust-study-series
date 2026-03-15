import type { Actions, PageServerLoad } from './$types';
import {
	listUsers,
	listPosts,
	listComments,
	updateUserRole,
	deleteUser,
	updatePostVisibility,
	deletePost,
	deleteComment,
	updateComment,
	type Comment,
	type Post
} from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

interface CommentWithPost extends Comment {
	post_title: string;
}

export const load: PageServerLoad = async ({ parent, cookies, url }) => {
	const { user } = await parent();
	if (!user || user.role !== 'admin') {
		throw redirect(303, '/');
	}

	const authCookie = cookies.get('auth');
	if (!authCookie) throw redirect(303, '/login');
	const { token } = JSON.parse(authCookie);

	const tab = url.searchParams.get('tab') ?? 'users';
	const page = Number(url.searchParams.get('page') ?? '1');

	try {
		const [usersResult, postsResult] = await Promise.all([
			listUsers(token, page, 20),
			listPosts(page, 50, token)
		]);

		const posts: Post[] = postsResult.posts ?? [];

		// 댓글 탭일 때 모든 포스트의 댓글을 병렬로 가져옴
		let allComments: CommentWithPost[] = [];
		if (tab === 'comments') {
			const commentResults = await Promise.all(
				posts.map(async (post) => {
					const comments = await listComments(post.id, token);
					return comments.map((c) => ({ ...c, post_title: post.title }));
				})
			);
			allComments = commentResults.flat();
		}

		return {
			users: usersResult.users ?? [],
			usersTotal: usersResult.total,
			posts,
			postsTotal: postsResult.total,
			comments: allComments,
			commentsTotal: allComments.length,
			tab,
			page,
			error: null as string | null
		};
	} catch (e) {
		return {
			users: [],
			usersTotal: 0,
			posts: [],
			postsTotal: 0,
			comments: [] as CommentWithPost[],
			commentsTotal: 0,
			tab,
			page,
			error: `데이터 로드 실패: ${e}`
		};
	}
};

export const actions: Actions = {
	updateRole: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const userId = data.get('userId') as string;
		const role = data.get('role') as string;

		try {
			await updateUserRole(token, userId, role);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `역할 변경 실패: ${e}` });
		}
	},
	deleteUser: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const userId = data.get('userId') as string;

		try {
			await deleteUser(token, userId);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `사용자 삭제 실패: ${e}` });
		}
	},
	updateVisibility: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const postId = data.get('postId') as string;
		const visibility = data.get('visibility') as string;

		try {
			await updatePostVisibility(token, postId, visibility);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `공개범위 변경 실패: ${e}` });
		}
	},
	deletePost: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const postId = data.get('postId') as string;

		try {
			await deletePost(token, postId);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `포스트 삭제 실패: ${e}` });
		}
	},
	updateComment: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const commentId = data.get('commentId') as string;
		const content = (data.get('content') as string)?.trim();

		if (!content) {
			return fail(400, { error: '댓글 내용을 입력해주세요.' });
		}

		try {
			await updateComment(token, commentId, content);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `댓글 수정 실패: ${e}` });
		}
	},
	deleteComment: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });
		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const commentId = data.get('commentId') as string;

		try {
			await deleteComment(token, commentId);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `댓글 삭제 실패: ${e}` });
		}
	}
};
