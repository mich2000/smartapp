-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT unique_tuple_id_and_email;
ALTER TABLE storages DROP CONSTRAINT unique_tuple_id_name;