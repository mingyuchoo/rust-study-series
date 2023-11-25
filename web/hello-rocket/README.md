# README
## Prerequisites

If you use Fedora Linux

```bash
sudo dnf install -y postgresql-devel
```

Install `diesel_cli` for use Diesel ORM

```bash
cargo install diesel_cli --no-default-features --features postgres
```

## Create a new Rust project

```bash
cargo new <project_name> --bin
cd <project_name>
```

## Create a database environment file

```bash
echo DATABASE_URL=postgresql://postgres:postgres@localhost:5432/postgres > .envrc
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
# write the SQL for migrtions
```

## Migrate SQL

```bash
diesel migration run
```

## Check Cargo

```bash
cargo check
```
