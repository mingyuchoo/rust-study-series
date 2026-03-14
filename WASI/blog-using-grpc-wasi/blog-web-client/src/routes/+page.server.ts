import type { PageServerLoad } from './$types';
import { listPosts } from '$lib/grpc';

export const load: PageServerLoad = async ({ url }) => {
	const page = Number(url.searchParams.get('page') ?? '1');
	try {
		const result = await listPosts(page, 10);
		return { posts: result.posts ?? [], total: result.total, error: null as string | null };
	} catch (e) {
		return {
			posts: [] as Array<Record<string, unknown>>,
			total: 0,
			error: `gRPC 서버 연결 실패: ${e}. 서버를 먼저 실행하세요.`
		};
	}
};
