import type { Actions, PageServerLoad } from './$types';
import { getPost, listComments, createComment } from '$lib/grpc';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, parent }) => {
	const { user } = await parent();
	try {
		const [post, comments] = await Promise.all([
			getPost(params.id),
			listComments(params.id)
		]);
		return {
			post,
			comments: comments ?? [],
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
	addComment: async ({ request, params, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) {
			return fail(401, { error: '로그인이 필요합니다.' });
		}

		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const content = (data.get('content') as string)?.trim();

		if (!content) {
			return fail(400, { error: '댓글 내용을 입력해주세요.' });
		}

		try {
			await createComment(token, params.id, content);
			return { error: null };
		} catch (e) {
			return fail(500, { error: `댓글 작성 실패: ${e}` });
		}
	}
};
