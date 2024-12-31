DO
$$
    DECLARE
        event_id        INTEGER;
        DECLARE max_int INTEGER := 2147483647;
    BEGIN
        INSERT INTO event
            (name, description, room, minimum_section)
        VALUES ('Assente', '', '', 1)
        RETURNING id INTO event_id;

        FOR i IN 0..3
            LOOP
                INSERT INTO round_max_users
                    (round, event_id, max_users)
                VALUES (i, event_id, max_int);
            END LOOP;
    END
$$;