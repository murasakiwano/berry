import { err, ok, Result, ResultAsync } from "neverthrow";
import type { ZodSchema } from "zod";

export function formatCurrency(value: number): string {
	return Intl.NumberFormat("pt-BR", {
		currency: "BRL",
		style: "currency",
	}).format(value);
}

export function capitalize(s: string): string {
	if (s.length > 0) {
		const firstLetter = s.slice(0, 1);

		return firstLetter.toUpperCase() + s.substring(1);
	}

	return s;
}

export const readResponseAsJson = ResultAsync.fromThrowable(
	(res: Response) => res.json(),
	(e) => ({
		status: 500,
		message: `Failed to read response as json: ${e}`,
	}),
);

export const parseBody: <T>(
	body: unknown,
	Schema: ZodSchema,
) => Result<T, { status: number; message: string }> = (body, Schema) => {
	const parsed = Schema.safeParse(body);
	if (parsed.error) {
		return err({ status: 400, message: parsed.error.toString() });
	}

	return ok(parsed.data);
};

export const checkIfResponseIsOk = (res: Response) => {
	if (!res.ok) {
		return err({ status: res.status, message: res.statusText });
	}

	return ok(res);
};
