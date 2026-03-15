import type { Actions } from './$types';
import { login, getMyProfile } from '$lib/grpc';
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

			// 서버에 저장된 테마를 쿠키에 반영
			try {
				const profile = await getMyProfile(result.token);
				const savedTheme = profile.theme || 'dark';
				cookies.set('theme', savedTheme, {
					path: '/',
					maxAge: 60 * 60 * 24 * 365,
					sameSite: 'strict'
				});
			} catch {
				// 프로필 조회 실패 시 기본 dark 적용
				cookies.set('theme', 'dark', {
					path: '/',
					maxAge: 60 * 60 * 24 * 365,
					sameSite: 'strict'
				});
			}

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
