import type { Actions, PageServerLoad } from './$types';
import { getPost, updatePost } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, parent, cookies }) => {
	const { user } = await parent();

	if (!user) {
		throw redirect(303, '/login');
	}

	const authCookie = cookies.get('auth');
	const token = authCookie ? JSON.parse(authCookie).token ?? '' : '';

	try {
		const post = await getPost(params.id, token);

		// 작성자이거나 admin인 경우만 수정 가능
		if (post.author.id !== user.id && user.role !== 'admin') {
			throw redirect(303, `/posts/${params.id}`);
		}

		return { post, user };
	} catch (e) {
		if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
		return { post: null, user, error: `포스트를 불러올 수 없습니다: ${e}` };
	}
};

export const actions: Actions = {
	updatePost: async ({ request, params, cookies }) => {
		const authCookie = cookies.get('auth');
		const data = await request.formData();
		const title = (data.get('title') as string)?.trim() ?? '';
		const content = (data.get('content') as string)?.trim() ?? '';

		if (!authCookie) {
			return fail(401, { error: '로그인이 필요합니다.', title, content });
		}

		const { token } = JSON.parse(authCookie);

		if (!title || !content) {
			return fail(400, { error: '제목과 내용을 모두 입력해주세요.', title, content });
		}

		try {
			await updatePost(token, params.id, title, content);
			throw redirect(303, `/posts/${params.id}`);
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(500, { error: `포스트 수정 실패: ${e}`, title, content });
		}
	}
};
