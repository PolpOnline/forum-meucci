CREATE TABLE event_admin
(
    event_id INT NOT NULL,
    user_id  INT NOT NULL,
    FOREIGN KEY (event_id) REFERENCES event (id),
    FOREIGN KEY (user_id) REFERENCES "user" (id)
);