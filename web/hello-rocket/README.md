# README

## Prerequisites


### Install Linux Libraris 

If you are using Ubuntu Linux

```bash
sudo apt install -y libpq-dev          # for the PostgreSQL
sudo apt install -y libmysqlclient-dev # for the MySQL
sudo apt install -y libsqlite3-dev     # for the SQLite
```

If you are using Fedora Linux

```bash
sudo dnf install -y postgresql-devel
```

Install `diesel_cli` for use Diesel ORM

### Install Diesel CLI for PostgreSQL

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
