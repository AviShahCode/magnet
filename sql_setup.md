postgresql user

```postgresql
create database magnet;
create user magnetic with encrypted password 'pA5sw0rD';
grant ALL on database magnet to magnetic;
```

magnetic user

```postgresql
grant all on schema public to magnetic;

create table users
(
    id       SERIAL PRIMARY KEY,
    username varchar(32) UNIQUE NOT NULL,
    hashed   char(64)           NOT NULL,
    -- need to add salted + hashed password
    CONSTRAINT min_username_len CHECK ( LENGTH(username) >= 8 )
);

-- pw: password
insert into users (username, hashed)
VALUES ('avi_shah', '5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8');
-- pw: user1234
insert into users (username, hashed)
VALUES ('user1234', '831c237928e6212bedaa4451a514ace3174562f6761f6a157a2fe5082b36e2fb');
select *
from users;

create table roles
(
    id   SERIAL PRIMARY KEY,
    role VARCHAR(64)
);

insert into roles (role)
values ('admin');
insert into roles (role)
values ('user');

create table user_roles
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES users (id),
    role_id INT REFERENCES roles (id)
);

insert into user_roles (user_id, role_id)
values (1, 1);
insert into user_roles (user_id, role_id)
values (2, 2);

create table sessions
(
    id      SERIAL PRIMARY KEY,
    user_id INT REFERENCES users (id) NOT NULL,
    token   CHAR(32)                  NOT NULL UNIQUE,
    iat     TIMESTAMP                 NOT NULL DEFAULT now(),
    exp     TIMESTAMP GENERATED ALWAYS AS (iat + INTERVAL '2 hours') STORED
);

grant ALL on all TABLES in SCHEMA public to magnetic;
grant ALL on all SEQUENCES in SCHEMA public to magnetic;

select *
from sessions;

-- drop table users, roles, user_roles, sessions;


select 12::bytea as a;

SET bytea_output = 'hex';
insert into sessions (user_id, token)
values (2, 'a5e3ec9aa60acf55ee91ad3c28a75f28');

select r.role, u.username
from users u,
     user_roles ur,
     roles r
where u.id = ur.user_id
  and ur.id = r.id;

CREATE TYPE drive_item_type AS ENUM ('folder', 'file');

create table drive
(
    id        SERIAL PRIMARY KEY,
    user_id   INT             NOT NULL,
    filename  TEXT            NOT NULL,
    item_type drive_item_type NOT NULL,
    parent_id INTEGER REFERENCES drive (id)
)
```