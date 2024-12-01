CREATE TYPE user_type AS enum ('normal', 'host', 'admin');

ALTER TABLE "user" DROP COLUMN admin;
ALTER TABLE "user" ADD COLUMN type user_type DEFAULT 'normal' NOT NULL;