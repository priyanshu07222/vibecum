use poem::{listener::TcpListener, Route, Server};


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let _ = db::say_hello();
    let app = Route::new();
    
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
