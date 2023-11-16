# diesel_postgres_init

## Prerequisite

If you use Fedora Linux

```bash
sudo dnf install postgresql-devel
```

install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Create a new Rust project

```bash
cargo new <project_name> --lib
cd <project_name>
```

## Create a database environment file

```bash
echo DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres > .env

# or change `.env.test` to `.env`
```

## Generate initial migrations

```bash
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migrations

```bash
# write the SQL for migrations
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
