import type { Actions } from './$types';
import { login, getMyProfile } from '$lib/grpc';
import { fail, redirect } from '@sveltejs/kit';
import { t, type Locale } from '$lib/i18n';

export const actions: Actions = {
	login: async ({ request, cookies }) => {
		const locale = (cookies.get('locale') ?? 'ko') as Locale;
		const data = await request.formData();
		const email = (data.get('email') as string)?.trim();
		const password = data.get('password') as string;

		if (!email || !password) {
			return fail(400, { error: t(locale, 'login.emailPasswordRequired') });
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

			try {
				const profile = await getMyProfile(result.token);
				const savedTheme = profile.theme || 'dark';
				const savedLocale = profile.locale || 'ko';
				cookies.set('theme', savedTheme, {
					path: '/',
					maxAge: 60 * 60 * 24 * 365,
					sameSite: 'strict'
				});
				cookies.set('locale', savedLocale, {
					path: '/',
					maxAge: 60 * 60 * 24 * 365,
					sameSite: 'strict'
				});
			} catch {
				cookies.set('theme', 'dark', {
					path: '/',
					maxAge: 60 * 60 * 24 * 365,
					sameSite: 'strict'
				});
			}

			throw redirect(303, '/');
		} catch (e) {
			if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
			return fail(401, { error: t(locale, 'login.invalidCredentials') });
		}
	},
	logout: async ({ cookies }) => {
		cookies.delete('auth', { path: '/' });
		throw redirect(303, '/');
	}
};
