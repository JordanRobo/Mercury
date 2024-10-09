import type { PageServerLoad, Actions } from './$types';
import { db } from '$lib/db';
import { fail } from '@sveltejs/kit';

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
        try {
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

            return { status: response.status, success: true};
        } catch (error) {
            return fail(404, { message: 'Failed to save post', success: false});
        }
    }
};
