# Development

## Prerequisites

For Ubuntu Linux

```bash
$ sudo apt update
$ sudo apt install libwebkit2gtk-4.1-dev \
                   build-essential \
                   pkg-config \
                   libgtk-3-dev \
                   libssl-dev \
                   libsoup-3.0-dev \
                   libxdo-dev

$ cargo install cargo-binstall
$ cargo binstall dioxus-cli
```

## Create a new project

```bash
$ dx new my-app
$ cd my-app
```

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform desktop
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```
