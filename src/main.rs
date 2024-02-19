use tokio::net::TcpListener;
use std::{collections::HashMap, io};

mod http;
use http::*;

async fn process_socket(socket: tokio::net::TcpStream) -> Result<(), Error> {
    let mut connection = Connection::new(socket).await?;
    println!("method {:?}\nuri:{:?}\nversion:{:?}\nheaders:{:?}\n", 
    connection.request.method, connection.request.uri, connection.request.version, connection.request.headers);

    let mut response_headers: HashMap<String, String> = HashMap::new();
    response_headers.insert("header1key".to_string(), "header1value".to_string());

    connection.respond(Response { 
        status: StatusCode::ok(),
        headers: response_headers, 
        body: &String::from("This is the response body")
    }).await?;
    Ok(())
}

#[tokio::main]
 async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}