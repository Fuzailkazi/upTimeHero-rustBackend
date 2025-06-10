use poem::{
    get, post, handler, listener::TcpListener, middleware::Tracing, web::{Json,Path}, EndpointExt, Route, Server,
};

pub mod request_inputs;
pub mod request_outputs;

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::CreateWebsiteOutput;

#[handler]
fn get_website(Path(name): Path<String>) -> String {
    format!("hello: {name}")
}

#[handler] 
fn create_website(Json(data) : Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let website = data.url;

    let response = CreateWebsiteOutput{
        id: String::from("ID")
    };

    Json(response)
}

#[tokio::main]

async fn main() -> Result<(), std::io::Error> {

    let app = Route::new().at("/website/:website_id", get(get_website)).
    at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}