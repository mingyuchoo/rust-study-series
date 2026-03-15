import type { Actions, PageServerLoad } from './$types';
import { getPost, listComments, createComment, deletePost, deleteComment } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, parent, cookies }) => {
	const { user } = await parent();
	const authCookie = cookies.get('auth');
	const token = authCookie ? JSON.parse(authCookie).token ?? '' : '';

	try {
		const [post, commentsResult] = await Promise.all([
			getPost(params.id, token),
			listComments(params.id, token)
		]);
		return {
			post,
			comments: commentsResult.comments ?? [],
			user,
			error: null as string | null
		};
	} catch (e) {
		return {
			post: null,
			comments: [],
			user,
			error: `포스트를 불러올 수 없습니다: ${e}`
		};
	}
};

export const actions: Actions = {
	deletePost: async ({ params, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) {
			return fail(401, { error: '로그인이 필요합니다.' });
		}

		const { token } = JSON.parse(authCookie);

		try {
			await deletePost(token, params.id);
			throw redirect(303, '/');
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(500, { error: `포스트 삭제 실패: ${e}` });
		}
	},
	deleteComment: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) {
			return fail(401, { error: '로그인이 필요합니다.' });
		}

		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const commentId = data.get('commentId') as string;

		if (!commentId) {
			return fail(400, { error: '삭제할 댓글을 찾을 수 없습니다.' });
		}

		try {
			await deleteComment(token, commentId);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `댓글 삭제 실패: ${e}` });
		}
	},
	addComment: async ({ request, params, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) {
			return fail(401, { error: '로그인이 필요합니다.' });
		}

		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const content = (data.get('content') as string)?.trim();
		const visibility = data.get('visibility') ? 'public' : 'private';

		if (!content) {
			return fail(400, { error: '댓글 내용을 입력해주세요.' });
		}

		try {
			await createComment(token, params.id, content, visibility);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `댓글 작성 실패: ${e}` });
		}
	}
};
