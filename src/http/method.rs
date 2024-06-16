use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATH" => Ok(Self::PATH),
            _ => Err(MethodError)
        }

    }
}

pub struct MethodError;