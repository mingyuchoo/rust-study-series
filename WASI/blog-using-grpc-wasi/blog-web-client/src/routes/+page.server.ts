import type { PageServerLoad } from './$types';
import { listPosts, type Post } from '$lib/grpc';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const page = Number(url.searchParams.get('page') ?? '1');
	const authCookie = cookies.get('auth');
	const token = authCookie ? JSON.parse(authCookie).token ?? '' : '';

	try {
		const result = await listPosts(page, 10, token);
		return { posts: result.posts ?? [], total: result.total, error: null as string | null };
	} catch (e) {
		return {
			posts: [] as Post[],
			total: 0,
			error: `gRPC 서버 연결 실패: ${e}. 서버를 먼저 실행하세요.`
		};
	}
};
