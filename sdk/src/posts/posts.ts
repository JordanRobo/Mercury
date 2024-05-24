import MercuryClient from "../client";

export interface Post {
    id:        number;
    title:     string;
    content:   string;
    authorid:  number;
    published: boolean;
}

class PostsAPI extends MercuryClient {
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

export default PostsAPI;