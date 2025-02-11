/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
	'/activities/available/{round}': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Available Activities */
		get: operations['available'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/activities/selected': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Selected Activities */
		get: operations['selected'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/activities/set': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		get?: never;
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		/** Set Activity */
		patch: operations['set'];
		trace?: never;
	};
	'/admin/activities': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Activities List */
		get: operations['activities'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/admin/call_register': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		get?: never;
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		/** Call Register */
		patch: operations['call_register'];
		trace?: never;
	};
	'/admin/presences/{activity_id}/{round}': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Presences List */
		get: operations['presences'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/admin/rounds/{activity_id}': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Rounds List */
		get: operations['rounds'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/admin/set_presence': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		get?: never;
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		/** Set Presence */
		patch: operations['set_presence'];
		trace?: never;
	};
	'/auth/callback': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** OAuth2 callback endpoint */
		get: operations['google_oauth_callback_handler'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/auth/login': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Login with Google OAuth */
		get: operations['google_login_handler'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/auth/logout': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Logout */
		get: operations['logout_handler'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/healthcheck': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Healthcheck */
		get: operations['healthcheck'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/registrations_start_date': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Registrations Start Date */
		get: operations['registrations_start_date'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/sys_info': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** System Info */
		get: operations['sys_info'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
}
export type webhooks = Record<string, never>;
export interface components {
	schemas: {
		Activity: {
			/** Format: date-time */
			date: string;
			/** @example This is the description of activity 1 */
			description?: string | null;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Activity 1 */
			name: string;
			/** @example true */
			present: boolean;
			/** @example Room 1 */
			room: string;
			/**
			 * Format: int32
			 * @description The round of the activity (0-indexed)
			 * @example 0
			 */
			round: number;
			/**
			 * Format: int64
			 * @example 20
			 */
			total_seats: number;
			/**
			 * Format: int64
			 * @example 10
			 */
			used_seats: number;
		};
		AdminActivity: {
			/** @example This is the description of activity 1 */
			description: string;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Activity 1 */
			name: string;
			/** @example Room 1 */
			room: string;
		};
		AdminActivityResponse: {
			activities: components['schemas']['AdminActivity'][];
		};
		AdminPresenceResponse: {
			/** Format: date-time */
			date: string;
			/** @example Activity 1 */
			name: string;
			presences: components['schemas']['Presence'][];
			/** @example Room 1 */
			room: string;
			/**
			 * Format: int32
			 * @example 20
			 */
			total_seats: number;
		};
		AdminRound: {
			/** Format: date-time */
			date: string;
			/**
			 * Format: int32
			 * @example 0
			 */
			round: number;
			/**
			 * Format: int64
			 * @example 20
			 */
			total_seats: number;
			/**
			 * Format: int64
			 * @example 10
			 */
			used_seats?: number | null;
		};
		AdminRoundResponse: {
			/** @example Activity 1 */
			name: string;
			/** @example Room 1 */
			room: string;
			rounds: components['schemas']['AdminRound'][];
		};
		AdminSetPresenceRequest: {
			/**
			 * Format: int32
			 * @description The ID of the activity
			 * @example 1
			 */
			activity_id: number;
			/**
			 * @description Whether the user is present
			 * @example true
			 */
			present: boolean;
			/**
			 * Format: int32
			 * @description The round number
			 * @example 1
			 */
			round: number;
			/**
			 * Format: int32
			 * @description The ID of the user
			 * @example 1
			 */
			user_id: number;
		};
		AvailableActivity: {
			/** @example This is the description of the first activity */
			description: string;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Activity 1 */
			name: string;
			/** @example Room 1 */
			room: string;
			/**
			 * Format: int64
			 * @example 20
			 */
			total_seats: number;
			/**
			 * Format: int64
			 * @example 10
			 */
			used_seats?: number | null;
		};
		AvailableActivityResponse: {
			activities: components['schemas']['AvailableActivity'][];
		};
		BasicSystemInfo: {
			system_host_name: string;
			system_kernel_version: string;
			system_name: string;
			system_os_version: string;
		};
		CallRegisterRequest: {
			/**
			 * Format: int32
			 * @description The ID of the activity
			 * @example 1
			 */
			activity_id: number;
			/**
			 * Format: int32
			 * @description The round number
			 * @example 1
			 */
			round: number;
		};
		CpuInfo: {
			brand: string;
			frequency: string;
			name: string;
			/** Format: float */
			usage: number;
			vendor_id: string;
		};
		MemInfo: {
			free: string;
			total: string;
			used: string;
		};
		Presence: {
			/**
			 * Format: int32
			 * @example 1
			 */
			class: number;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example John Doe */
			name: string;
			/** @example false */
			present: boolean;
			/** @example false */
			randomized: boolean;
			/** @example A */
			section?: string | null;
		};
		RegistrationsStartDateResponse: {
			/** Format: date-time */
			registrations_start_date: string;
		};
		SelectedActivityResponse: {
			activities: components['schemas']['Activity'][];
			/** Format: date-time */
			registrations_end_date: string;
		};
		SetActivityRequest: {
			/**
			 * Format: int32
			 * @description The id of the activity to set to, do not provide to set absent on that
			 *     round
			 * @example 1
			 */
			activity_id?: number | null;
			/**
			 * Format: int32
			 * @description The round to set the activity to
			 * @example 0
			 */
			round: number;
		};
		SwapInfo: {
			free: string;
			total: string;
			used: string;
		};
		SystemInfoResponse: {
			basic: components['schemas']['BasicSystemInfo'];
			cpu_info: components['schemas']['CpuInfo'];
			memory: components['schemas']['MemInfo'];
			swap: components['schemas']['SwapInfo'];
		};
	};
	responses: never;
	parameters: never;
	requestBodies: never;
	headers: never;
	pathItems: never;
}
export type $defs = Record<string, never>;
export interface operations {
	available: {
		parameters: {
			query?: never;
			header?: never;
			path: {
				/**
				 * @description The round of the activity (0-indexed)
				 * @example 0
				 */
				round: number;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the available activities */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AvailableActivityResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have ended */
			410: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	selected: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the selected activities */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['SelectedActivityResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description You are an admin */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have ended */
			410: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	set: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody: {
			content: {
				'application/json': components['schemas']['SetActivityRequest'];
			};
		};
		responses: {
			/** @description The activity was set successfully */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description The activity is full */
			409: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have ended */
			410: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	activities: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description List of the activities the user has access to, if admin all activities, if host only those they are hosting */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AdminActivityResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not an admin or host */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	call_register: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody: {
			content: {
				'application/json': components['schemas']['CallRegisterRequest'];
			};
		};
		responses: {
			/** @description Last edited by updated */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not an admin or host */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	presences: {
		parameters: {
			query?: never;
			header?: never;
			path: {
				/**
				 * @description The ID of the activity
				 * @example 1
				 */
				activity_id: number;
				/**
				 * @description The round number
				 * @example 1
				 */
				round: number;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description List of the presences for a given activity and round */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AdminPresenceResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not an admin or host */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	rounds: {
		parameters: {
			query?: never;
			header?: never;
			path: {
				/**
				 * @description The ID of the activity
				 * @example 1
				 */
				activity_id: number;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description List of the rounds for an activity the user has access to */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AdminRoundResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not an admin or host */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	set_presence: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody: {
			content: {
				'application/json': components['schemas']['AdminSetPresenceRequest'];
			};
		};
		responses: {
			/** @description Presence set */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Not an admin or host */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Registrations have not started yet */
			425: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Internal server error */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	google_oauth_callback_handler: {
		parameters: {
			query?: never;
			header?: never;
			path: {
				code: string;
				state: string;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Redirect to Auth Success page, with a message in the reason query param */
			303: {
				headers: {
					/** @description Session cookie */
					'Set-Cookie'?: string;
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description csrf_state not found in session */
			400: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Invalid CSRF state */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Invalid email domain */
			403: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Failed to authenticate user */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	google_login_handler: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Redirect to Google OAuth */
			303: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	logout_handler: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Successfully logged out */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
			/** @description Failed to logout user, user may be not logged in */
			500: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	healthcheck: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Success */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'text/plain': string;
				};
			};
		};
	};
	registrations_start_date: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the start date of the registrations */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['RegistrationsStartDateResponse'];
				};
			};
		};
	};
	sys_info: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description System info */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['SystemInfoResponse'];
				};
			};
		};
	};
}
