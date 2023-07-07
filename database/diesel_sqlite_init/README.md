# diesel_sqlite_init

## Prerequisite

install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

## Create a new Rust project

```bash
cargo new <project_name> --lib
cd <project_name>
```

## Create a database environment file

```bash
echo DATABASE_URL=mydb.sqlite3 > .env

# or change `.env.test` to `.env`
```

## Generate initial migrations

```bash
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migrations

In `up.sql`

```sql
# write the SQL for migrations
CREATE TABLE posts (
  id INTEGER        NOT NULL PRIMARY KEY AUTOINCREMENT,
  title TEXT        NOT NULL,
  body  TEXT        NOT NULL,
  published INTEGER NOT NULL
);
```

In `down.sql`

```sql
DROP TABLE posts;
```

## Migrate SQL

```bash
diesel migration run
```

## Check Cargo

```bash
cargo check
```

## How to use this application

```bash
cargo run --bin show_posts
cargo run --bin write_post
cargo run --bin publish_post
cargo run --bin delete_post
```