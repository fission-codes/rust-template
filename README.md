<div align="center">
  <a href="https://github.com/fission-codes/rust-template" target="_blank">
    <img src="./assets/logo.png" alt="rust-template Logo" width="100"></img>
  </a>

  <h1 align="center">rust-template</h1>

  <p>
    <a href="https://github.com/fission-codes/rust-template/actions?query=">
      <img src="https://github.com/fission-codes/rust-template/actions/workflows/build.yml/badge.svg" alt="Build Status">
    </a>
    <a href="./LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="./LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="https://discord.gg/AunfpqKUHU">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

This template repository is [Fission's][fission] opinionated [Rust][rust] and
[Rust][rust]+[WebAssembly][wasm] (Wasm) project generator, which leverages the
[cargo-generate][cargo-generate] tool.

These templates provide various features for getting-up and running with Rust or
Rust *and* Wasm, including:

* README standardization, code of conduct, contribuing guidelines, and
  a consistent project layout
* GitHub issue and pull-request templates
* Default Rust dependencies (particulary for Wasm)
* [`cargo-bench`][cargo-bench] scaffolding (*optional*)
* Release GitHub Action workflow(s) using the
  [release-please-action][release-please-action] and the
  [release-please][release-please] deploy strategy (*optional*)
  * For Wasm libraries, this includes publishing to npm with
    [wasm-pack][wasm-pack], reliant on the Cargo version for the Wasm package.
* Test, lint, audit, and code coverage (via [Codecov][codecov]) GitHub Action
  workflows (*optional*)
* [Pre-commit][pre-commit] and [rustfmt][rustfmt] opinionated defaults
* [Dependabot][dependabot] support (*optional*)
* [Nix flake][nix-flake] support (*optional*)
* A choice of an [Apache][apache], [MIT][mit], or a dual Apache/MIT license

## Outline

- [Project Templates](#project-templates)
- [Getting Started](#getting-started)
- [Contributing](#contributing)
- [Template References](#template-references)
- [License](#license)

## Project Templates

This repository contains two sub-templates:

* `rust`: for generating a rust-only library or binary/executable project
* `rust+wasm` for generating a [cargo workspace][cargo-workspace]
  with a rust-only crate of the project (library or binary) and another crate
  for wasm-bindings (library-only), meant for execution in [Node.js][node-js]
  or running in modern browsers and/or with bundlers like [webpack][webpack].

## Getting Started

First, install [cargo-generate][cargo-generate] via `cargo install cargo-generate`.
More installation options are available [here][cargo-generate-install].

The experience running through the experience should look something like this:

![cargo-generate Rust Binary Application Screenshot](./assets/screenshot.png)

### Generating a Rust-Only Project

The `rust` template is designed for generating a rust binary or application
library.

- Generate a binary project:

  ``` console
  cargo generate --bin --git https://github.com/fission-codes/rust-template
  ```
*Note on binary crate types*: If using the `--bin` flag, this template will
generate a Rust binary project scaffolding with both a `src/main.rs` and a
`src/lib.rs`. This allows for better support for
[integration testing][integration-testing] and helps with
[separation of concerns][sep-of-concerns].

- Generate an application library project:

  ``` console
  cargo generate --lib --git https://github.com/fission-codes/rust-template
  ```

- Generate a project from src, locally:

  ``` console
  cargo generate --lib --path fission-codes/rust-template/
  ```

*Note on SSH-Keys*: When genearting a project/repository, please be aware
that RSA keys used with SHA-1 signatures are [no longer supported by
GitHub][github-rsa]. There is currently [an issue][cargo-generate-issue] in the
`cargo-generate` repository involving an `id_rsa` default. If you run into an
associated error using the template, please specify your private key when
generating a project/repository like so:

```console
cargo generate -i ~/.ssh/id_ed25519 https://github.com/fission-codes/rust-template
```

#### 🔋 Batteries Included

- [`anyhow`][anyhow] as a ergonomic and idiomatic alternative for explicitly
  handling errors
- [`proptest`][proptest] and [`criterion`][criterion] for generating inputs and
  running benchmarks (*optional*)
- [`tracing`][tracing] for instrumenting Rust programs to collect structured,
  event-based diagnostic information, going beyond just logging-style
  diagnostics
- [`tracing-subscriber`][tracing-subscriber] for Rust binary applications
  to collect trace data, such as by logging it to standard output, and
  consume messages emitted by log-instrumented libraries and modules.

### Generating a Rust+Wasm Workspace Project

The `rust+wasm` template is designed for generating a workspace containing both
a rust-native library, as well as one for compilation to Wasm and leveraging
[wasm-pack][wasm-pack]. We don't currently support any Javascript examples
or frameworks that can use Wasm npm package explicitly, but this is on our
radar.

Generate a project just like before and choose the `rust+wasm` template:

```console
cargo generate --lib --git https://github.com/fission-codes/rust-template
```

*Note*: Currently, `wasm-pack` [does not support building binary
 crates][no-binary], so even with the `--bin` flag specified, a library
will still be generated.

#### 🔋 Batteries Included

- [`wasm-bindgen`][wasm-bindgen] for communicating
  between WebAssembly and JavaScript
- [`wasm-bindgen-futures`][wasm-bindgen-futures] for converting between
  Javascript Promises and Rust futures
- [`console_error_panic_hook`][console-hook]
  for logging panic messages to the developer console.
- [`js-sys`][js-sys] for bindings to Javascript's standard, built-in
  objects
- [`web-sys`][web-sys] for bindings to Web APIs like `window.fetch`, WebGL,
  WebAudio, etc. (*optional*, via feature-flag)

### Notes for Post-Project Generation

- If using `nix` via [Nix flake][nix-flake], please install [nix][nix] and
  [direnv][direnv] to get started. Then, make sure to run `direnv allow`
  and add your files via `git add`.

- If [Codecov][codecov] upload is enabled through GitHub Actions make
  sure to sync your project and gather tokens/badges. Read more
  [here][codecov-quick].

- There are stock integration tests available for all templates, including
  a [wasm-bindgen][wasm-bindgen] decorated test, `#[wasm_bindgen_test]`, that
  can be tested with [wasm-pack][wasm-pack].

- For CI/CD purposes, be aware there's [some secrets you'll need to configure
  in Github][gh-secrets], including:
  * `CODECOV_TOKEN` if you choose to use coverage via [Codecov][codecov]
  * `CARGO_REGISTRY_TOKEN` for publshing Rust packages to [crates.io][crates-io]
  * `NPM_TOKEN` for publishing a Wasm project to [npm][npm]

## Contributing

:balloon: We're thankful for any feedback and help in improving our template
generator! We have a [contributing guide](./CONTRIBUTING.md) to help you get
involved. We also adhere to Fission's [Code of Conduct](./CODE_OF_CONDUCT.md).

### Pre-commit Hook

This repository recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.
- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

## Template References

- [bevy-template-rs][bevy-template]
- [rust-nix-template][rust-nix-template]
- [wasm-pack-template][wasm-pack-template]

## License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0][apache])
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or
  [http://opensource.org/licenses/MIT][mit])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[anyhow]: https://github.com/dtolnay/anyhow
[bevy-template]: https://github.com/taurr/bevy-template-rs
[cargo-bench]: https://doc.rust-lang.org/cargo/commands/cargo-bench.html
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[cargo-generate-install]: https://github.com/cargo-generate/cargo-generate#installation
[cargo-generate-issue]: https://github.com/cargo-generate/cargo-generate/issues/384
[cargo-workspace]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[codecov]: https://about.codecov.io/
[codecov-quick]: https://docs.codecov.com/docs/quick-start
[console-hook]: https://github.com/rustwasm/console_error_panic_hook
[crates-io]: https://crates.io/
[criterion]: https://github.com/bheisler/criterion.rs
[dependabot]: https://github.com/dependabot
[direnv]:https://direnv.net/
[fission]: https://fission.codes/
[github-rsa]: https://github.blog/2021-09-01-improving-git-protocol-security-github/
[gh-secrets]: https://docs.github.com/en/rest/actions/secrets
[integration-testing]: https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests-for-binary-crates
[js-sys]: https://docs.rs/js-sys/latest/js_sys/
[mit]: http://opensource.org/licenses/MIT
[nix]:https://nixos.org/download.html
[nix-flake]: https://nixos.wiki/wiki/Flakes
[node-js]: https://nodejs.dev/en/
[no-binary]: https://github.com/rustwasm/wasm-pack/issues/734
[npm]: https://www.npmjs.com/
[pre-commit]: https://pre-commit.com/
[proptest]: https://github.com/proptest-rs/proptest
[release-please]: https://github.com/googleapis/release-please
[release-please-action]: https://github.com/google-github-actions/release-please-action
[rust]: https://www.rust-lang.org/
[rust-nix-template]: https://github.com/nerosnm/rust-nix-template
[rustfmt]: https://github.com/rust-lang/rustfmt
[sep-of-concerns]: https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects
[tracing]: https://github.com/tokio-rs/tracing
[tracing-subscriber]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html
[wasm]: https://webassembly.org/
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[wasm-bindgen-futures]: https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[wasm-pack-template]: https://github.com/rustwasm/wasm-pack-template
[webpack]: https://webpack.js.org/
[web-sys]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
