import { readable, writable } from 'svelte/store';

export const internalBackendUrl = readable(import.meta.env.VITE_INTERNAL_BACKEND_BASE_URL);
export const externalBackendUrl = readable(import.meta.env.VITE_EXTERNAL_BACKEND_BASE_URL);
export const isLoggedIn = writable(false);
