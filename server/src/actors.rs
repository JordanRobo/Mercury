use crate::db_models::User;
use crate::db_utils::DbActor;
use crate::schema::users::dsl::*;
use crate::messages::{ FetchUser, CreateUser, UpdateUser, DeleteUser };
use crate::insertables::{ NewUser, UserUpdated };
use crate::services::update_user;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        
        let new_user = NewUser {
            firstname: msg.firstname,
            lastname: msg.lastname,
            email: msg.email
        };
        
        diesel::insert_into(users)
            .values(new_user)
            .returning((id, firstname, lastname, email))
            .get_result::<User>(&mut conn)
    }
}

impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        
        let update_user = UserUpdated {
            firstname: msg.firstname,
            lastname: msg.lastname,
            email: msg.email
        };
        
        diesel::update(users.find(id))
            .set([update_user])
            .returning((id, firstname, lastname, email))
            .get_result::<User>(&mut conn)
    }   
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeleteUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(users.find(msg.id)).execute(&mut conn)
    }
}