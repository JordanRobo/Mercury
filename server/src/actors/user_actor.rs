use crate::db::{ User, NewUser };
use crate::db::DbActor;
use crate::db::users::dsl::*;
use crate::messages::{ FetchUsers, FetchUser, CreateUser, UpdateUser, DeleteUser };
use actix::Handler;
use diesel::{ self, prelude::* };

impl Handler<FetchUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

        users.get_results::<User>(&mut conn)
    }
}

impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");

        users.find(msg.id).get_result::<User>(&mut conn)
    }
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        
        let new_user = NewUser {
            firstname: msg.firstname.unwrap_or("".to_string()),
            lastname: msg.lastname.unwrap_or("".to_string()),
            email: msg.email.unwrap_or("".to_string())
        };
        
        diesel::insert_into(users)
            .values(new_user)
            .returning((id, firstname, lastname, email))
            .get_result::<User>(&mut conn)
    }
}

impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        let user = users.find(msg.id).get_result::<User>(&mut conn).expect("Failed to find user");

        diesel::update(users.find(msg.id))
            .set((
                firstname.eq(msg.firstname.unwrap_or(user.firstname)),
                lastname.eq(msg.lastname.unwrap_or(user.lastname)),
                email.eq(msg.email.unwrap_or(user.email))
            ))
            .returning((id, firstname, lastname, email))
            .get_result::<User>(&mut conn)
    }   
}

impl Handler<DeleteUser> for DbActor {
    type Result = QueryResult<usize>;

    fn handle(&mut self, msg: DeleteUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Failed to get connection");
        diesel::delete(users.find(msg.id))
        .execute(&mut conn)
    }
}