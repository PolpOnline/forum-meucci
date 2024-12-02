import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type AvailableEventItem = {
	id: number;
	name: string;
	description: string;
	room: string;
	availableSeats: number;
	totalSeats: number;
};
