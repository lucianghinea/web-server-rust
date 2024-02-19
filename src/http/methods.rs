#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE
}

impl  From<&str> for Method {
    fn from(s: &str) -> Self {
        if s == "GET" {
            Method::GET
        } else if s == "POST" {
            Method::POST
        } else {
            Method::GET
        }
    }
}