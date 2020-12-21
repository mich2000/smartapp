# Xam-xam smart application

## Description

Application that will give the possibility to an user to do create, update and delete storages for food. Within these storages for food, food will be able to be added, updated and deleted.

The attributes for food are its name, the number of it and it peremption date. Following the peremption date it will show the food products in the colors:

* Red: Peremption date is over the today's date.
* Yellow: The date of today and the peremption date are the same.
* Green: Peremption date is not over today's date.

## Models

![ ERD of the application ](erd.png)

## Technologies

### Backend

I am going to use the rust as the backend language. For the web framework I am going to use actix. As a database I am going to use mysql or postgresql, and the ORM to manage these will the the diesel ORM. To send emails for recovery codes i am using mailgang, a mini library I wrote that uses sparkpost api to send emails. For the authentication I am using JWT tokens. For the password recovery codes are stored inside a redis memory-database.

I am going to try to separate the authentication and user related task and business sides in 2 micro-services. These 2 microservices will be wrapped into a docker image so to easily ship them.

### Frontend

For the frontend I am using nginx to serve my react application. I will use the service worker to cache static files and control internet access. If the app is offline it will send a json object which will make the app aware it is offline and warns the user about it.

## Scope

### In-scope

Design of the style of the application, rust backend and databases. The only attribute that will hold accountable is the peremption date. Project that shows where the bad products are and not their state.

### Out-scope

Other values, like the nutritive values are not registered in the database.

## Services

For this project I am going to try to seperate the small monolith into microservices. My nginx and my microservices all enable compression for their requests to make transferring lighter. The microservices because off their size will only pool 1 connection to their respective data sources(Postgresq, Redis). If scaling is needed you can tune the service which is loaded a lot.

### Authentication service(Rust actix-web)

This service will manage the user side of the application. This means:

* Registrations of users
* Login: will attach a private cookie(JWT token)
* Confirming email
* Changing email if logged in, if they are a confirmed user
* Changing password if logged in, if they are a confirmed user
* Logging out users
* Sending email with a code for users who forgot their password, this code will be in a redis database and expire.

### Business service(Rust actix-web)

This service will manage business related side of the application, this will only accept and execute requests with a valid JWT. The following things are done by this service:

* CRUD operations of storages
* CRUD operations of products

### Frontend service(Nginx)

Service used to serve react app to the users. Will send out static assets under only https to be fully compatible with service workers. Will also reroute http requests to https.

### Postgres

Database that is used to store all the data that is permanent.

### Redis

Database that temporarily stores the tokens needed to create a user, change their email or is used for passwords that the user forgot to reset these. These tokens will after a time not be good anymore after 10 minutes.