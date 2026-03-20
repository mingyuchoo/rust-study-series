import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const theme = cookies.get('theme') ?? 'dark';
	const locale = cookies.get('locale') ?? 'ko';

	const authCookie = cookies.get('auth');
	if (authCookie) {
		try {
			const auth = JSON.parse(authCookie);
			return {
				user: {
					username: auth.username,
					id: auth.userId,
					role: auth.role ?? 'user'
				},
				theme,
				locale
			};
		} catch {
			return { user: null, theme, locale };
		}
	}
	return { user: null, theme, locale };
};
