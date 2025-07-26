use crate::{store::Store};
use diesel::{insert_into, prelude::*, sql_types::Uuid};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub createdat: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String, email: String) -> Result<User, diesel::result::Error> {
        let new_user = NewUser {
            username: &username,
            password: &password,
            email: &email,
        };
        let user: User = insert_into(crate::schema::users::table)
            .values(&new_user)
            .get_result(&mut self.conn)?;
        Ok(user)
    }

    pub fn sign_in(&mut self, input_username: String, input_password: String) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let user_result: User = users
            .filter(username.eq(&input_username))
            .first(&mut self.conn)?;
            
        if user_result.password != input_password {
            return Ok(false);
        }
        Ok(true)
    }
}