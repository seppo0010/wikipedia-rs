pub use crate::Error;

pub trait HttpClient {
    /// Set the user agent. Default user agent is empty string.
    fn user_agent(&mut self, user_agent: String);

    /// Set a Wikimedia Personal API authentication token.
    fn bearer_token(&mut self, bearer_token: String);

    /// Run an http request with the given url and args, returning
    /// the result as a string.
    fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
    where
        I: Iterator<Item = (&'a str, &'a str)>;
}

#[cfg(feature = "http-client")]
pub mod default {
    use reqwest;
    use std::io::Read;

    use super::{Error, HttpClient};

    pub struct Client {
        user_agent: String,
        bearer_token: Option<String>,
    }

    impl Default for Client {
        fn default() -> Self {
            Client {
                user_agent: "wikipedia (https://github.com/seppo0010/wikipedia-rs)".to_owned(),
                bearer_token: None,
            }
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Error {
            Error::HTTPError(Box::new(e))
        }
    }

    impl HttpClient for Client {
        fn user_agent(&mut self, user_agent: String) {
            self.user_agent = user_agent;
        }

        fn bearer_token(&mut self, bearer_token: String) {
            self.bearer_token = Some(bearer_token);
        }

        fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
        where
            I: Iterator<Item = (&'a str, &'a str)>,
        {
            let url =
                reqwest::Url::parse_with_params(base_url, args).map_err(|_| Error::URLError)?;
            let mut request = reqwest::blocking::Client::new()
                .get(url)
                .header(reqwest::header::USER_AGENT, self.user_agent.clone());
            if let Some(ref bearer_token) = self.bearer_token {
                request = request.header(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", bearer_token),
                );
            }
            let mut response = request.send()?.error_for_status()?;

            let mut response_str = String::new();
            response.read_to_string(&mut response_str)?;
            Ok(response_str)
        }
    }
}
