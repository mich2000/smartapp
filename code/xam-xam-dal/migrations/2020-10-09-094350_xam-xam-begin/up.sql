-- Your SQL goes here
CREATE TABLE "users"
(
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL
);

CREATE TYPE StorageKind AS ENUM ('Other','Closet','Fridge','Freezer');

CREATE TYPE ProductKind AS ENUM ('Other','Vegetables','Fruit','Grain','Meat','Fish','Dairy','FatAndSugar','Bean');

CREATE TABLE "storages" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    storage_kind StorageKind NOT NULL
);

CREATE TABLE "products"(
    id SERIAL PRIMARY KEY,
    storage_id INTEGER NOT NULL REFERENCES storages(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    amount INTEGER NOT NULL,
    peremption_date DATE NOT NULL,
    product_kind ProductKind NOT NULL,
);