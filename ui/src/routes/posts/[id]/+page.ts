import type { PageLoad } from "./$types";
import { base } from "$app/paths";
import Cookies from "js-cookie";

export const load: PageLoad = async ({ params }) => {
	let app_base: string = base;
	if (!import.meta.env.PROD) {
		app_base = "http://127.0.0.1:2030";
	}

	const url = app_base + `/admin/posts/${params.id}`;
	const token = Cookies.get("jwt");

	try {
		const response = await fetch(url, {
			method: "GET",
			headers: {
				"Content-Type": "application/json",
				Authorization: `Bearer ${token}`,
			},
		});

		if (!response.ok) {
			throw new Error("Login failed");
		}
	} catch (error) {
		console.error("Login error:", error);
	}
};
