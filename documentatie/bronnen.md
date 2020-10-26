# Bronnen

These are sources I used and researched to be able to make my project.

## Links

* React functional hooks: https://reactjs.org/docs/hooks-intro.html
* Rust web framework: https://actix.rs/
* Rust orm for mysql/postgresql/sqlite: https://diesel.rs/
* Rust library to use sparkpost api to send emails only: https://crates.io/crates/mailgang
* Rust library used to dealt with toml config files and deserialization into it: https://github.com/alexcrichton/toml-rs
* Rust blog post talking about diesel enumerations: https://atsuzaki.com/blog/diesel-enums/
* Used to resolve the installlation of diesel cli, cli used to use the orm: https://github.com/diesel-rs/diesel/blob/5d87c99592d83eee4c2d7eb51641cb89a3be6f76/diesel_cli/README.md
* rust ORM helper library to write less code for enums: https://github.com/adwhit/diesel-derive-enums
* Project that I wrote a during vacation, I am re-using code of this project: https://github.com/mich2000/identity
* Stackoverflow blog post detailing about necessary features in the project config files to increase compatibility with the diesel orm for enums: https://stackoverflow.com/questions/55783064/the-trait-dieselexpression-is-not-implemented-for-bigdecimalbigdecimal/55783407#55783407
* Rust diesel post on the rust forum talking about how to make a Database connection pool: https://users.rust-lang.org/t/first-baby-steps-with-diesel-r2d2/37858/3
* Rust forum post about converting raw sql to a struct: https://discourse.diesel.rs/t/get-raw-result-without-structuring/80/4
* Rust redis client I use for retaining tokens: https://docs.rs/redis/0.17.0/redis/#basic-operation
* Rust diesel function to count rows: https://docs.diesel.rs/1.4.x/diesel/dsl/fn.count_star.html
* Rust actix-web errors documentation page: https://actix.rs/docs/errors/
* Rust actix-web responses documentation page: https://actix.rs/docs/response/
* Rust actix-web application state documentation page: https://actix.rs/docs/application/
* Rust blog post about redis pools: https://blog.logrocket.com/using-redis-in-a-rust-web-service/
* Rust crate I am using for my private cookies: https://docs.rs/actix-identity/0.3.1/actix_identity/
* Rust repo that is used to implement TLS on the web server: https://github.com/actix/examples/blob/master/rustls/src/main.rs
* Rust crate I use to implement CORS, this is important for when I will start to send requests with React: https://docs.rs/actix-cors/0.5.0/actix_cors/
* Let self-signed certificates through Firefox: https://www.starnet.com/xwin32kb/installing-a-self-signed-certificate-on-firefox/