# Opherast

Opherast is a small experimental web service built with [Axum](https://crates.io/crates/axum).
The repository is organised as a Cargo workspace containing the application
server, a CLI tool and a supporting framework crate.

## Building

The project can be compiled with Cargo. To build the entire workspace run:

```bash
cargo build --workspace
```

## Running the web server

From the repository root execute:

```bash
cargo run
```

This starts an HTTP server on `127.0.0.1:8080` that simply replies `"Hello, Opherast!"`.

## Running the CLI

The CLI tool is provided by the `opherast-cli` crate. Invoke it via Cargo:

```bash
cargo run -p opherast-cli -- <command>
```

Alternatively, you can use the alias defined in `.cargo/config.toml` which
maps `cargo opherast` to the command above:

```bash
cargo opherast <command>
```

Run the `list` command to see the available commands.

## Crates overview

- **opherast** – the main Axum based web server.
- **opherast-framework** – library with shared utilities and a small framework
  used by both the server and the CLI. It provides environment checks,
  scaffolding helpers and common result types.
- **opherast-cli** – binary crate exposing command line tools built on top of
  `opherast-framework`.

## License

This project is licensed under the terms of the MIT license. See
[LICENSE](LICENSE) for details.
