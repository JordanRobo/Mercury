use crate::db::{ Author, NewAuthor };
use crate::db::DbActor;
use crate::db::authors::dsl::*;
use crate::messages::{ FetchAuthors, FetchAuthor, CreateAuthor, UpdateAuthor, DeleteAuthor };
use actix::Handler;
use diesel::{ self, prelude::* };

impl Handler<FetchAuthors> for DbActor {
    type Result = QueryResult<Vec<Author>>;

    fn handle(&mut self, _msg: FetchAuthors, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch Author: Unable to establish connection");

        authors.get_results::<Author>(&mut conn)
    }
}

impl Handler<FetchAuthor> for DbActor {
    type Result = QueryResult<Author>;

    fn handle(&mut self, msg: FetchAuthor, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

        authors.find(msg.id).get_result::<Author>(&mut conn)
    }
}

impl Handler<CreateAuthor> for DbActor {
    type Result = QueryResult<Author>;

    fn handle(&mut self, msg: CreateAuthor, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        
        let new_author = NewAuthor {
            firstname: msg.firstname.unwrap_or("".to_string()),
            lastname: msg.lastname.unwrap_or("".to_string()),
            email: msg.email.unwrap_or("".to_string())
        };
        
        diesel::insert_into(authors)
            .values(new_author)
            .returning((id, firstname, lastname, email))
            .get_result::<Author>(&mut conn)
    }
}

impl Handler<UpdateAuthor> for DbActor {
    type Result = QueryResult<Author>;

    fn handle(&mut self, msg: UpdateAuthor, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        let user = authors.find(msg.id).get_result::<Author>(&mut conn).expect("Failed to find user");

        diesel::update(authors.find(msg.id))
            .set((
                firstname.eq(msg.firstname.unwrap_or(user.firstname)),
                lastname.eq(msg.lastname.unwrap_or(user.lastname)),
                email.eq(msg.email.unwrap_or(user.email))
            ))
            .returning((id, firstname, lastname, email))
            .get_result::<Author>(&mut conn)
    }   
}

impl Handler<DeleteAuthor> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeleteAuthor, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(authors.find(msg.id))
        .execute(&mut conn)
    }
}