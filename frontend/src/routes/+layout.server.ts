import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
	const theme = cookies.get('berry-theme') ?? 'light';

	return {
		theme
	};
};