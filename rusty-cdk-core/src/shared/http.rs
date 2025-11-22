pub enum HttpMethod {
    Any,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Delete
}

impl From<HttpMethod> for String {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Any => "*".to_string(),
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Head => "HEAD".to_string(),
            HttpMethod::Options => "OPTIONS".to_string(),
            HttpMethod::Patch => "PATCH".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string()
        }
    }
}

pub enum Protocol {
    Http,
    Https,
}

impl From<Protocol> for String {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::Http => "http".to_string(),
            Protocol::Https => "https".to_string(),
        }
    }
}
