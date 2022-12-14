# PG_URL

Implements URL manipulation methods as a postgres extension. Using this extension will allow you to work with URLS
directly inside database. You could use it to create an index on a URL host

```sql
CREATE INDEX tbl_url_host on tbl (url_host(url));
```

### Installation

1. Download the latest [Release](https://github.com/semtexzv/pg_url/releases).
2. Install it using your favorite package manager:

    ```bash
    dpkg -i OR rpm -i <Downloaded package>  
    ```

3. Enable it in postgres:

    ```sql
    CREATE EXTENSION IF NOT EXISTS pg_url;
    ```

### Schema
What functions does this extension define? This is the schema:

```sql
-- src/lib.rs:11
-- pg_url::url_set_scheme
CREATE FUNCTION "url_set_scheme"(
    "url" TEXT, /* &str */
    "scheme" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_scheme_wrapper';

-- src/lib.rs:79
-- pg_url::url_set_query_param
CREATE FUNCTION "url_set_query_param"(
    "url" TEXT, /* &str */
    "name" TEXT, /* &str */
    "value" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_query_param_wrapper';

-- src/lib.rs:54
-- pg_url::url_set_query
CREATE FUNCTION "url_set_query"(
    "url" TEXT, /* &str */
    "query" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_query_wrapper';

-- src/lib.rs:42
-- pg_url::url_set_path
CREATE FUNCTION "url_set_path"(
    "url" TEXT, /* &str */
    "path" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_path_wrapper';

-- src/lib.rs:23
-- pg_url::url_set_host
CREATE FUNCTION "url_set_host"(
    "url" TEXT, /* &str */
    "host" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_host_wrapper';

-- src/lib.rs:66
-- pg_url::url_set_fragment
CREATE FUNCTION "url_set_fragment"(
    "url" TEXT, /* &str */
    "query" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_set_fragment_wrapper';

-- src/lib.rs:6
-- pg_url::url_scheme
CREATE FUNCTION "url_scheme"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_scheme_wrapper';

-- src/lib.rs:73
-- pg_url::url_query_param
CREATE FUNCTION "url_query_param"(
    "url" TEXT, /* &str */
    "name" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_query_param_wrapper';

-- src/lib.rs:49
-- pg_url::url_query
CREATE FUNCTION "url_query"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_query_wrapper';

-- src/lib.rs:37
-- pg_url::url_path
CREATE FUNCTION "url_path"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_path_wrapper';

-- src/lib.rs:18
-- pg_url::url_host
CREATE FUNCTION "url_host"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_host_wrapper';

-- src/lib.rs:61
-- pg_url::url_fragment
CREATE FUNCTION "url_fragment"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_fragment_wrapper';

-- src/lib.rs:30
-- pg_url::url_clear_host
CREATE FUNCTION "url_clear_host"(
    "url" TEXT /* &str */
) RETURNS TEXT /* core::option::Option<alloc::string::String> */
    IMMUTABLE STRICT PARALLEL SAFE
    LANGUAGE c /* Rust */
AS
'MODULE_PATHNAME',
'url_clear_host_wrapper';
```