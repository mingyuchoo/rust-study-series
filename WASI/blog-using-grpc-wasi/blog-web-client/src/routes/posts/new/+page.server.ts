import type { Actions } from './$types';
import { createPost } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) {
			throw redirect(303, '/login');
		}

		const { token } = JSON.parse(authCookie);
		const data = await request.formData();
		const title = (data.get('title') as string)?.trim();
		const content = (data.get('content') as string)?.trim();
		const visibility = data.get('visibility') ? 'public' : 'private';

		if (!title || !content) {
			return fail(400, { error: '제목과 내용을 입력해주세요.' });
		}

		try {
			const post = await createPost(token, title, content, visibility);
			throw redirect(303, `/posts/${post.id}`);
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(500, { error: `포스트 작성 실패: ${e}` });
		}
	}
};
