export function splitAndTrim(input: string): Array<string> {
		return input.split(',').map((item) => item.trim());
	}