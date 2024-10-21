import { writable } from "svelte/store";
import { jwtDecode } from "jwt-decode";
import type { JWTPayload } from "$lib/types/auth";
import Cookies from "js-cookie";

interface AuthState {
	isAuthenticated: boolean;
	user: JWTPayload["sub"] | null;
	site: string | null;
}

const createAuthStore = () => {
	const { subscribe, set, update } = writable<AuthState>({
		isAuthenticated: false,
		user: null,
		site: null,
	});

	return {
		subscribe,
		setToken: (token: string) => {
			try {
				const decoded = jwtDecode<JWTPayload>(token);
				Cookies.set("jwt", token, {
					expires: new Date(decoded.exp * 1000),
					secure: true,
					sameSite: "strict",
				});
				set({
					isAuthenticated: true,
					user: decoded.sub,
					site: decoded.site,
				});
			} catch (error) {
				console.error("Invalid token:", error);
				Cookies.remove("jwt");
				set({
					isAuthenticated: false,
					user: null,
					site: null,
				});
			}
		},
		logout: () => {
			Cookies.remove("jwt");
			set({
				isAuthenticated: false,
				user: null,
				site: null,
			});
		},
		checkAuth: () => {
			const token = Cookies.get("jwt");
			if (token) {
				try {
					const decoded = jwtDecode<JWTPayload>(token);
					if (decoded.exp * 1000 > Date.now()) {
						set({
							isAuthenticated: true,
							user: decoded.sub,
							site: decoded.site,
						});
					} else {
						Cookies.remove("jwt");
						set({
							isAuthenticated: false,
							user: null,
							site: null,
						});
					}
				} catch (error) {
					console.error("Invalid token:", error);
					Cookies.remove("jwt");
					set({
						isAuthenticated: false,
						user: null,
						site: null,
					});
				}
			}
		},
	};
};

export const auth = createAuthStore();
