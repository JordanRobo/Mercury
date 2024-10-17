import { auth } from "../stores/auth";
import { base } from "$app/paths";

export async function login(email: String, password: String) {
	let app_base: String = base;
	if (!import.meta.env.PROD) {
		app_base = "http://127.0.0.1:2030";
	}

	const url = app_base + "/login";

	try {
		const response = await fetch(url, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ email, password }),
		});

		if (!response.ok) {
			throw new Error("Login failed");
		}

		const data = await response.json();
		auth.login(data.token);
		return true;
	} catch (error) {
		console.error("Login error:", error);
		return false;
	}
}
