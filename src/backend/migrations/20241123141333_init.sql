CREATE TABLE event
(
    id              SERIAL PRIMARY KEY,
    name            TEXT    NOT NULL,
    description     TEXT    NOT NULL,
    room            TEXT    NOT NULL,
    minimum_section INT     NOT NULL DEFAULT 1 CHECK ( minimum_section > 0 ),
    should_display  BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE TYPE user_type AS enum ('normal', 'host', 'admin');

CREATE TABLE "user"
(
    id               SERIAL PRIMARY KEY,
    name             TEXT,
    email            TEXT      NOT NULL UNIQUE,
    interactive_done BOOLEAN   NOT NULL DEFAULT FALSE,
    section          INT       NOT NULL DEFAULT 1 CHECK ( section > 0 ),
    class            TEXT,
    type             user_type NOT NULL DEFAULT 'normal'
);

CREATE INDEX ON "user" (email);

CREATE TABLE event_user
(
    event_id  INT       NOT NULL,
    user_id   INT       NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    round     INT       NOT NULL CHECK ( round >= 0 ),
    PRIMARY KEY (user_id, round),
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
);

CREATE TABLE round_max_users
(
    round     INT NOT NULL,
    event_id  INT NOT NULL,
    max_users INT NOT NULL CHECK ( max_users >= 0 ),
    PRIMARY KEY (round, event_id),
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE
);