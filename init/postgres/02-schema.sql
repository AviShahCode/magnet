\c magnet;

REVOKE ALL ON SCHEMA public FROM PUBLIC;
GRANT USAGE ON SCHEMA public TO magnetic;

CREATE TABLE IF NOT EXISTS users
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR(64) UNIQUE NOT NULL,
    hashed   CHAR(64)           NOT NULL,
    salt     CHAR(8)            NOT NULL
);

CREATE TABLE IF NOT EXISTS roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(64)
);

CREATE TABLE IF NOT EXISTS user_roles
(
    user_id INTEGER REFERENCES users (id) ON DELETE CASCADE,
    role_id INTEGER REFERENCES roles (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE SCHEMA drive;

CREATE TYPE drive.item_type AS ENUM ('folder', 'file');

CREATE TABLE IF NOT EXISTS drive.files
(
    id        CHAR(22) PRIMARY KEY, -- 16 chars = 22 chars in base64 (no pad)
    name      VARCHAR(256)                                           NOT NULL,
    user_id   INTEGER REFERENCES public.users (id) ON DELETE CASCADE NOT NULL,
    item_type drive.item_type                                        NOT NULL,
    parent    CHAR(22) REFERENCES drive.files (id) ON DELETE CASCADE
);

-- CREATE TYPE drive.share_type AS ENUM ('read', 'write');
--
-- CREATE TABLE IF NOT EXISTS drive.share
-- (
--     file_id    CHAR(22) REFERENCES drive.files (id) ON DELETE CASCADE NOT NULL,
--     to_user    INTEGER REFERENCES users (id) ON DELETE CASCADE        NOT NULL,
--     share_type drive.share_type                                       NOT NULL,
--     PRIMARY KEY (file_id, to_user)
-- );

INSERT INTO roles
VALUES (1, 'admin'),
       (2, 'user');