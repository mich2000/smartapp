# Timesheet

Timesheet to demonstrate my learning and making to complete my project.

## Week 1

* Course: 4h
* Learning on a small react project I made using rust backend: 2h

## Week 2

* Conversation with instructor to discuss project: 20m
* Working on a mini library for JWT tokens later to be used in my project: 4h
* Working on a mini library for JWT tokens and my models: 3h
* Working on a working database connection between and models: 1h30

## Week 3

* Conversation with instructor to discuss project: 10m
* Working on database models(adding some constraints), adding errors and some utilities(regex, random hash giver) to use: 4h
* Working on enumeration problem: 1h
* Working on problems related to enums, this is difficult between the diesel orm and the database: 2h30
* Working on my models and making database insertable structs for them: 1h30

## Week 4

* Conversation with instructor to discuss project: 10m
* Working on the data access layer(focused on the user): 3h
* Working on my data access layer: 3h
* Working on jwt configuration, data access layer and setting up the basic project for the service authentication: 2h
* Working on my service layer and adjusting my data access layer: 4h
* Working on the service layer and a little bit on my models: 1h
* Working on my business layer for the authentication service and a little bit on my data access layer: 2h
* Working on my business layer and error handling: 3h
* Working on my web layer and on the errors: 1h30
* Working a bit on my models: 20m
* Working on my business layer and a bit on my web layer for my authentication service: 2h

## Week 5
* Conversation with instructor to discuss project: 10m
* Working on my business layer, web layer and fixing problems with sending emails: 4h30
* Working primarly on the web layer: 2h30
* Working on my web layer for identification and HTTPS2: 1h
* Working on my web layer(https, cors) and beginning my react app: 3h
* Working on my web layer and react app, troubleshooting errors and cors: 4h
* Working on my web layer and react app, solving errors: 3h
* Working on my react app, trying to learn functional hooks: 2h
* Working on login problem, was resolved: 1h
* Working on my web layer and react app: 2h30
* Working on problems related to cookies: 2h
* Working on my web layer to make the cookies stay until their max age is reached and also on my react app: 3h

## Week 6

* Working on my web layer and meeting with instructor: 1h15
* Working on my web layer and react app, focusing on retrieval of forgotten password: 2h45
* Working on my react app, web layer and service layer: 2h30
* Working on my react app, web layer. Solve problems related to middleware and jwt token in private cookies: 5h
* Working on my web, service layer and my react app. Trying to solve problems related to credential middleware and changing passwords: 3h30
* Working a bit on my react app: 25min
* Working on my api and react app, finish up the auth service: 1h30
* Learning to use the context api from react: 2h
* Beginning to transition my session in my react app to more react hooks and context api management: 2h30
* Working on state management within my react app, and working a bit on the cors side of my web api: 2h25
* Working on my react app and converting a class to function: 50min
* Working on my react app fixing bugs in it, testing my api and app on linux: 1h
* Working on my web layer and beginning with my business service and api for my storages and products: 1h15
* Working on my business and web business micro service: 1h
* Working on my storage logic and storage dialogs: 4h
* Working on my react application, to add and remove storages: 4h

## Week 7

* Working on my react layout: 2h
* Working on my auth web service and my react app: 1h30
* Working on my service worker: 2h15
* Working on my service worker: 3h
* Working on my service worker and working on updating storages: 3h30
* Working on my react app, working on the DAL of my api to implement some CRUD features: 4h
* Working on my DAL and business service layer for my api: 1h30
* Working on my business api and react app for the products page: 1h45
* Working on my routes within my backend api and repaire the product routing: 45min
* Refactoring git repo, and my react app. Working on my react app: 2h

## Week 8

* Working on my react app and discussing with instructor: 1h30
* Working on my react app and trying to solve problems related to date serialization: 3h15
* Problem related to a custom sql query to try to solve an insert: 3h30
* Making dockerfiles for my services and nginx: 10m
* Working on delete and add feature for products in my api and frontend: 1h30
* Working on my react app, the home page of an authenticated user and the nav bar: 1h
* Working on bugs in my react app having to do with crud operations not resulting in proper UI changes and I worked on my backend to be able to update products: 1h

## Week 9

* Convert android drawables from the previous android project to svg icons: 10m
* Updating feature for products in storages: 1h30
* Working on my react app and backend to show the 5 oldest products of an user: 3h
* Working on setting icons on my web app: 30min
* Working react app: 2h30
* Working on the usability of my app: 20min

## Week 10

* Working on my production config, in my react app and docker-compose.yml: 2h20
* Trying to deploy my app with docker and docker-compose, didn't succeed problems with ports, volumes and build of rust app: 23h20
* Solving bugs related to the deployment to production of the webapp and api: 6h

## Deployment problems

Errors I had:
* Cannot deploy a volume on the root of a container.
* Cannot deploy a container whose port is already being listened to.
* Rust build of the authentication service fails on my vps but not on my linux desktop.
* Rust building of Authentication service error message: 
```process didn't exit successfully: `rustc --crate-name xam_xam_id_web --edition=2018 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=z -C panic=abort -C lto -C codegen-units=1 -C metadata=1de7fef8615bf336 -C extra-filename=-1de7fef8615bf336 --out-dir /xam-xam-id-web/target/release/deps -L dependency=/xam-xam-id-web/target/release/deps --extern actix_cors=/xam-xam-id-web/target/release/deps/libactix_cors-d9df84cc0c22cf5b.rlib --extern actix_identity=/xam-xam-id-web/target/release/deps/libactix_identity-679ab82e63f83f3d.rlib --extern actix_web=/xam-xam-id-web/target/release/deps/libactix_web-02e91e66f8948eb7.rlib --extern actix_web_httpauth=/xam-xam-id-web/target/release/deps/libactix_web_httpauth-a31192eda17bb5b4.rlib --extern futures_util=/xam-xam-id-web/target/release/deps/libfutures_util-51c5483afad7606b.rlib --extern jwt_gang=/xam-xam-id-web/target/release/deps/libjwt_gang-ef8c9d7b564d53ef.rlib --extern log=/xam-xam-id-web/target/release/deps/liblog-9f954c959ce6348b.rlib --extern log4rs=/xam-xam-id-web/target/release/deps/liblog4rs-561ae4aa8ca33383.rlib --extern mailgang=/xam-xam-id-web/target/release/deps/libmailgang-c317905a00803e87.rlib --extern rustls=/xam-xam-id-web/target/release/deps/librustls-02e9145bfe52ad91.rlib --extern xam_xam_common=/xam-xam-id-web/target/release/deps/libxam_xam_common-378dec558d999979.rlib --extern xam_xam_id_bll=/xam-xam-id-web/target/release/deps/libxam_xam_id_bll-ee123005831b19df.rlib -L native=/xam-xam-id-web/target/release/build/ring-287f767d5dbcec7c/out -L native=/xam-xam-id-web/target/release/build/brotli-sys-69dd56223ff5bb19/out -L native=/usr/lib/x86_64-linux-gnu` (signal: 9, SIGKILL: kill)```
* Another docker compose error linked to SSL certificates: `error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory`
* Open ports on the linux instance.
* Make sure you map correctly ports to the ports of your api.
* Make sure the smtp domain that you configure is right, an error I overlooked quite easily.
* Make sure you SSL certificates are changes: `error:1416F086:SSL routines:tls_process_server_certificate:certificate verify failed:../ssl/statem/statem_clnt.c:1245: (unable to get local issuer certificate)`
* Make sure that the timezone and ntp server is okay on your VPS to not get certificate errors.