use poem::{listener::TcpListener, post, Route, Server};

use crate::routes::user::sign_up;

pub mod request_input;
pub mod routes;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = db::say_hello();
    let app = Route::new()
        .at("/api/sign_up", post(sign_up));
    
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
