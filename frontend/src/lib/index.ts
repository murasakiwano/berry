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
