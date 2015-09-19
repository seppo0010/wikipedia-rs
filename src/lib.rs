extern crate hyper;
extern crate url;
extern crate serde_json;

use std::result;
use std::io;
use std::io::Read;

use hyper::{Client, Url};
use hyper::header::UserAgent;

const LANGUAGE_URL_MARKER:&'static str = "{language}";

#[derive(Debug)]
pub enum Error {
    UrlError(url::ParseError),
    HyperError(hyper::error::Error),
    HTTPError(hyper::client::response::Response),
    IOError(io::Error),
    JSONError(serde_json::error::Error),
    JSONPathError,
    InvalidParameter(String),
}

impl std::convert::From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlError(e)
    }
}

impl std::convert::From<hyper::error::Error> for Error {
    fn from(e: hyper::error::Error) -> Self {
        Error::HyperError(e)
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(e)
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::JSONError(e)
    }
}

pub type Result<T> = result::Result<T, Error>;

pub struct Wikipedia {
    pub pre_language_url:String,
    pub post_language_url:String,
    pub user_agent:String,
    pub language:String,
    pub search_results:u32,
}

impl Default for Wikipedia {
    fn default() -> Self {
        Wikipedia {
            pre_language_url: "https://".to_owned(),
            post_language_url: ".wikipedia.org/w/api.php".to_owned(),
            user_agent: "wikipedia (https://github.com/seppo0010/wikipedia-rs)".to_owned(),
            language: "en".to_owned(),
            search_results: 10,
        }
    }
}

impl Wikipedia {
    pub fn base_url(&self) -> String {
        format!("{}{}{}", self.pre_language_url, self.language, self.post_language_url)
    }

    pub fn set_base_url(&mut self, base_url: &str) {
        let index = match base_url.find(LANGUAGE_URL_MARKER) {
            Some(i) => i,
            None => {
                self.pre_language_url = base_url.to_owned();
                self.language = "".to_owned();
                self.post_language_url = "".to_owned();
                return;
            }
        };
        self.pre_language_url = base_url[0..index].to_owned();
        self.post_language_url = base_url[index+LANGUAGE_URL_MARKER.len()..].to_owned();
    }

    fn query(&self, url: Url) -> Result<serde_json::Value> {
        let client = Client::new();
        let mut response = try!(client.get(url)
            .header(UserAgent(self.user_agent.clone()))
            .send());

        if !response.status.is_success() {
            return Err(Error::HTTPError(response));
        }

        let mut response_str = String::new();
        try!(response.read_to_string(&mut response_str));

        Ok(try!(serde_json::from_str(&*response_str)))
    }

    fn search_url(&self, query: &str) -> Result<Url> {
        let mut url = try!(Url::parse(&*self.base_url()));
        let results = &*format!("{}", self.search_results);
        url.set_query_from_pairs(vec![
                ("list", "search"),
                ("srprop", ""),
                ("srlimit", results),
                ("srsearch", query),
                ("continue", ""),
                ("format", "json"),
                ("action", "query"),
                ].into_iter());
        Ok(url)
    }

    pub fn search(&self, query: &str) -> Result<Vec<String>> {
        let url = try!(self.search_url(query));
        let data = try!(self.query(url));

        // There has to be a better way to write the following code
        Ok(try!(
            data.as_object()
            .and_then(|x| x.get("query"))
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("search"))
            .and_then(|x| x.as_array())
            .ok_or(Error::JSONPathError)
            ).into_iter().filter_map(|i|
                i.as_object()
                .and_then(|i| i.get("title"))
                .and_then(|s| s.as_string().map(|s| s.to_owned()))
                ).collect())
    }

    fn geosearch_url(&self, latitude: f64, longitude: f64, radius: u16) -> Result<Url> {
        if latitude < -90.0 || latitude > 90.0 {
            return Err(Error::InvalidParameter("latitude".to_string()))
        }
        if longitude < -180.0 || longitude > 180.0 {
            return Err(Error::InvalidParameter("longitude".to_string()))
        }
        if radius < 10 || radius > 10000 {
            return Err(Error::InvalidParameter("radius".to_string()))
        }
        let mut url = try!(Url::parse(&*self.base_url()));
        let results = &*format!("{}", self.search_results);
        url.set_query_from_pairs(vec![
                ("list", "geosearch"),
                ("gsradius", &*format!("{}", radius)),
                ("gscoord", &*format!("{}|{}", latitude, longitude)),
                ("gslimit", results),
                ("format", "json"),
                ("action", "query"),
                ].into_iter());
        Ok(url)
    }

    pub fn geosearch(&self, latitude: f64, longitude: f64, radius: u16) -> Result<Vec<String>> {
        let url = try!(self.geosearch_url(latitude, longitude, radius));
        let data = try!(self.query(url));

        // There has to be a better way to write the following code
        Ok(try!(
            data.as_object()
            .and_then(|x| x.get("query"))
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("geosearch"))
            .and_then(|x| x.as_array())
            .ok_or(Error::JSONPathError)
            ).into_iter().filter_map(|i|
                i.as_object()
                .and_then(|i| i.get("title"))
                .and_then(|s| s.as_string().map(|s| s.to_owned()))
                ).collect())
    }
}

#[test]
fn base_url() {
    let mut wikipedia = Wikipedia::default();
    assert_eq!(wikipedia.base_url(), "https://en.wikipedia.org/w/api.php");
    wikipedia.language = "es".to_owned();
    assert_eq!(wikipedia.base_url(), "https://es.wikipedia.org/w/api.php");

    wikipedia.set_base_url("https://hello.{language}.world/");
    assert_eq!(wikipedia.base_url(), "https://hello.es.world/");

    wikipedia.set_base_url("https://hello.world/");
    assert_eq!(wikipedia.base_url(), "https://hello.world/");
}

#[test]
fn search_url() {
    let wikipedia = Wikipedia::default();
    assert_eq!(&*format!("{}", wikipedia.search_url("hello world").unwrap()),
            "https://en.wikipedia.org/w/api.php?list=search&srprop=&srlimit=10&srsearch=hello+world&continue=&format=json&action=query");
}

#[test]
fn search() {
    let wikipedia = Wikipedia::default();
    let results = wikipedia.search("hello world").unwrap();
    assert!(results.len() > 0);
    assert!(results.contains(&"\"Hello, World!\" program".to_owned()));
}

#[test]
fn geosearch_url() {
    let wikipedia = Wikipedia::default();
    assert_eq!(&*format!("{}", wikipedia.geosearch_url(-34.603333, -58.381667, 10).unwrap()),
            "https://en.wikipedia.org/w/api.php?list=geosearch&gsradius=10&gscoord=-34.603333%7C-58.381667&gslimit=10&format=json&action=query");
}

#[test]
fn geosearch() {
    let wikipedia = Wikipedia::default();
    let results = wikipedia.geosearch(-34.603333, -58.381667, 10).unwrap();
    assert!(results.len() > 0);
    assert!(results.contains(&"Buenos Aires".to_owned()));
}
