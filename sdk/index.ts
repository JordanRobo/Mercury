import { PostsAPI } from "./build";

const postsAPI = new PostsAPI();

const post = await postsAPI.getPost(1);

console.log(post);