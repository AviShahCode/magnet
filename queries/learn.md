## Basics

Login to postgres shell using Linux postgres user:

```shell
sudo -u postgres pqsl
```

Create user with encrypted password:

```postgresql
CREATE USER username WITH PASSWORD 'password';
```

Create database:

```postgresql
CREATE DATABASE sandbox;
```

Switch owner of database:

```postgresql
ALTER DATABASE sandbox OWNER TO test;
```

Grant to database:

```postgresql
GRANT CREATE, TEMPORARY, CONNECT ON DATABASE sandbox TO username;
```

1. CONNECT $\rightarrow$ connect
2. CREATE $\rightarrow$ create schemas
3. TEMPORARY $\rightarrow$ create temporary tables
4. ALL PRIVILEGES $\rightarrow$ all of the above

Grant usage and create tables on schema:

```postgresql
GRANT USAGE, CREATE ON SCHEMA public TO username;
```

1. USAGE $\rightarrow$ access 
2. CREATE $\rightarrow$ create tables
3. ALL PRIVILEGES $\rightarrow$ all of the above

Grant permissions on tables:

```postgresql
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO username;
```

1. SELECT
2. INSERT
3. UPDATE
4. DELETE
5. TRUNCATE $\rightarrow$ clear table
6. REFERENCES
7. TRIGGER

