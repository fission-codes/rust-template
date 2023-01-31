This workspace contains numerous examples demonstrating template-generated
code and combinations. Each example is setup as its own crate, with its own
dependencies.

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
