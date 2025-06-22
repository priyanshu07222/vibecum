use poem::{handler, web::Json};

use crate::request_input::SignupInput;



#[handler]
pub fn sign_up(Json(data): Json<SignupInput>){
    println!("{}", "hello from signup")
}