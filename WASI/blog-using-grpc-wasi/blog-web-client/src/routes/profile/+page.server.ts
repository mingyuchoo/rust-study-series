import type { PageServerLoad, Actions } from './$types';
import { getMyProfile, updateProfile, changePassword } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ cookies }) => {
	const authCookie = cookies.get('auth');
	if (!authCookie) {
		throw redirect(303, '/login');
	}

	try {
		const auth = JSON.parse(authCookie);
		const profile = await getMyProfile(auth.token);
		return { profile };
	} catch {
		throw redirect(303, '/login');
	}
};

export const actions: Actions = {
	updateProfile: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });

		const auth = JSON.parse(authCookie);
		const data = await request.formData();
		const bio = (data.get('bio') as string) ?? '';
		const website = (data.get('website') as string)?.trim() ?? '';
		const theme = (data.get('theme') as string) ?? 'dark';

		try {
			await updateProfile(auth.token, bio, website, theme);
			// 테마 쿠키도 함께 업데이트
			cookies.set('theme', theme, {
				path: '/',
				maxAge: 60 * 60 * 24 * 365,
				sameSite: 'strict'
			});
			return { success: '프로필이 저장되었습니다.' };
		} catch (e) {
			const msg = e instanceof Error ? e.message : '프로필 저장에 실패했습니다.';
			return fail(400, { error: msg });
		}
	},
	changePassword: async ({ request, cookies }) => {
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: '로그인이 필요합니다.' });

		const auth = JSON.parse(authCookie);
		const data = await request.formData();
		const currentPassword = data.get('current_password') as string;
		const newPassword = data.get('new_password') as string;
		const confirmPassword = data.get('confirm_password') as string;

		if (!currentPassword || !newPassword) {
			return fail(400, { passwordError: '모든 필드를 입력해주세요.' });
		}
		if (newPassword !== confirmPassword) {
			return fail(400, { passwordError: '새 비밀번호가 일치하지 않습니다.' });
		}

		try {
			const result = await changePassword(auth.token, currentPassword, newPassword);
			return { passwordSuccess: result.message };
		} catch (e) {
			const msg = e instanceof Error ? e.message : '비밀번호 변경에 실패했습니다.';
			return fail(400, { passwordError: msg });
		}
	}
};
