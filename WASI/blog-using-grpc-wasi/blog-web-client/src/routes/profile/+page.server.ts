import type { PageServerLoad, Actions } from './$types';
import { getMyProfile, updateProfile, changePassword } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';
import { t, type Locale } from '$lib/i18n';

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
		const locale = (cookies.get('locale') ?? 'ko') as Locale;
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: t(locale, 'profile.loginRequired') });

		const auth = JSON.parse(authCookie);
		const data = await request.formData();
		const bio = (data.get('bio') as string) ?? '';
		const website = (data.get('website') as string)?.trim() ?? '';
		const theme = (data.get('theme') as string) ?? 'dark';
		const newLocale = (data.get('locale') as string) ?? 'ko';

		try {
			await updateProfile(auth.token, bio, website, theme, newLocale);
			cookies.set('theme', theme, {
				path: '/',
				maxAge: 60 * 60 * 24 * 365,
				sameSite: 'strict'
			});
			cookies.set('locale', newLocale, {
				path: '/',
				maxAge: 60 * 60 * 24 * 365,
				sameSite: 'strict'
			});
			return { success: t(newLocale as Locale, 'profile.saved') };
		} catch (e) {
			const msg = e instanceof Error ? e.message : t(locale, 'profile.saveFailed');
			return fail(400, { error: msg });
		}
	},
	changePassword: async ({ request, cookies }) => {
		const locale = (cookies.get('locale') ?? 'ko') as Locale;
		const authCookie = cookies.get('auth');
		if (!authCookie) return fail(401, { error: t(locale, 'profile.loginRequired') });

		const auth = JSON.parse(authCookie);
		const data = await request.formData();
		const currentPassword = data.get('current_password') as string;
		const newPassword = data.get('new_password') as string;
		const confirmPassword = data.get('confirm_password') as string;

		if (!currentPassword || !newPassword) {
			return fail(400, { passwordError: t(locale, 'profile.allFieldsRequired') });
		}
		if (newPassword !== confirmPassword) {
			return fail(400, { passwordError: t(locale, 'profile.passwordMismatch') });
		}

		try {
			const result = await changePassword(auth.token, currentPassword, newPassword);
			return { passwordSuccess: result.message };
		} catch (e) {
			const msg = e instanceof Error ? e.message : t(locale, 'profile.passwordChangeFailed');
			return fail(400, { passwordError: msg });
		}
	}
};
