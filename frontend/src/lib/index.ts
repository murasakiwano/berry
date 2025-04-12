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
