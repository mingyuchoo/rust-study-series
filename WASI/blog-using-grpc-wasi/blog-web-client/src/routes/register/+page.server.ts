import type { Actions } from './$types';
import { register } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	default: async ({ request, cookies }) => {
		const data = await request.formData();
		const username = (data.get('username') as string)?.trim();
		const email = (data.get('email') as string)?.trim();
		const password = data.get('password') as string;

		if (!username || !email || !password) {
			return fail(400, { error: '모든 필드를 입력해주세요.' });
		}

		if (password.length < 8) {
			return fail(400, { error: '비밀번호는 8자 이상이어야 합니다.' });
		}

		try {
			const result = await register(username, email, password);
			cookies.set(
				'auth',
				JSON.stringify({ token: result.token, username: result.user.username, userId: result.user.id }),
				{ path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 60 * 24 }
			);
			throw redirect(303, '/');
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(409, { error: '이미 존재하는 사용자명 또는 이메일입니다.' });
		}
	}
};
