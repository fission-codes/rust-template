This workspace contains numerous examples demonstrating template-generated
code and combinations. Each example is setup as its own crate, with its own
dependencies.

## Cargo.lock Synchronization

The workspace automatically maintains synchronized `Cargo.lock` files for all
example templates. When dependencies are updated at the workspace level, a
pre-commit hook automatically copies the workspace `Cargo.lock` to each example
directory that contains a `Cargo.toml` file. This ensures all templates have
up-to-date dependency resolution for use with `cargo generate`.

This synchronization happens automatically during commits when `Cargo.toml` or
`Cargo.lock` files are modified. You can also run it manually:

```console
./scripts/copy-example-lockfiles.sh
```

## Running Examples

To run any example from this top-level:

```console
cargo run -p <example>
```

For example, `cargo run -p gen-axum --all-features` will run a *mostly*
stock-generated rust [axum][axum] server.

## Running Tests

- Run tests

  ```console
  cargo test -p <example>
  ```

[axum]: https://docs.rs/axum/latest/axum/
