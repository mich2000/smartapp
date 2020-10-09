-- This file should undo anything in `up.sql`

DROP TABLE users;
DROP TABLE storages;
DROP TABLE products;

DROP TYPE StorageKind;
DROP TYPE ProductKind;