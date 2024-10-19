import { writable } from "svelte/store";

function createAuthStore() {
	const { subscribe, set, update } = writable({
		isAuthenticated: false,
		token: null,
		isLoading: true,
	});

	return {
		subscribe,
		login: (token) => {
			localStorage.setItem("token", token);
			set({ isAuthenticated: true, token, isLoading: false });
		},
		logout: () => {
			localStorage.removeItem("token");
			set({ isAuthenticated: false, token: null, isLoading: false });
		},
		checkAuth: () => {
			const token = localStorage.getItem("token");
			if (token) {
				set({ isAuthenticated: true, token, isLoading: false });
			} else {
				set({ isAuthenticated: false, token: null, isLoading: false });
			}
		},
	};
}

export const auth = createAuthStore();

export const adminExists = writable(false);
