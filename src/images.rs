use std::collections::btree_map::IntoIter;

use serde_json::Value;

use super::{Page, Result};

pub struct Iter<'a> {
    page: &'a Page<'a>,
    inner: IntoIter<String, Value>,
    cont: Option<Vec<(String, String)>>
}

impl<'a> Iter<'a> {
    pub fn new(page: &'a Page) -> Result<Iter<'a>> {
        let (array, cont) = try!(page.request_images(&None));
        Ok(Iter {
            page: page,
            inner: array.into_iter(),
            cont: cont,
        })
    }

    fn fetch_next(&mut self) -> Result <()> {
        if self.cont.is_some() {
            let (array, cont) = try!(self.page.request_images(&self.cont));
            self.inner = array.into_iter();
            self.cont = cont;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub description_url: String,
}

impl Image {
    fn new(value: &Value) -> Option<Image>{
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

impl<'a> Iterator for Iter<'a> {
    type Item = Image;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(ref v) => Image::new(&v.1),
            None => match self.cont {
                Some(_) => match self.fetch_next() {
                    Ok(_) => self.inner.next().and_then(|x| Image::new(&x.1)),
                    Err(_) => None,
                },
                None => None,
            }
        }
    }
}
