import axios from 'axios';
import type { AxiosInstance, AxiosRequestConfig } from 'axios';
import type { Post } from './types';

const MERCURY_API_URL = Bun.env.MERCURY_API_URL || 'http://127.0.0.1:2323/api';

class MercuryClient {
    protected client: AxiosInstance;

    constructor(options: AxiosRequestConfig = {}) {
        this.client = axios.create({
            baseURL: MERCURY_API_URL,
            ...options,
        });
    }

    async getPosts(): Promise<Post[]> {
        try {
            const response = await this.client.get<Post[]>('/posts');
            return response.data;
        } catch (error) {
            console.error('Error fetching posts:', error);
            throw error;
        }
    }

    async getPost(id: number): Promise<Post> {
        try {
            const response = await this.client.get<Post>(`/posts/${id}`);
            return response.data;
        } catch (error) {
            console.error('Error fetching post:', error);
            throw error;
        }
    }

    async createPost(post: Post): Promise<Post> {
        try {
            const response = await this.client.post<Post>('/posts', post);
            return response.data;
        } catch (error) {
            console.error('Error creating post:', error);
            throw error;
        }
    }

    async updatePost(post: Post): Promise<Post> {
        try {
            const response = await this.client.patch<Post>(`/posts/${post.id}`, post);
            return response.data;
        } catch (error) {
            console.error('Error updating post:', error);
            throw error;
        }
    }

    async deletePost(id: number): Promise<void> {
        try {
            await this.client.delete(`/posts/${id}`);
        } catch (error) {
            console.error('Error deleting post:', error);
            throw error;
        }
    }
}

export default MercuryClient;