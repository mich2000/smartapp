# Bronnen

These are sources I used and researched to be able to make my project.

## Links

### React + Bootstrap + Javascript

* React functional hooks: https://reactjs.org/docs/hooks-intro.html
* React stack overflow post about updating nested objects: https://stackoverflow.com/questions/56802815/react-hooks-how-do-i-update-state-on-a-nested-object-with-usestate
* React hooks updating nested objects and using useEffects and mastering it to update state: https://stackoverflow.com/questions/53428291/react-hooks-modified-state-not-reflecting-immediately
* React modal package to make dialog windows: https://www.npmjs.com/package/react-modal
* React article to generate an option tag from an enum: https://spectrum.chat/palmer/formik/generate-select-option-based-on-enum-type-using-typescript~150d00f7-66d9-40db-ad0d-2ebcae2ca905
* React popup article: https://react-popup.elazizi.com/react-modal/
* React documentation to help me out with routing: https://rookiecoder.medium.com/react-button-click-navigate-to-new-page-6af7397ea220
* React stackoverflow post about history api: https://stackoverflow.com/questions/31079081/programmatically-navigate-using-react-router

### Bootstrap

* Bootstrap table layout: https://getbootstrap.com/docs/4.1/content/tables/
* Bootstrap documentation about colors: https://getbootstrap.com/docs/4.0/utilities/colors/
* Bootstrap icons I use to make icons for my website: https://icons.getbootstrap.com/icons

### Javascript

* Offline and online events: https://developer.mozilla.org/en-US/docs/Web/API/NavigatorOnLine/Online_and_offline_events
* Javascript post about finding out the amount of days between 2 dates: https://stackoverflow.com/questions/2627473/how-to-calculate-the-number-of-days-between-two-dates
* Javascript manner to filter an array: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter
* Service worker that is used to cache files: https://web.dev/offline-cookbook/
* Service worker used to cache files: https://css-tricks.com/serviceworker-for-offline/
* Service worker used to cache files: https://www.digitalocean.com/community/tutorials/demystifying-the-service-worker-lifecycle
* Javascript documentation to set an alert to an user: https://www.w3schools.com/jsref/met_win_alert.asp
* Javascript documentation to set an confirm prompt to an user: https://www.w3schools.com/jsref/met_win_confirm.asp
* Javascript documentation to set an input prompt to an user: https://www.w3schools.com/jsref/met_win_prompt.asp
* Javascript blog post about the context api for state management: https://dev.to/ayushmanbthakur/redux-vs-context-api-3182
* Javascript enums article to explain more about it: https://www.sohamkamani.com/blog/2017/08/21/enums-in-javascript/
* Basic service worker to cache stuff: https://googlechrome.github.io/samples/service-worker/basic/
* React show toast: https://blog.logrocket.com/how-to-create-a-custom-toast-component-with-react/
* Deploying react application: https://create-react-app.dev/docs/deployment/
* .env file doesn't work in react app, prefixing with REACT_APP_ to make it work: https://stackoverflow.com/questions/48378337/create-react-app-not-picking-up-env-files

### Rust

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
* Rust actix example to help me with an user id extractor: https://github.com/emreyalvac/actix-web-jwt/blob/master/src/middlewares/auth.rs
* Rust actix cookies documentation: https://docs.rs/actix-web/3.1.0/actix_web/http/struct.Cookie.html
* Rust session crate: https://docs.rs/actix-session/0.4.0/actix_session/
* Rust post about handling request in a async manner: https://stackoverflow.com/questions/63308246/how-to-use-async-code-in-actix-web-extractors
* Rust post about handling identity in middleware: https://users.rust-lang.org/t/actix-web-token-validation-in-middleware/38205
* Rust ORM manner to embed migrations into application at its start: https://docs.rs/diesel_migrations/1.4.0/diesel_migrations/macro.embed_migrations.html

### Postgres

* Postgres article to help me out with Date datatype: https://www.postgresqltutorial.com/postgresql-date/
* Postgres stackoverflow post about returning columns after inserting something: https://stackoverflow.com/questions/6560447/can-i-use-return-value-of-insert-returning-in-another-insert
* Postgres alter type: https://blog.yo1.dog/updating-enum-values-in-postgresql-the-safe-and-easy-way/
* Postgres problem about sql query where I had problems about sql types: https://stackoverflow.com/questions/57750212/postgresql-error-column-qty-is-of-type-integer-but-expression-is-of-type-text

### Redis

* Redis documentation for security: https://redis.io/topics/config

### Nginx

* Nginx reverse proxy documentation: https://docs.nginx.com/nginx/admin-guide/web-server/reverse-proxy/
* Nginx routing configuration to https and setting up SSL certificats: https://serverfault.com/questions/934893/nginx-https-auto-redirect-to-specific-port

### Firefox

* Let self-signed certificates through Firefox: https://www.starnet.com/xwin32kb/installing-a-self-signed-certificate-on-firefox/

### HTTP protocol

* CORS blog post helping me out with one of the headers: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS/Errors/CORSNotSupportingCredentials
* Post talking about Basic and Bearer HTTP header: https://stackoverflow.com/questions/34013299/web-api-authentication-basic-vs-bearer
* Post about setting up authorization header for basic authentication: https://stackoverflow.com/questions/33505130/how-to-assign-basic-authentication-header-to-xmlhttprequest
* Node development with HTTPS activated: https://create-react-app.dev/docs/using-https-in-development/

### AndroidVectorDrawable to SVG

* Link to npm package that converts the android drawables to svg: https://github.com/neworld/vd2svg