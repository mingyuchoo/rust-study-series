import type { PageServerLoad } from './$types';
import { getStats, getVersion } from '$lib/grpc';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ cookies }) => {
	const authCookie = cookies.get('auth');
	if (!authCookie) {
		throw redirect(303, '/login');
	}

	try {
		const auth = JSON.parse(authCookie);
		if (auth.role !== 'admin') {
			throw redirect(303, '/');
		}

		const [stats, version] = await Promise.all([
			getStats(auth.token),
			getVersion()
		]);

		return { stats, version, error: null as string | null };
	} catch (e) {
		if (e instanceof Response || (e as { status?: number })?.status === 303) throw e;
		return {
			stats: { total_users: 0, total_posts: 0, total_comments: 0, public_posts: 0, private_posts: 0 },
			version: 'unknown',
			error: `${e}`
		};
	}
};
