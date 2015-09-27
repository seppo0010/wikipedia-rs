#[cfg(feature="http-client")] extern crate hyper;
extern crate wikipedia;

#[cfg(feature = "http-client")]
mod tests {
    use wikipedia::Wikipedia;
    use wikipedia::http;
    use std::collections::HashSet;

    fn w() -> Wikipedia<http::hyper::Client> {
        Wikipedia::default()
    }

    #[test]
    fn search() {
        let wikipedia = w();
        let results = wikipedia.search("hello world").unwrap();
        assert!(results.len() > 0);
        assert!(results.contains(&"\"Hello, World!\" program".to_owned()));
    }

    #[test]
    fn geosearch() {
        let wikipedia = w();
        let results = wikipedia.geosearch(-34.603333, -58.381667, 10).unwrap();
        assert!(results.len() > 0);
        assert!(results.contains(&"Buenos Aires".to_owned()));
    }

    #[test]
    fn random() {
        let wikipedia = w();
        wikipedia.random().unwrap().unwrap();
    }

    #[test]
    fn random_count() {
        let wikipedia = w();
        assert_eq!(wikipedia.random_count(3).unwrap().len(), 3);
    }

    #[test]
    fn page_content() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Parkinson's law of triviality".to_owned());
        assert!(page.get_content().unwrap().contains("bikeshedding"));
    }

    #[test]
    fn title() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Parkinson's law of triviality".to_owned());
        assert_eq!(page.get_title().unwrap(), "Parkinson's law of triviality".to_owned());
        let page = wikipedia.page_from_pageid("4138548".to_owned());
        assert_eq!(page.get_title().unwrap(), "Parkinson's law of triviality".to_owned());
    }

    #[test]
    fn pageid() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Parkinson's law of triviality".to_owned());
        assert_eq!(page.get_pageid().unwrap(), "4138548".to_owned());
        let page = wikipedia.page_from_title("Bikeshedding".to_owned());
        assert_eq!(page.get_pageid().unwrap(), "4138548".to_owned());
        let page = wikipedia.page_from_pageid("4138548".to_owned());
        assert_eq!(page.get_pageid().unwrap(), "4138548".to_owned());
    }

    #[test]
    fn page_html_content() {
        let wikipedia = w();
        let page = wikipedia.page_from_pageid("4138548".to_owned());
        let html = page.get_html_content().unwrap();
        assert!(html.contains("bikeshedding"));
        assert!(html.contains("</div>")); // it would not be html otherwise
    }

    #[test]
    fn page_summary() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Parkinson's law of triviality".to_owned());
        let summary = page.get_summary().unwrap();
        let content = page.get_content().unwrap();
        assert!(summary.contains("bikeshedding"));
        assert!(summary.len() < content.len());
    }

    #[test]
    fn page_redirect_summary() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Bikeshedding".to_owned());
        let summary = page.get_summary().unwrap();
        let content = page.get_content().unwrap();
        assert!(summary.contains("bikeshedding"));
        assert!(summary.len() < content.len());
    }

    #[test]
    fn page_images() {
        let mut wikipedia = w();
        wikipedia.images_results = "5".to_owned();
        let page = wikipedia.page_from_title("Argentina".to_owned());
        let images = page.get_images().unwrap();
        let mut c = 0;
        let mut set = HashSet::new();
        for i in images {
            assert!(i.title.len() > 0);
            assert!(i.url.len() > 0);
            assert!(i.description_url.len() > 0);
            c += 1;
            set.insert(i.title);
            if c == 11 {
                break;
            }
        }
        assert_eq!(set.len(), 11);
    }

    #[test]
    fn coordinates() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("San Francisco".to_owned());
        let (lat, lon) = page.get_coordinates().unwrap().unwrap();
        assert!(lat > 0.0);
        assert!(lon < 0.0);
    }

    #[test]
    fn no_coordinates() {
        let wikipedia = w();
        let page = wikipedia.page_from_title("Bikeshedding".to_owned());
        assert!(page.get_coordinates().unwrap().is_none());
    }

    #[test]
    fn references() {
        let mut wikipedia = w();
        wikipedia.links_results = "3".to_owned();
        let page = wikipedia.page_from_title("Argentina".to_owned());
        let references = page.get_references().unwrap();
        let mut c = 0;
        let mut set = HashSet::new();
        for r in references {
            assert!(r.url.starts_with("http"));
            c += 1;
            set.insert(r.url);
            if c == 7 {
                break;
            }
        }
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn links() {
        let mut wikipedia = w();
        wikipedia.links_results = "3".to_owned();
        let page = wikipedia.page_from_title("Argentina".to_owned());
        let links = page.get_links().unwrap();
        let mut c = 0;
        let mut set = HashSet::new();
        for r in links {
            c += 1;
            set.insert(r.title);
            if c == 7 {
                break;
            }
        }
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn categories() {
        let mut wikipedia = w();
        wikipedia.categories_results = "3".to_owned();
        let page = wikipedia.page_from_title("Argentina".to_owned());
        let categories = page.get_links().unwrap();
        let mut c = 0;
        let mut set = HashSet::new();
        for ca in categories {
            c += 1;
            set.insert(ca.title);
            if c == 7 {
                break;
            }
        }
        assert_eq!(set.len(), 7);
    }
}
