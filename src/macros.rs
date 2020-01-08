#[macro_export]
macro_rules! results {
    ($data: expr, $query_field: expr) => {
        // There has to be a better way to write the following code
        try!($data
            .as_object()
            .and_then(|x| x.get("query"))
            .and_then(|x| x.as_object())
            .and_then(|x| x.get($query_field))
            .and_then(|x| x.as_array())
            .ok_or(Error::JSONPathError))
        .into_iter()
        .filter_map(|i| {
            i.as_object()
                .and_then(|i| i.get("title"))
                .and_then(|s| s.as_str().map(|s| s.to_owned()))
        })
        .collect()
    };
}

#[macro_export]
macro_rules! cont {
    ($this: expr, $cont: expr, $($params: expr),*) => {{
        let qp = $this.identifier.query_param();
        let mut params = vec![
            $($params),*,
            ("format", "json"),
            ("action", "query"),
            (&*qp.0, &*qp.1),
        ];
        match *$cont {
            Some(ref v) => {
                for x in v.iter() { params.push((&*x.0, &*x.1)); }
            },
            None => params.push(("continue", "")),
        }
        let q = try!($this.wikipedia.query(params.into_iter()));

        let pages = try!(q
            .as_object()
            .and_then(|x| x.get("query"))
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("pages"))
            .and_then(|x| x.as_object())
            .ok_or(Error::JSONPathError));

        Ok((pages.values().cloned().collect(), try!($this.parse_cont(&q))))
    }}
}
