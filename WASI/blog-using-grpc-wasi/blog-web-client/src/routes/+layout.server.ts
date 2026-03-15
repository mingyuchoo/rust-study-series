import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const authCookie = cookies.get('auth');
	if (authCookie) {
		try {
			const auth = JSON.parse(authCookie);
			return {
				user: {
					username: auth.username,
					id: auth.userId,
					role: auth.role ?? 'user'
				}
			};
		} catch {
			return { user: null };
		}
	}
	return { user: null };
};
