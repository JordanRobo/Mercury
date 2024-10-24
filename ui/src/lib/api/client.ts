import Cookies from "js-cookie";
import { auth } from "$lib/stores/auth";
import { browser } from "$app/environment";
import { base } from "$app/paths";

// Get the base URL based on environment
const getBaseUrl = () => {
	if (!import.meta.env.PROD) {
		return "http://127.0.0.1:2030";
	}
	return base;
};

interface FetchOptions extends RequestInit {
	authenticated?: boolean;
}

export async function fetchClient(endpoint: string, options: FetchOptions = {}) {
	const { authenticated = true, ...fetchOptions } = options;
	const baseUrl = getBaseUrl();
	const url = `${baseUrl}${endpoint}`;

	// Prepare headers
	const headers = new Headers(fetchOptions.headers);
	headers.set("Content-Type", "application/json");

	// Add authorization header if authenticated request
	if (authenticated && browser) {
		const token = Cookies.get("jwt");
		if (token) {
			headers.set("Authorization", `Bearer ${token}`);
		}
	}

	try {
		const response = await fetch(url, {
			...fetchOptions,
			headers,
		});

		// Handle 401 responses by logging out
		if (response.status === 401) {
			auth.logout();
			throw new Error("Unauthorized");
		}

		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}

		// Only try to parse JSON if there's content
		const contentType = response.headers.get("content-type");
		if (contentType && contentType.includes("application/json")) {
			return await response.json();
		}

		return response;
	} catch (error) {
		console.error("API request failed:", error);
		throw error;
	}
}
