CREATE TABLE event
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
    id               SERIAL PRIMARY KEY,
    name             TEXT,
    email            TEXT      NOT NULL UNIQUE,
    interactive_done BOOLEAN   NOT NULL DEFAULT FALSE,
    class            INT       NOT NULL DEFAULT 1 CHECK ( class > 0 ),
    section          TEXT,
    type             user_type NOT NULL DEFAULT 'normal'
);

CREATE INDEX ON "user" (email);

CREATE TABLE event_user
(
    event_id       INT       NOT NULL REFERENCES event (id) ON DELETE CASCADE,
    user_id        INT       NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
    scheduled_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    joined_at      TIMESTAMP,
    last_edited_by INT REFERENCES "user" (id),
    round          INT       NOT NULL CHECK ( round >= 0 ),
    PRIMARY KEY (user_id, round)
);

CREATE TABLE round_max_users
(
    round     INT NOT NULL,
    event_id  INT NOT NULL REFERENCES event (id) ON DELETE CASCADE,
    max_users INT NOT NULL CHECK ( max_users >= 0 ),
    PRIMARY KEY (round, event_id)
);