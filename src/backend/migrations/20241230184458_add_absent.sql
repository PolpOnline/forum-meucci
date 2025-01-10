DO
$$
    DECLARE
        event_id                 INTEGER;
        DECLARE max_int CONSTANT INTEGER := 2147483647;
    BEGIN
        INSERT INTO event
            (name, description, room, minimum_class, should_display)
        VALUES ('absent', '', '', max_int, FALSE) -- Absent should not be shown in the list of events
        RETURNING id INTO event_id;

        FOR i IN 0..3
            LOOP
                INSERT INTO round_max_users
                    (round, event_id, max_users)
                VALUES (i, event_id, max_int); -- Absent should not have a limit of participants
            END LOOP;
    END
$$;