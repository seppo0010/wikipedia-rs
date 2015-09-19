const LANGUAGE_URL_MARKER:&'static str = "{language}";

pub struct Wikipedia {
    pub pre_language_url:String,
    pub post_language_url:String,
    pub user_agent:String,
    pub language:String,
}

impl Default for Wikipedia {
    fn default() -> Self {
        Wikipedia {
            pre_language_url: "http://".to_owned(),
            post_language_url: ".wikipedia.org/w/api.php".to_owned(),
            user_agent: "wikipedia (https://github.com/seppo0010/wikipedia-rs)".to_owned(),
            language: "en".to_owned(),
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
}

#[test]
fn base_url() {
    let mut wikipedia = Wikipedia::default();
    assert_eq!(wikipedia.base_url(), "http://en.wikipedia.org/w/api.php");
    wikipedia.language = "es".to_owned();
    assert_eq!(wikipedia.base_url(), "http://es.wikipedia.org/w/api.php");

    wikipedia.set_base_url("http://hello.{language}.world/");
    assert_eq!(wikipedia.base_url(), "http://hello.es.world/");

    wikipedia.set_base_url("http://hello.world/");
    assert_eq!(wikipedia.base_url(), "http://hello.world/");
}
