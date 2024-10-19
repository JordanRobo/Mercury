import { auth, adminExists } from "../stores/auth";
import { base } from "$app/paths";

export async function login(email: string, password: string) {
	let app_base: string = base;
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

export async function adminCheck() {
	let app_base: string = base;
	if (!import.meta.env.PROD) {
		app_base = "http://127.0.0.1:2030";
	}

	const url = app_base + "/check";

	try {
		const resp = await fetch(url, {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
			},
		});

		if (!resp.ok) {
			adminExists.set(true);
		}
	} catch (error) {
		console.error("Admin check error:", error);
	}
}

export async function register(name: string, email: string, pass: string) {
	let app_base: string = base;
	if (!import.meta.env.PROD) {
		app_base = "http://127.0.0.1:2030";
	}

	const url = app_base + "/check";

	try {
		const response = await fetch(url, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
			},
			body: JSON.stringify({ name, email, pass }),
		});

		if (!response.ok) {
			throw new Error("Error creating admin");
		}
		return true;
	} catch (error) {
		console.error("Login error:", error);
		return false;
	}
}
