pub use crate::Error;

pub trait HttpClient {
    fn user_agent(&mut self, user_agent: String);
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
    }

    impl Default for Client {
        fn default() -> Self {
            Client {
                user_agent: "".to_owned(),
            }
        }
    }

    impl HttpClient for Client {
        fn user_agent(&mut self, user_agent: String) {
            self.user_agent = user_agent;
        }

        fn get<'a, I>(&self, base_url: &str, args: I) -> Result<String, Error>
        where
            I: Iterator<Item = (&'a str, &'a str)>,
        {
            let url = reqwest::Url::parse_with_params(base_url, args)
                .map_err(|_| Error::URLError)?;
            let client = reqwest::blocking::Client::new();
            let mut response = client
                .get(url)
                .header(reqwest::header::USER_AGENT, self.user_agent.clone())
                .send()
                .map_err(|_| Error::HTTPError)?;

            if !response.status().is_success() {
                return Err(Error::HTTPError);
            }

            let mut response_str = String::new();
            response.read_to_string(&mut response_str)?;
            Ok(response_str)
        }
    }
}
