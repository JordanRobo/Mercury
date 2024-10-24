import { auth } from "../stores/auth";
import { fetchClient } from "./client";
import { toast } from "svelte-sonner";

// Define response types
interface LoginResponse {
	token: string;
}

// Define possible error types
export type LoginError = "INVALID_CREDENTIALS" | "USER_NOT_FOUND" | "ACCOUNT_LOCKED" | "SERVER_ERROR" | "NETWORK_ERROR";

// Define error messages
const ERROR_MESSAGES: Record<LoginError, string> = {
	INVALID_CREDENTIALS: "Invalid email or password",
	USER_NOT_FOUND: "No account found with this email",
	ACCOUNT_LOCKED: "Account temporarily locked. Please try again later",
	SERVER_ERROR: "Server error occurred. Please try again later",
	NETWORK_ERROR: "Network error. Please check your connection",
};

export async function login(email: string, password: string): Promise<boolean> {
	try {
		const response = await fetchClient<LoginResponse>("/login", {
			method: "POST",
			authenticated: false, // Login doesn't need auth header
			body: JSON.stringify({ email, password }),
		});

		if (response.token) {
			await auth.setToken(response.token);
			toast.success("Welcome back!");
			return true;
		}

		return false;
	} catch (error) {
		let errorType: LoginError = "SERVER_ERROR";

		if (error instanceof Error) {
			// Handle specific error cases
			if (error.message.includes("401")) {
				errorType = "INVALID_CREDENTIALS";
			} else if (error.message.includes("404")) {
				errorType = "USER_NOT_FOUND";
			} else if (error.message.includes("423")) {
				errorType = "ACCOUNT_LOCKED";
			} else if (error.message.includes("network")) {
				errorType = "NETWORK_ERROR";
			}
		}

		toast.error(ERROR_MESSAGES[errorType]);
		console.error("Login error:", error);
		return false;
	}
}

// Add more auth-related functions here
export const auth_api = {
	login,
	// requestPasswordReset: (email: string) => {...},
	// validateResetToken: (token: string) => {...},
	// resetPassword: (token: string, newPassword: string) => {...},
};
