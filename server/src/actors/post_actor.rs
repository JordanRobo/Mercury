use crate::db::{ Post, NewPost };
use crate::db::DbActor;
use crate::db::posts::dsl::*;
use crate::messages::{ FetchPosts, FetchPost, CreatePost, UpdatePost, DeletePost };
use actix::Handler;
use diesel::{ self, prelude::* };

impl Handler<FetchPosts> for DbActor {
    type Result = QueryResult<Vec<Post>>;

    fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch Post: Unable to establish connection");

        posts.get_results::<Post>(&mut conn)
    }
}

impl Handler<FetchPost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: FetchPost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch Post: Unable to establish connection");

        posts.find(msg.id).get_result::<Post>(&mut conn)
    }
}

impl Handler<CreatePost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: CreatePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        
        let new_post = NewPost {
            title: msg.title.unwrap_or("".to_string()),
            content: msg.content.unwrap_or("".to_string()),
            authorid: msg.authorid.unwrap_or_default(),
        };
        
        diesel::insert_into(posts)
            .values(new_post)
            .returning((id, title, content, authorid, published))
            .get_result::<Post>(&mut conn)
    }
}

impl Handler<UpdatePost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: UpdatePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        let post = posts.find(msg.id).get_result::<Post>(&mut conn).expect("Failed to find post");

        diesel::update(posts.find(msg.id))
            .set((
                title.eq(msg.title.unwrap_or(post.title)),
                content.eq(msg.content.unwrap_or(post.content)),
                authorid.eq(msg.authorid.unwrap_or(post.authorid)),
                published.eq(msg.published.unwrap_or(post.published.is_some()))
            ))
            .returning((id, title, content, authorid, published))
            .get_result::<Post>(&mut conn)
    }   
}

impl Handler<DeletePost> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeletePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(posts.find(msg.id))
        .execute(&mut conn)
    }
}