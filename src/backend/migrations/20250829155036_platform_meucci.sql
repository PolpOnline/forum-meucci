ALTER TABLE activity
    RENAME TO forum_activity;

ALTER TYPE user_type RENAME TO forum_user_role;

ALTER TABLE "user"
    RENAME COLUMN type TO forum_role;

ALTER TABLE activity_user
    RENAME TO forum_activity_user;

ALTER TABLE activity_admin
    RENAME TO forum_activity_host;

ALTER TABLE round_max_users
    RENAME TO forum_round_max_users;

ALTER TABLE admin_register_call
    RENAME TO forum_host_register_call;
