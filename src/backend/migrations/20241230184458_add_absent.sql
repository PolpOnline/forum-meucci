DO
$$
    DECLARE
        activity_id INTEGER;
        DECLARE max_int CONSTANT INTEGER := 2147483647;
    BEGIN
        INSERT INTO activity
            (name, description, room, minimum_class, should_display)
        VALUES ('absent', '', '', max_int, FALSE) -- Absent should not be shown in the list of activities
        RETURNING id INTO activity_id;

        FOR i IN 0..3
            LOOP
                INSERT INTO round_max_users
                    (round, activity_id, max_users)
                VALUES (i, activity_id, max_int); -- Absent should not have a limit of participants
            END LOOP;
    END
$$;