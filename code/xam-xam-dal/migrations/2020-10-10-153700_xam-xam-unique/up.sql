-- Your SQL goes here
ALTER TABLE users ADD CONSTRAINT unique_tuple_id_and_email UNIQUE (id, email);
ALTER TABLE storages ADD CONSTRAINT unique_tuple_id_name UNIQUE (id, name);