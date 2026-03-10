-- as postgresql user

create database magnet;
create user magnetic with encrypted password 'pA5sw0rD';
grant ALL on database magnet to magnetic;

-- as magnetic user

-- user related tables

create table if not exists users
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR(64) UNIQUE NOT NULL,
    hashed   CHAR(64)           NOT NULL,
    salt     CHAR(8)            NOT NULL
);

create table if not exists user_roles
(
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES roles (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

create table if not exists roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(64)
);

insert into roles (name)
values ('admin'),
       ('user');

create table if not exists sessions
(
    id         CHAR(32) PRIMARY KEY,                                    -- TODO: constant
    user_id    INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
    created_at timestamp default now()                         NOT NULL,
    expires_at timestamp default now() + INTERVAL '30 days'    NOT NULL -- TODO: constant
);
-- create index sessions_index ON sessions (id);

-- drive related tables

create schema drive;

create type drive.item_type as ENUM ('folder', 'file');

create table if not exists drive.files
(
    id        CHAR(32) PRIMARY KEY,                                            -- TODO: constant
    name      VARCHAR(256)                                           NOT NULL, -- TODO: constant
    user_id   INTEGER REFERENCES public.users (id) ON DELETE CASCADE NOT NULL,
    item_type drive.item_type                                        NOT NULL,
    parent    CHAR(32) REFERENCES drive.files (id) ON DELETE CASCADE           -- TODO: add folder constraint
);
-- create index files_index on drive.files (parent);

create type drive.share_type as ENUM ('read', 'write');

create table if not exists drive.share
(
    file_id    CHAR(32) REFERENCES drive.files (id) ON DELETE CASCADE NOT NULL,
    to_user    INTEGER REFERENCES users (id) ON DELETE CASCADE        NOT NULL,
    share_type drive.share_type                                       NOT NULL,
    PRIMARY KEY (file_id, to_user)
);

grant all privileges on all tables in schema public to magnetic;
grant all privileges on all sequences in schema public to magnetic;

grant usage on schema drive to magnetic;
grant all privileges on all tables in schema drive to magnetic;
grant all privileges on all sequences in schema drive to magnetic;

-- TODO: make cron job cleanup sessions
truncate sessions;