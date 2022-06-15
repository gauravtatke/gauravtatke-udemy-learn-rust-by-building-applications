use super::method::{HTTPMethod, MethodError};
use std::{convert::TryFrom, error::Error, fmt::Display, str, str::Utf8Error};
use super::{QueryString, QueryStringVal};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: HTTPMethod,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &HTTPMethod {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abs&sort=1 HTTP/1.1\r\n...HEADERS...
    // only parse above type of one line. No headers
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }
        // parse the method string into an enum
        let method: HTTPMethod = method.parse()?;
        
        // parse the path to get the path and query string separated by "?"
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // first space separated word and remaining of the string as tuple
    for (i, ch) in request.chars().enumerate() {
        if ch == ' ' || ch == '\r'{
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
    
}
#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request"
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    } 
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {

}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message())
        
    }
}