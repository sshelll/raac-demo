PRAGMA foreign_keys = OFF;

BEGIN TRANSACTION;

CREATE TABLE user (
  id INTEGER not null constraint user_pk primary key,
  name TEXT not null
);

INSERT INTO
  user
VALUES
  (1, 'kayce');

INSERT INTO
  user
VALUES
  (2, 'jiale');

INSERT INTO
  user
VALUES
  (3, 'nobody_but_shared');

INSERT INTO
  user
VALUES
  (4, 'real_nobody');

INSERT INTO
  user
VALUES
  (5, 'system_manager');

CREATE TABLE user_preset_role_ref (
  id integer not null constraint user_preset_role_ref_pk primary key,
  user_id integer not null,
  role_id integer not null
);

INSERT INTO
  user_preset_role_ref
VALUES
  (1, 1, 1);

INSERT INTO
  user_preset_role_ref
VALUES
  (2, 5, 2);

CREATE TABLE user_diy_role_ref (
  id integer not null constraint user_diy_role_ref_pk primary key,
  user_id integer not null,
  role_id integer not null
);

INSERT INTO
  user_diy_role_ref
VALUES
  (1, 2, 1);

CREATE TABLE IF NOT EXISTS "actions" (
  id integer not null constraint actions_pk primary key,
  atom text not null,
  desc text,
  resource text
);

INSERT INTO
  actions
VALUES
  (
    1,
    '/system_setting/view',
    'can view system settings',
    'SystemSetting'
  );

INSERT INTO
  actions
VALUES
  (
    2,
    '/system_setting/modify',
    'can modify system settings',
    'SystemSetting'
  );

INSERT INTO
  actions
VALUES
  (
    100000,
    '/talent/view',
    'can view talent',
    'Talent'
  );

INSERT INTO
  actions
VALUES
  (
    100001,
    '/talent/modify',
    'can modify talent',
    'Talent'
  );

INSERT INTO
  actions
VALUES
  (
    100002,
    '/talent/delete',
    'can delete talent',
    'Talent'
  );

INSERT INTO
  actions
VALUES
  (
    100003,
    '/talent/create',
    'can create talent',
    'Talent'
  );

INSERT INTO
  actions
VALUES
  (
    100004,
    '/talent/view_hidden',
    'can view hiden talent',
    'Talent'
  );

CREATE TABLE IF NOT EXISTS "roles_preset" (
  id integer not null constraint roles_preset_pk primary key,
  desc text not null
);

INSERT INTO
  roles_preset
VALUES
  (1, 'SuperAdmin');

INSERT INTO
  roles_preset
VALUES
  (2, 'SystemAdmin');

INSERT INTO
  roles_preset
VALUES
  (3, 'HR');

CREATE TABLE IF NOT EXISTS "roles_diy" (
  id integer not null constraint roles_diy_pk primary key,
  desc text
);

INSERT INTO
  roles_diy
VALUES
  (1, 'DIY_SuperAdmin');

CREATE TABLE IF NOT EXISTS "diy_role_action_ref" (
  id integer not null constraint diy_role_action_ref_pk primary key,
  diy_role_id integer,
  action_id integer
);

INSERT INTO
  diy_role_action_ref
VALUES
  (1, 1, 1);

INSERT INTO
  diy_role_action_ref
VALUES
  (2, 1, 2);

INSERT INTO
  diy_role_action_ref
VALUES
  (3, 1, 100000);

INSERT INTO
  diy_role_action_ref
VALUES
  (4, 1, 100001);

INSERT INTO
  diy_role_action_ref
VALUES
  (5, 1, 100002);

INSERT INTO
  diy_role_action_ref
VALUES
  (6, 1, 100003);

INSERT INTO
  diy_role_action_ref
VALUES
  (7, 1, 100004);

CREATE TABLE talent (
  id integer not null constraint talent_pk primary key,
  hide integer not null
);

INSERT INTO
  talent
VALUES
  (2781, 0);

CREATE TABLE talent_share_ref (
  talent_id integer not null,
  user_id integer not null
);

INSERT INTO
  talent_share_ref
VALUES
  (2781, 2);

INSERT INTO
  talent_share_ref
VALUES
  (2781, 3);

CREATE INDEX user_name_index on user (name);

CREATE INDEX user_preset_role_ref_user_id_index on user_preset_role_ref (user_id);

CREATE UNIQUE INDEX diy_role_action_ref_diy_role_id_action_id_uindex on diy_role_action_ref (diy_role_id, action_id);

CREATE INDEX talent_share_ref_talent_id_index on talent_share_ref (talent_id);

CREATE INDEX talent_share_ref_user_id_index on talent_share_ref (user_id);

COMMIT;
