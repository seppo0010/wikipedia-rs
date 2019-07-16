use crate::Error;
pub use failure::Error as FError;
use futures::{future, Future, Stream};
use reqwest::{
    header::{self, HeaderValue},
    r#async::{Client as RClient, Decoder},
};
use std::{
    io::{Cursor, Read},
    mem,
};

pub mod wikipedia;

pub trait HttpClient {
    fn user_agent(&mut self, user_agent: String);
    fn get<'a, I, S>(
        &self,
        base_url: &str,
        args: I,
    ) -> Box<dyn Future<Item = String, Error = Error> + 'static>
    where
        I: IntoIterator<Item = (&'a str, S)>,
        S: AsRef<str>;
}

pub struct Client {
    user_agent: String,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            user_agent: "".into(),
        }
    }
}

impl HttpClient for Client {
    fn user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    fn get<'a, I, S>(
        &self,
        base_url: &str,
        args: I,
    ) -> Box<dyn Future<Item = String, Error = Error> + 'static>
    where
        I: IntoIterator<Item = (&'a str, S)>,
        S: AsRef<str>,
    {
        // let url = reqwest::Url::parse_with_params(base_url, args).unwrap();
        // let req = RClient::new().get(url);
        // let req = match HeaderValue::from_str(&self.user_agent) {
        //     Ok(header) => req.header(header::USER_AGENT, header),
        //     Err(_) => req,
        // };
        let header = HeaderValue::from_str(&self.user_agent);
        Box::new(
            future::result(reqwest::Url::parse_with_params(base_url, args))
                .from_err::<Error>()
                .map(|url| RClient::new().get(url))
                .map(|req| match header {
                    Ok(header) => req.header(header::USER_AGENT, header),
                    Err(_) => req,
                })
                .and_then(|req| req.send().map_err(|_| Error::HTTPError))
                // .map_err(|e| e.into::<Error>())
                // req.send()
                // .from_err::<Error>()
                // .and_then(|res| {
                //     if res.status().is_success() {
                //         Ok(res)
                //     } else {
                //         Err(Error::BadStatus)
                //     }
                // })
                // .map_err(|_| Error::HTTPError)
                .and_then(|mut res| {
                    // ensure!(res.status().is_success(), Error::BadStatus);
                    let body = mem::replace(res.body_mut(), Decoder::empty());
                    body.concat2().from_err::<Error>()
                })
                .and_then(|body| {
                    let mut body = Cursor::new(body);
                    let mut buffer = String::new();
                    body.read_to_string(&mut buffer)?;
                    Ok(buffer)
                }),
        )
        // future::result(reqwest::Url::parse_with_params(base_url, args))
        //     .map(|url| self.inner.get(url))
        //     .from_err::<FError>()
        //     .map(|req| match HeaderValue::from_str(&self.user_agent) {
        //         Ok(header) => req.header(header::USER_AGENT, header),
        //         Err(_) => req,
        //     })
        //     .and_then(|req| req.send().from_err())
        //     .and_then(|mut res| {
        //         ensure!(res.status().is_success(), err_msg("Bad status"));
        //         let body = mem::replace(res.body_mut(), Decoder::empty());
        //         Ok(body.concat2())
        //     })
        //     .and_then(|body| {
        //         let mut body = Cursor::new(body);
        //         let mut buffer = String::new();
        //         body.read_to_string(&mut buffer)?;
        //         Ok(buffer)
        //     });
        // unimplemented!()
        //
        // let mut response = client
        //     .get(url)
        //     // .header(reqwest::header::USER_AGENT, self.user_agent.clone())
        //     .send()?;

        // self.inner.get(url)
        // ensure!(response.status().is_success(), err_msg("Bad status"));

        // let mut response_str = String::new();
        // response.read_to_string(&mut response_str)?;
        // Ok(response_str)
    }
}
