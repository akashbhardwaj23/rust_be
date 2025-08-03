use poem::{
    listener::TcpListener, post, Route, Server
};

use crate::{routes::user::{sign_in, sign_up}};


pub mod types;
pub mod routes;
pub mod middlleware;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
                    .at("/user/signin", post(sign_in))
                    .at("/user/signup", post(sign_up));


    Server::new(TcpListener::bind("0.0.0.0:3000")).name("rust_server").run(app).await
}
