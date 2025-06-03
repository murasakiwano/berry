import { err, ok, Result, ResultAsync } from "neverthrow";
import type { ZodSchema } from "zod";

export type ErrResponse = { status: number; message: string };

export function formatCurrency(value: number | string): string {
	if (typeof value === "string") {
		value = parseFloat(value);

		if (isNaN(value)) {
			return "";
		}
	}

	return Intl.NumberFormat("pt-BR", {
		currency: "BRL",
		style: "currency",
		maximumFractionDigits: 2,
	}).format(value);
}

export function capitalize(s: string): string {
	if (s.length > 0) {
		const firstLetter = s.slice(0, 1);

		return firstLetter.toUpperCase() + s.substring(1);
	}

	return s;
}

export const readResponseAsJson: (res: Response) => ResultAsync<unknown, ErrResponse> =
	ResultAsync.fromThrowable(
		(res: Response) => res.json(),
		(e) => ({
			status: 500,
			message: `Failed to read response as json: ${e}`,
		}),
	);

export const parseBody: <T>(body: unknown, Schema: ZodSchema) => Result<T, ErrResponse> = (
	body,
	Schema,
) => {
	const parsed = Schema.safeParse(body);
	if (parsed.error) {
		return err({
			status: 400,
			message: parsed.error.toString(),
		});
	}

	return ok(parsed.data);
};

export const checkIfResponseIsOk = (res: Response): Result<Response, ErrResponse> => {
	if (!res.ok) {
		return err({ status: res.status, message: res.statusText });
	}

	return ok(res);
};

/**
 * Receives a currency (string), returns the value as a number.
 *
 * Remove non-number characters from a currency value, replace the comma for a dot
 * (in Brazil, ',' is used to separate decimal places).
 */
export const parseCurrency = (currency: string): number => {
	const replaced = currency.replace(/[^\d,-]/g, "").replace(/,/g, ".");

	return parseFloat(replaced);
};
