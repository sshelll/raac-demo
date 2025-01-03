CREATE TABLE user (
  id INTEGER not null constraint user_pk primary key,
  name TEXT not null
);

CREATE TABLE user_preset_role_ref (
  id integer not null constraint user_preset_role_ref_pk primary key,
  user_id integer not null,
  role_id integer not null
);

CREATE TABLE user_diy_role_ref (
  id integer not null constraint user_diy_role_ref_pk primary key,
  user_id integer not null,
  role_id integer not null
);

CREATE TABLE IF NOT EXISTS "actions" (
  id integer not null constraint actions_pk primary key,
  atom text not null,
  desc text,
  resource text
);

CREATE TABLE IF NOT EXISTS "roles_preset" (
  id integer not null constraint roles_preset_pk primary key,
  desc text not null
);

CREATE TABLE IF NOT EXISTS "roles_diy" (
  id integer not null constraint roles_diy_pk primary key,
  desc text
);

CREATE TABLE IF NOT EXISTS "diy_role_action_ref" (
  id integer not null constraint diy_role_action_ref_pk primary key,
  diy_role_id integer,
  action_id integer
);

CREATE INDEX user_name_index on user (name);

CREATE INDEX user_preset_role_ref_user_id_index on user_preset_role_ref (user_id);

CREATE UNIQUE INDEX diy_role_action_ref_diy_role_id_action_id_uindex on diy_role_action_ref (diy_role_id, action_id);

CREATE TABLE talent (
  id integer not null constraint talent_pk primary key,
  hide integer not null
);

CREATE TABLE talent_share_ref (
  talent_id integer not null,
  user_id integer not null
);

CREATE INDEX talent_share_ref_talent_id_index on talent_share_ref (talent_id);

CREATE INDEX talent_share_ref_user_id_index on talent_share_ref (user_id);
