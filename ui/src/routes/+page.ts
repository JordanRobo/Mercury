import type { PageLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { browser } from "$app/environment";

export const load: PageLoad = async () => {
	// Only redirect on client-side to avoid SSR issues
	if (browser) {
		throw redirect(307, "/dashboard");
	}
};
