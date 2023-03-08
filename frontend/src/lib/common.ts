export function splitAndTrim(input: string | undefined): Array<string> {
	if (!input) {
		return [];
	}

	return input.split(',').map((item) => item.trim());
}
