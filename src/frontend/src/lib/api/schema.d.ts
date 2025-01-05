/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
	'/admin/events': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Events List */
		get: operations['events'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/admin/presences/{event_id}/{round}': {
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
	'/admin/rounds/{event_id}': {
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
	'/events/available/{round}': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Available Events */
		get: operations['available'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/events/selected': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** Selected Events */
		get: operations['selected'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/events/set': {
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
		/** Set Event */
		patch: operations['set'];
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
	'/user/me': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** User Info */
		get: operations['me'];
		put?: never;
		post?: never;
		delete?: never;
		options?: never;
		head?: never;
		patch?: never;
		trace?: never;
	};
	'/user/my_type': {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		/** User Type */
		get: operations['my_type'];
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
		AdminEvent: {
			/** @example This is the description of event 1 */
			description: string;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Event 1 */
			name: string;
			/** @example Room 1 */
			room: string;
		};
		AdminEventResponse: {
			events: components['schemas']['AdminEvent'][];
		};
		AdminPresenceResponse: {
			/** @example Event 1 */
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
			/**
			 * Format: int64
			 * @example 10
			 */
			available_seats?: number | null;
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
		};
		AdminRoundResponse: {
			/** @example Event 1 */
			name: string;
			/** @example Room 1 */
			room: string;
			rounds: components['schemas']['AdminRound'][];
		};
		AdminSetPresenceRequest: {
			/**
			 * Format: int32
			 * @description The ID of the event
			 * @example 1
			 */
			event_id: number;
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
		AvailableEvent: {
			/**
			 * Format: int64
			 * @example 10
			 */
			available_seats?: number | null;
			/** @example This is the description of event 1 */
			description: string;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Event 1 */
			name: string;
			/** @example Room 1 */
			room: string;
			/**
			 * Format: int64
			 * @example 20
			 */
			total_seats: number;
		};
		AvailableEventResponse: {
			events: components['schemas']['AvailableEvent'][];
		};
		BasicSystemInfo: {
			system_host_name: string;
			system_kernel_version: string;
			system_name: string;
			system_os_version: string;
		};
		CpuInfo: {
			brand: string;
			frequency: string;
			name: string;
			/** Format: float */
			usage: number;
			vendor_id: string;
		};
		Event: {
			/**
			 * Format: int64
			 * @example 10
			 */
			available_seats?: number | null;
			/** Format: date-time */
			date: string;
			/** @example This is the description of event 1 */
			description?: string | null;
			/**
			 * Format: int32
			 * @example 1
			 */
			id: number;
			/** @example Event 1 */
			name: string;
			/** @example Room 1 */
			room?: string | null;
			/**
			 * Format: int32
			 * @description The round of the event (0-indexed)
			 * @example 0
			 */
			round: number;
			/**
			 * Format: int64
			 * @example 20
			 */
			total_seats: number;
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
			id: number;
			/** @example John Doe */
			name: string;
			/** @example false */
			present: boolean;
		};
		SelectedEventResponse: {
			events: components['schemas']['Event'][];
		};
		SetEventRequest: {
			/**
			 * Format: int32
			 * @description The id of the event to set to, do not provide to set absent on that
			 *     round
			 * @example 1
			 */
			event_id?: number | null;
			/**
			 * Format: int32
			 * @description The round to set the event to
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
		User: {
			/** @example john.doe@example.com */
			email: string;
			/** @example John Doe */
			name?: string | null;
		};
		/** @enum {string} */
		UserType: 'normal' | 'host' | 'admin';
		UserTypeResponse: {
			user_type: components['schemas']['UserType'];
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
	events: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description List of the events the user has access to, if admin all events, if host only those they are hosting */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AdminEventResponse'];
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
				 * @description The ID of the event
				 * @example 1
				 */
				event_id: number;
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
			/** @description List of the presences for a given event and round */
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
				 * @description The ID of the event
				 * @example 1
				 */
				event_id: number;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description List of the rounds for an event the user has access to */
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
	available: {
		parameters: {
			query?: never;
			header?: never;
			path: {
				/**
				 * @description The round of the event (0-indexed)
				 * @example 0
				 */
				round: number;
			};
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the available events */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['AvailableEventResponse'];
				};
			};
			/** @description Not logged in */
			401: {
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
			/** @description Returns the selected events */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['SelectedEventResponse'];
				};
			};
			/** @description Not logged in */
			401: {
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
				'application/json': components['schemas']['SetEventRequest'];
			};
		};
		responses: {
			/** @description The event was set successfully */
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
			/** @description Internal server error */
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
	me: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the user's info */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['User'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
	my_type: {
		parameters: {
			query?: never;
			header?: never;
			path?: never;
			cookie?: never;
		};
		requestBody?: never;
		responses: {
			/** @description Returns the user's type */
			200: {
				headers: {
					[name: string]: unknown;
				};
				content: {
					'application/json': components['schemas']['UserTypeResponse'];
				};
			};
			/** @description Not logged in */
			401: {
				headers: {
					[name: string]: unknown;
				};
				content?: never;
			};
		};
	};
}
