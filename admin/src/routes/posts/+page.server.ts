import type { PageServerLoad } from './$types';
import { db } from '$lib/db';

export const load: PageServerLoad = async () => {
    const posts = await fetch(`${db}/posts`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
    });
    return {
        posts: await posts.json(),
    }
};
