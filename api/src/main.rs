use poem::{
    get, post, handler, listener::TcpListener, web::{Json,Path}, Route, Server,
};

pub mod request_inputs;
pub mod request_outputs;

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateWebsiteOutput;
use store::Store;
#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler] 
fn create_website(Json(data) : Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {

    let s = Store();
    let id = s.create_website();
    let response = CreateWebsiteOutput{
        id
    };

    Json(response)
}

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {

    let app = Route::new()
    .at("/website/:website_id", get(get_website))
    .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}