export function capitalizeFirstLetter(str: string): string {
	return str.charAt(0).toUpperCase() + str.slice(1);
}

export function fromEmailEstimateName(email: string): string {
	return (email.toString().split('@')[0] || '')
		.split('.')
		.map((part) => capitalizeFirstLetter(part))
		.join(' ');
}
