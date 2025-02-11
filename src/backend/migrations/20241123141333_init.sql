CREATE TABLE activity
(
    id             SERIAL PRIMARY KEY,
    name           TEXT    NOT NULL,
    description    TEXT    NOT NULL,
    room           TEXT    NOT NULL,
    minimum_class  INT     NOT NULL DEFAULT 1 CHECK ( minimum_class > 0 ),
    should_display BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TYPE user_type AS enum ('normal', 'host', 'admin');

CREATE TABLE "user"
(
    id      SERIAL PRIMARY KEY,
    name    TEXT,
    email   TEXT      NOT NULL UNIQUE,
    class   INT       NOT NULL DEFAULT 1 CHECK ( class > 0 ),
    section TEXT,
    type    user_type NOT NULL DEFAULT 'normal'
);

CREATE INDEX ON "user" (email);

CREATE TABLE activity_user
(
    activity_id              INT       NOT NULL REFERENCES activity (id) ON DELETE CASCADE,
    user_id                  INT       NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
    scheduled_at             TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    joined_at                TIMESTAMP,
    joined_at_last_edited_by INT REFERENCES "user" (id),
    round                    INT       NOT NULL CHECK ( round >= 0 ),
    randomized               BOOLEAN   NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_id, round)
);

CREATE TABLE activity_admin
(
    activity_id INT NOT NULL REFERENCES activity (id),
    user_id     INT NOT NULL REFERENCES "user" (id)
);

CREATE TABLE round_max_users
(
    round       INT NOT NULL,
    activity_id INT NOT NULL REFERENCES activity (id) ON DELETE CASCADE,
    max_users   INT NOT NULL CHECK ( max_users >= 0 ),
    PRIMARY KEY (round, activity_id)
);