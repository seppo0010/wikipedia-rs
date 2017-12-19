#[derive(Debug)]
pub struct Error;

pub trait HttpClient {
    fn user_agent(&mut self, user_agent: String);
    fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
        where I: Iterator<Item=(&'a str, &'a str)>;
}

#[cfg(feature="http-client")]
pub mod default {
    use std::convert;
    use std::io;
    use std::io::Read;

    use reqwest;
    use url;

    use super::{Error, HttpClient};

    pub struct Client {
        user_agent: String,
    }

    impl Default for Client {
        fn default() -> Self {
            Client { user_agent: "".to_owned() }
        }
    }

    impl HttpClient for Client {
        fn user_agent(&mut self, user_agent: String) {
            self.user_agent = user_agent;
        }

        fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
                where I: Iterator<Item=(&'a str, &'a str)> {
            let url = reqwest::Url::parse_with_params(base_url, args)?;
            let client = reqwest::Client::new();
            let mut response = client.get(url)
                .header(reqwest::header::UserAgent::new(self.user_agent.clone()))
                .send()?;

            if !response.status().is_success() {
                return Err(Error);
            }

            let mut response_str = String::new();
            response.read_to_string(&mut response_str)?;
            Ok(response_str)
        }
    }

    impl convert::From<reqwest::Error> for Error {
        fn from(_: reqwest::Error) -> Self {
            Error
        }
    }

    impl convert::From<url::ParseError> for Error {
        fn from(_: url::ParseError) -> Self {
            Error
        }
    }

    impl convert::From<io::Error> for Error {
        fn from(_: io::Error) -> Self {
            Error
        }
    }
}
