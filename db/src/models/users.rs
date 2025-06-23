use diesel::{prelude::*};
use uuid::Uuid;

use crate::connection::Connect;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub password: String,
    pub name: Option<String>
}

impl Connect{
    pub fn sign_up(&mut self, input_user_name:String, input_password: String, input_name: Option<String>) -> Result<String, diesel::result::Error>{
        if input_user_name.is_empty() || input_password.is_empty() {
            panic!("Both user_name and password is required");
        }
        
        let id = Uuid::new_v4();
        let u = User{
            id: id.to_string(),
            user_name: input_user_name,
            password: input_password,
            name: input_name
        };
        
        diesel::insert_into(crate::schema::users::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)?;
        
        
        Ok(String::from(id))
    }
    
    pub fn sign_in(&mut self, input_user_name:String, input_password: String) -> Result<bool, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        
        let user_result = users
            .filter(user_name.eq(input_user_name))
            .select(User::as_select())
            .first(&mut self.conn)?;
        
        if user_result.password != input_password {
            return Ok(false);
        }
        
        Ok(true)
    }
}