// lib/stores/auth.ts
import { writable } from "svelte/store";
import { jwtDecode } from "jwt-decode";
import type { JWTPayload } from "$lib/types/auth";
import Cookies from "js-cookie";
import { goto } from "$app/navigation";
import { browser } from "$app/environment";

interface AuthState {
	isAuthenticated: boolean;
	isLoading: boolean;
	user: JWTPayload["sub"] | null;
	site: string | null;
}

const createAuthStore = () => {
	const { subscribe, set, update } = writable<AuthState>({
		isAuthenticated: false,
		isLoading: true,
		user: null,
		site: null,
	});

	return {
		subscribe,
		setToken: async (token: string) => {
			try {
				const decoded = jwtDecode<JWTPayload>(token);
				Cookies.set("jwt", token, {
					expires: new Date(decoded.exp * 1000),
					secure: true,
					sameSite: "strict",
				});
				set({
					isAuthenticated: true,
					isLoading: false,
					user: decoded.sub,
					site: decoded.site,
				});

				// Only redirect if we're in the browser and not already on dashboard
				if (browser && window.location.pathname !== "/dashboard") {
					await goto("/dashboard");
				}
			} catch (error) {
				console.error("Invalid token:", error);
				Cookies.remove("jwt");
				set({
					isAuthenticated: false,
					isLoading: false,
					user: null,
					site: null,
				});
			}
		},
		logout: async () => {
			Cookies.remove("jwt");
			set({
				isAuthenticated: false,
				isLoading: false,
				user: null,
				site: null,
			});

			// Only redirect if we're in the browser and not already on the root page
			if (browser && window.location.pathname !== "/") {
				await goto("/");
			}
		},
		checkAuth: async () => {
			const token = Cookies.get("jwt");
			if (token) {
				try {
					const decoded = jwtDecode<JWTPayload>(token);
					if (decoded.exp * 1000 > Date.now()) {
						set({
							isAuthenticated: true,
							isLoading: false,
							user: decoded.sub,
							site: decoded.site,
						});

						// If we're on the root page and authenticated, redirect to dashboard
						if (browser && window.location.pathname === "/") {
							await goto("/dashboard");
						}
					} else {
						Cookies.remove("jwt");
						set({
							isAuthenticated: false,
							isLoading: false,
							user: null,
							site: null,
						});
					}
				} catch (error) {
					console.error("Invalid token:", error);
					Cookies.remove("jwt");
					set({
						isAuthenticated: false,
						isLoading: false,
						user: null,
						site: null,
					});
				}
			} else {
				set({
					isAuthenticated: false,
					isLoading: false,
					user: null,
					site: null,
				});
			}
		},
	};
};

export const auth = createAuthStore();
