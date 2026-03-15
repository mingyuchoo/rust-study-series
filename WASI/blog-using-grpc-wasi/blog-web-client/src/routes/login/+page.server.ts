import type { Actions } from './$types';
import { login } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	login: async ({ request, cookies }) => {
		const data = await request.formData();
		const email = (data.get('email') as string)?.trim();
		const password = data.get('password') as string;

		if (!email || !password) {
			return fail(400, { error: '이메일과 비밀번호를 입력해주세요.' });
		}

		try {
			const result = await login(email, password);
			cookies.set(
				'auth',
				JSON.stringify({
					token: result.token,
					username: result.user.username,
					userId: result.user.id,
					role: result.user.role
				}),
				{ path: '/', httpOnly: true, sameSite: 'strict', maxAge: 60 * 60 * 24 }
			);
			throw redirect(303, '/');
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(401, { error: '이메일 또는 비밀번호가 올바르지 않습니다.' });
		}
	},
	logout: async ({ cookies }) => {
		cookies.delete('auth', { path: '/' });
		throw redirect(303, '/');
	}
};
