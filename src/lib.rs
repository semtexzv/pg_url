use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern(immutable, parallel_safe)]
fn url_scheme(url: &str) -> Option<String> {
    url::Url::parse(url).ok().map(|u| u.scheme().to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_scheme(url: &str, scheme: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    url.set_scheme(scheme).ok()?;
    Some(url.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_host(url: &str) -> Option<String> {
    url::Url::parse(url).ok().and_then(|u| u.host_str().map(|v| v.to_string()))
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_host(url: &str, host: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    url.set_host(Some(host)).ok()?;
    Some(url.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_path(url: &str) -> Option<String> {
    url::Url::parse(url).ok().map(|u| u.path().to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_path(url: &str, path: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    url.set_path(path);
    Some(url.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_query(url: &str) -> Option<String> {
    url::Url::parse(url).ok().and_then(|u| u.query().map(|v| v.to_string()))
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_query(url: &str, query: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    url.set_query(Some(query));
    Some(url.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_fragment(url: &str) -> Option<String> {
    url::Url::parse(url).ok().and_then(|u| u.fragment().map(|v| v.to_string()))
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_fragment(url: &str, query: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    url.set_fragment(Some(query));
    Some(url.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_query_param(url: &str, name: &str) -> Option<String> {
    let url = url::Url::parse(url).ok()?;
    url.query_pairs().find(|q| q.0 == name).map(|v| v.1.to_string())
}

#[pg_extern(immutable, parallel_safe)]
fn url_set_query_param(url: &str, name: &str, value: &str) -> Option<String> {
    let mut url = url::Url::parse(url).ok()?;
    let c = url.clone();
    let q = c.query_pairs();

    url.query_pairs_mut()
        .clear()
        .extend_pairs(q.filter(|it| it.0 != name))
        .append_pair(name, value)
        .finish();

    Some(url.to_string())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_pgurl() {
        assert_eq!(Some("http"), crate::url_scheme("http://abc.com/a").as_deref());
        assert_eq!(Some("abc.com"), crate::url_host("http://abc.com/a").as_deref());
        assert_eq!(Some("/a"), crate::url_path("http://abc.com/a").as_deref());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
