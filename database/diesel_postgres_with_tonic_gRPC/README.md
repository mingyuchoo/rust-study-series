# README
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
# or
change `.env.test` to `.env`
```

## Generate initial migration

```bash
diesel setup
diesel migration generate <migration_name>
```

## Write the SQL for migration

```bash
# write the SQL for migration
```

## Migrate SQL

```bash
diesel migration run
```

## Check Cargo

```bash
cargo check
```
