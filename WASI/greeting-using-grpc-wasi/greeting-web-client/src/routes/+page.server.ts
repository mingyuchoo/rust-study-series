import type { Actions, PageServerLoad } from './$types';
import { greet, getVersion } from '$lib/grpc';
import { fail } from '@sveltejs/kit';

export const load: PageServerLoad = async () => {
	try {
		const version = await getVersion();
		return { version, connectionError: null as string | null };
	} catch (e) {
		return { version: null as string | null, connectionError: String(e) };
	}
};

export const actions: Actions = {
	greet: async ({ request }) => {
		const data = await request.formData();
		const raw = data.get('name');

		if (typeof raw !== 'string') {
			return fail(400, { error: 'Invalid input', message: null as string | null });
		}

		const name = raw.trim();

		if (!name) {
			return fail(400, { error: 'Name is required', message: null as string | null });
		}

		try {
			const message = await greet(name);
			return { error: null as string | null, message };
		} catch (e) {
			return fail(500, { error: String(e), message: null as string | null });
		}
	}
};
