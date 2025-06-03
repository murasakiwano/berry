export const wrapApiError = (e: unknown, statusCode: number = 500) => {
	if (e instanceof Error) {
		return { status: statusCode, body: e.message };
	}

	return { status: statusCode, body: `Unknown error occurred: ${e}` };
};
