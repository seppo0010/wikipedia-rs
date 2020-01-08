use crate::WikiResponse;
use std::marker::PhantomData;
use std::vec::IntoIter;

use serde_json::Value;

use super::{http, Page, Result};

pub struct Iter<'a, A: 'a + http::HttpClient, B: IterItem> {
    page: &'a Page<'a, A>,
    inner: IntoIter<Value>,
    cont: Option<Vec<(String, String)>>,
    phantom: PhantomData<B>,
}

impl<'a, A: http::HttpClient, B: IterItem> Iter<'a, A, B> {
    pub fn new(page: &'a Page<A>) -> Result<Iter<'a, A, B>> {
        let (array, cont) = try!(B::request_next(page, &None));
        Ok(Iter {
            page,
            inner: array.into_iter(),
            cont,
            phantom: PhantomData,
        })
    }

    fn fetch_next(&mut self) -> Result<()> {
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
            },
        }
    }
}

pub trait IterItem: Sized {
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse>;
    fn from_value(value: &Value) -> Option<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub description_url: String,
}

impl IterItem for Image {
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse> {
        page.request_images(&cont)
    }

    fn from_value(value: &Value) -> Option<Image> {
        let obj = match value.as_object() {
            Some(o) => o,
            None => return None,
        };

        let title = obj
            .get("title")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_owned();
        let url = obj
            .get("imageinfo")
            .and_then(|x| x.as_array())
            .and_then(|x| x.iter().next())
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("url"))
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_owned();
        let description_url = obj
            .get("imageinfo")
            .and_then(|x| x.as_array())
            .and_then(|x| x.iter().next())
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("descriptionurl"))
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_owned();

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
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse> {
        page.request_extlinks(&cont)
    }

    fn from_value(value: &Value) -> Option<Reference> {
        value
            .as_object()
            .and_then(|x| x.get("*"))
            .and_then(|x| x.as_str())
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
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse> {
        page.request_links(&cont)
    }

    fn from_value(value: &Value) -> Option<Link> {
        value
            .as_object()
            .and_then(|x| x.get("title"))
            .and_then(|x| x.as_str())
            .map(|s| Link {
                title: s.to_owned(),
            })
    }
}

#[derive(Debug, PartialEq)]
pub struct LangLink {
    /// The language ID
    pub lang: String,

    /// The page title in this language, may be `None` if undefined
    pub title: Option<String>,
}

impl IterItem for LangLink {
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse> {
        page.request_langlinks(&cont)
    }

    fn from_value(value: &Value) -> Option<LangLink> {
        value.as_object().map(|l| LangLink {
            lang: l.get("lang").unwrap().as_str().unwrap().into(),
            title: l.get("*").and_then(|n| n.as_str()).map(|n| n.into()),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Category {
    pub title: String,
}

impl IterItem for Category {
    fn request_next<A: http::HttpClient>(
        page: &Page<A>,
        cont: &Option<Vec<(String, String)>>,
    ) -> Result<WikiResponse> {
        page.request_categories(&cont)
    }

    fn from_value(value: &Value) -> Option<Category> {
        value
            .as_object()
            .and_then(|x| x.get("title"))
            .and_then(|x| x.as_str())
            .map(|s| Category {
                title: if s.starts_with("Category: ") {
                    s[10..].to_owned()
                } else {
                    s.to_owned()
                },
            })
    }
}
