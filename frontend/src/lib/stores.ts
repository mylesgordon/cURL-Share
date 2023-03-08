import { readable, writable } from 'svelte/store';

export const backendUrl = readable(import.meta.env.VITE_BACKEND_BASE_URL);
export const isLoggedIn = writable(false);
