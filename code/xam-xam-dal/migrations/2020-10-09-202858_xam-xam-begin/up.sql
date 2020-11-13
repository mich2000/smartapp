-- Your SQL goes here
CREATE TABLE "users"
(
    id SERIAL PRIMARY KEY,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    UNIQUE (email)
);

CREATE TYPE StorageKind AS ENUM ('other','closet','fridge','freezer');
CREATE TYPE ProductKind AS ENUM ('other','vegetables','fruit','grain','meat','fish','dairy','fat_and_sugar','bean');

CREATE TABLE "storages" (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    storage_kind StorageKind NOT NULL,
    UNIQUE (user_id,name)
);

CREATE TABLE "products"(
    id SERIAL PRIMARY KEY,
    storage_id INTEGER NOT NULL REFERENCES storages(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    amount SMALLINT NOT NULL CHECK(amount > 0),
    peremption_date DATE NOT NULL,
    product_kind ProductKind NOT NULL,
    UNIQUE (id,name)
);