import type { PageServerLoad, Actions } from './$types';
import { db } from '$lib/db';

export const load: PageServerLoad = async ({ params }) => {
    const id = params.id;
    const post = await fetch(`${db}/posts/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        },
    });
    return {
        post: await post.json(),
    }
};

export const actions: Actions = {
    save: async ({ request }) => {
        const formData = await request.formData();
        const id = formData.get('id');
        const content = formData.get('content');

        const response = await fetch(`${db}/posts/${id}`, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ "content": content }),
        });
        if (!response.ok) {
            return {
                status: response.status,
                error: await response.text(),
            };
        }
    }
};