# wikipedia-rs

[![Build Status](https://travis-ci.org/seppo0010/wikipedia-rs.svg?branch=master)](https://travis-ci.org/seppo0010/wikipedia-rs)
[![crates.io](http://meritbadge.herokuapp.com/wikipedia)](https://crates.io/crates/wikipedia)


Access wikipedia articles from Rust.

The crate is called `wikipedia` and you can depend on it via cargo:

```toml
[dependencies]
wikipedia = "0.1.0"
```


# Examples

```rust
extern crate wikipedia;

let wiki = wikipedia::Wikipedia::<wikipedia::http::hyper::Client>::default();
let page = wiki.page_from_title("Club Atletico River Plate".to_owned());
let content = page.get_content().unwrap();
assert!(content.contains("B Nacional"));
```

# Documentation

https://seppo0010.github.io/wikipedia-rs/
