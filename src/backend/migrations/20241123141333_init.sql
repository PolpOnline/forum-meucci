CREATE TABLE IF NOT EXISTS event
(
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL,
    room            TEXT NOT NULL,
    minimum_section INT  NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "user"
(
    id               SERIAL PRIMARY KEY,
    name             TEXT,
    email            TEXT    NOT NULL UNIQUE,
    interactive_done BOOLEAN NOT NULL DEFAULT FALSE,
    section          INT     NOT NULL DEFAULT 1,
    class            TEXT,
    admin            BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE INDEX ON "user" (email);

CREATE TABLE IF NOT EXISTS event_user
(
    event_id  INT       NOT NULL,
    user_id   INT       NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    round     INT       NOT NULL,
    PRIMARY KEY (user_id, round),
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user" (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS round_max_users
(
    round     INT NOT NULL,
    event_id  INT NOT NULL,
    max_users INT NOT NULL,
    PRIMARY KEY (round, event_id),
    FOREIGN KEY (event_id) REFERENCES event (id) ON DELETE CASCADE
);