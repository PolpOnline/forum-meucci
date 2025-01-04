/**
 * This file was auto-generated by openapi-typescript.
 * Do not make direct changes to the file.
 */

export interface paths {
    "/auth/callback": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["google_oauth_callback_handler"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/auth/login": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["google_login_handler"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/auth/logout": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["logout_handler"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/events/available/{round}": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["available"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/events/selected": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["selected"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/events/set": {
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
        patch: operations["set"];
        trace?: never;
    };
    "/healthcheck": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["healthcheck"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/sys_info": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["sys_info"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/user/me": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["me"];
        put?: never;
        post?: never;
        delete?: never;
        options?: never;
        head?: never;
        patch?: never;
        trace?: never;
    };
    "/user/my_type": {
        parameters: {
            query?: never;
            header?: never;
            path?: never;
            cookie?: never;
        };
        get: operations["my_type"];
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
            events: components["schemas"]["AvailableEvent"][];
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
        SelectedEventResponse: {
            events: components["schemas"]["Event"][];
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
            basic: components["schemas"]["BasicSystemInfo"];
            cpu_info: components["schemas"]["CpuInfo"];
            memory: components["schemas"]["MemInfo"];
            swap: components["schemas"]["SwapInfo"];
        };
        User: {
            /** @example john.doe@example.com */
            email: string;
            /** @example John Doe */
            name?: string | null;
        };
        /** @enum {string} */
        UserType: "normal" | "host" | "admin";
        UserTypeResponse: {
            user_type: components["schemas"]["UserType"];
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
                    "Set-Cookie"?: string;
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
                    "application/json": components["schemas"]["AvailableEventResponse"];
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
                    "application/json": components["schemas"]["SelectedEventResponse"];
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
                "application/json": components["schemas"]["SetEventRequest"];
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
                    "text/plain": string;
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
                    "application/json": components["schemas"]["SystemInfoResponse"];
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
                    "application/json": components["schemas"]["User"];
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
                    "application/json": components["schemas"]["UserTypeResponse"];
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
