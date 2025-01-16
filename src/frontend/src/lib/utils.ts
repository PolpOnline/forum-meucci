import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';
import type { components } from '$lib/api/schema';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type AvailableActivity = components['schemas']['AvailableActivity'];
export type AvailableActivityResponse = components['schemas']['AvailableActivityResponse'];
