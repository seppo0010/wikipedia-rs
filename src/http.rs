#[derive(Debug)]
pub struct Error;

pub trait HttpClient: Default {
    fn user_agent(&mut self, user_agent: String);
    fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
        where I: Iterator<Item=(&'a str, &'a str)>;
}

#[cfg(feature="http-client")]
pub mod hyper {
    use std::convert;
    use std::io;
    use std::io::Read;

    use hyper;
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
            let mut url = try!(hyper::Url::parse(base_url));
            url.set_query_from_pairs(args);
            let client = hyper::Client::new();
            let mut response = try!(client.get(url)
                .header(hyper::header::UserAgent(self.user_agent.clone()))
                .send());

            if !response.status.is_success() {
                return Err(Error);
            }

            let mut response_str = String::new();
            try!(response.read_to_string(&mut response_str));
            Ok(response_str)
        }
    }

    impl convert::From<hyper::error::Error> for Error {
        fn from(_: hyper::error::Error) -> Self {
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
