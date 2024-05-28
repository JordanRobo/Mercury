use crate::db::DbActor;
use crate::db::{Author, NewPost, Post, Tag};
use crate::db::{authors, posts, post_tags, tags};
use crate::messages::post_messages::*;
use actix::prelude::*;
use diesel::prelude::*;

impl Handler<FetchPosts> for DbActor {
    type Result = QueryResult<Vec<(Post, Author, Vec<Tag>)>>;

    fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");

        let data: Vec<(Post, Author, Vec<Tag>)> = posts::table
            .inner_join(authors::table)
            .load::<(Post, Author)>(&mut conn)?
            .into_iter()
            .map(|(post, author)| {
                let tags = post_tags::table
                    .inner_join(tags::table)
                    .filter(post_tags::post_id.eq(post.id))
                    .select(tags::all_columns)
                    .load(&mut conn)
                    .unwrap_or_else(|_| Vec::new());
                (post, author, tags)
            })
            .collect();

        Ok(data)
    }
}

impl Handler<FetchPost> for DbActor {
    type Result = QueryResult<(Post, Author, Vec<Tag>)>;

    fn handle(&mut self, msg: FetchPost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");

        let (post, author) = posts::table
            .inner_join(authors::table)
            .filter(posts::id.eq(msg.id))
            .first::<(Post, Author)>(&mut conn)?;

        let tags = post_tags::table
            .inner_join(tags::table)
            .filter(post_tags::post_id.eq(post.id))
            .select(tags::all_columns)
            .load(&mut conn)
            .unwrap_or_else(|_| Vec::new());

        Ok((post, author, tags))
    }
}

impl Handler<CreatePost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: CreatePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");

        let new_post = NewPost {
            title: msg.title.unwrap_or("".to_string()),
            slug: msg.slug.unwrap_or("".to_string()),
            content: msg.content.unwrap_or("".to_string()),
            feature_image: msg.feature_image,
            excerpt: msg.excerpt,
            published: msg.published,
            author_id: Some(msg.author_id)
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(&mut conn)
    }
}

impl Handler<UpdatePost> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: UpdatePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::update(posts::table.find(msg.id))
            .set((
                posts::title.eq(msg.title.unwrap_or("".to_string())),
                posts::slug.eq(msg.slug.unwrap_or("".to_string())),
                posts::content.eq(msg.content.unwrap_or("".to_string())),
                posts::feature_image.eq(msg.feature_image),
                posts::excerpt.eq(msg.excerpt),
                posts::published.eq(msg.published),
                posts::author_id.eq(msg.author_id)
            ))
            .get_result(&mut conn)
    }
}

impl Handler<DeletePost> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeletePost, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(posts::table.find(msg.id))
            .execute(&mut conn)
    }
}

impl Handler<DeletePosts> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeletePosts, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(posts::table.filter(posts::id.eq_any(msg.ids)))
            .execute(&mut conn)
    }
}