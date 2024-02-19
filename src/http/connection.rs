use tokio::io::AsyncWriteExt;

use crate::{Error, Request, Response};

pub struct Connection {
    pub request: Request,
    pub socket: tokio::net::TcpStream
}

impl Connection {
    pub async fn new(mut socket: tokio::net::TcpStream) -> Result<Connection, Error> {
        let request = Request::new(&mut socket).await?;
        Ok(Connection {
            request, socket
        })
    }

    pub async fn respond<'a>(&mut self, resp: Response<'a>) -> Result<(), std::io::Error> {
        self.socket.write_all(format!("{} {} {}", self.request.version, resp.status.code, resp.status.msg).as_bytes()).await?;
        println!("version: {} status code: {} status msg: {}", self.request.version, resp.status.code, resp.status.msg);
        for (k, v) in resp.headers.iter() {
            println!("headers: key {} value {}", k, v);
            self.socket.write_all(format!("{}: {}\r\n", k, v).as_bytes()).await?;
        }

        println!("response body: {}", resp.body);
        self.socket.write_all(resp.body.as_bytes()).await?;
        Ok(())
    }
}