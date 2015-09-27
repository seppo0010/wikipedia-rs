use std::vec::IntoIter;
use std::marker::PhantomData;

use serde_json::Value;

use super::{Page, Result, http};

pub struct Iter<'a, A: 'a + http::HttpClient, B: IterItem> {
    page: &'a Page<'a, A>,
    inner: IntoIter<Value>,
    cont: Option<Vec<(String, String)>>,
    phantom: PhantomData<B>
}

impl<'a, A: http::HttpClient, B: IterItem> Iter<'a, A, B> {
    pub fn new(page: &'a Page<A>) -> Result<Iter<'a, A, B>> {
        let (array, cont) = try!(B::request_next(page, &None));
        Ok(Iter {
            page: page,
            inner: array.into_iter(),
            cont: cont,
            phantom: PhantomData,
        })
    }

    fn fetch_next(&mut self) -> Result <()> {
        if self.cont.is_some() {
            let (array, cont) = try!(B::request_next(self.page, &self.cont));
            self.inner = array.into_iter();
            self.cont = cont;
        }
        Ok(())
    }
}

impl<'a, A: http::HttpClient, B: IterItem> Iterator for Iter<'a, A, B> {
    type Item = B;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(ref v) => B::from_value(&v),
            None => match self.cont {
                Some(_) => match self.fetch_next() {
                    Ok(_) => self.inner.next().and_then(|x| B::from_value(&x)),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }
}

pub trait IterItem: Sized {
    fn request_next<A: http::HttpClient>(page: &Page<A>, cont: &Option<Vec<(String, String)>>)
            -> Result<(Vec<Value>, Option<Vec<(String, String)>>)>;
    fn from_value(value: &Value) -> Option<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub description_url: String,
}

impl IterItem for Image {
    fn request_next<A: http::HttpClient>(page: &Page<A>, cont: &Option<Vec<(String, String)>>)
            -> Result<(Vec<Value>, Option<Vec<(String, String)>>)> {
        page.request_images(&cont)
    }

    fn from_value(value: &Value) -> Option<Image> {
        let obj = match value.as_object() {
            Some(o) => o,
            None => return None,
        };

        let title = obj
            .get("title")
            .and_then(|x| x.as_string())
            .unwrap_or("").to_owned();
        let url = obj
            .get("imageinfo")
            .and_then(|x| x.as_array())
            .and_then(|x| x.into_iter().next())
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("url"))
            .and_then(|x| x.as_string())
            .unwrap_or("").to_owned();
        let description_url = obj
            .get("imageinfo")
            .and_then(|x| x.as_array())
            .and_then(|x| x.into_iter().next())
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("descriptionurl"))
            .and_then(|x| x.as_string())
            .unwrap_or("").to_owned();

        Some(Image {
            url: url.to_owned(),
            title: title.to_owned(),
            description_url: description_url.to_owned(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Reference {
    pub url: String,
}

impl IterItem for Reference {
    fn request_next<A: http::HttpClient>(page: &Page<A>, cont: &Option<Vec<(String, String)>>)
            -> Result<(Vec<Value>, Option<Vec<(String, String)>>)> {
        page.request_extlinks(&cont)
    }

    fn from_value(value: &Value) -> Option<Reference> {
        value
            .as_object()
            .and_then(|x| x.get("*"))
            .and_then(|x| x.as_string())
            .map(|s| Reference {
                url: if s.starts_with("http:") {
                    s.to_owned()
                } else {
                    format!("http:{}", s)
                },
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct Link {
    pub title: String,
}

impl IterItem for Link {
    fn request_next<A: http::HttpClient>(page: &Page<A>, cont: &Option<Vec<(String, String)>>)
            -> Result<(Vec<Value>, Option<Vec<(String, String)>>)> {
        page.request_links(&cont)
    }

    fn from_value(value: &Value) -> Option<Link> {
        value
            .as_object()
            .and_then(|x| x.get("title"))
            .and_then(|x| x.as_string())
            .map(|s| Link { title: s.to_owned() })
    }
}
