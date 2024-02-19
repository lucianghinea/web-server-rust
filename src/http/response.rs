use std::collections::HashMap;

pub struct StatusCode {
    pub code: usize,
    pub msg: &'static str
}

impl StatusCode {
    pub fn ok() -> Self {
        StatusCode {
            code: 200,
            msg: "OK"
        }
    }
}

pub struct Response<'a> {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: &'a str
}