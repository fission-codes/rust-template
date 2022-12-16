<div align="center">
  <a href="https://github.com/{{github-name}}/{{project-name}}" target="_blank">
    <img src="https://raw.githubusercontent.com/{{github-name}}/{{project-name}}/main/assets/a_logo.png" alt="{{project-name}} Logo" width="100"></img>
  </a>

  <h1 align="center">{{project-name}}</h1>

  <p>
    <a href="https://crates.io/crates/{{project-name}}">
      <img src="https://img.shields.io/crates/v/{{project-name}}?label=crates" alt="Crate">
    </a>
    <a href="https://npmjs.com/package/{{project-name}}">
      <img src="https://img.shields.io/npm/v/{{project-name}}" alt="Npm">
    </a>{% if codecov %}
    <a href="https://codecov.io/gh/{{github-name}}/{{project-name}}">
      <img src="https://codecov.io/gh/{{github-name}}/{{project-name}}/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>{% endif %}{% if github_actions %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/actions?query=">
      <img src="https://github.com/{{github-name}}/{{project-name}}/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>{% endif %}{% if license == "Apache" %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>{% elsif license == "MIT" %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
    </a>{% else %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/blob/main/LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://github.com/{{github-name}}/{{project-name}}/blob/main/LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>{% endif %}
    <a href="https://docs.rs/{{project-name}}">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>{% if have_discord %}
    <a href="{{discordlink}}">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>{% endif %}
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Crates](#crates)
- {% if crate_type == "lib" %}[Usage](#usage){% else %}[Usage and Installation](#usage-and-installation){% endif %}
- [Testing the Project](#testing-the-project){% if bench %}
- [Benchmarking the Project](#benchmarking-the-project){% endif %}{% if docker %}
- [Running {{project-name}} on Docker](#running-{{project-name}}-on-docker){% endif %}
- [Setting-up {{project-name}}-wasm](#setting-up-{{project-name}}-wasm)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Crates

- [{{project-name}}](https://github.com/{{github-name}}/{{project-name}}/tree/main/{{project-name}})
- [{{project-name}}-wasm](https://github.com/{{github-name}}/{{project-name}}/tree/main/{{project-name-wasm}})
{% if crate_type == "lib" %}
## Usage

- Add the following to the `[dependencies]` section of your `Cargo.toml` file
  for using the rust-only `{{project-name}}` crate/workspace:

```toml
{{project-name}} = "0.1.0"
```

- Add the following to the `[dependencies]` section of your `Cargo.toml` file
  for using `{{project-name}}-wasm` crate/workspace:

```toml
{{project-name}}-wasm = "0.1.0"
```
{% else %}
## Usage and Installation

### Using `cargo`

This is just for the rust-only `{{project-name}}` binary application:

```console
$ cargo install {{project-name}}
```

### {{project-name}}-wasm Usage

Due to the reliance on [wasm-pack][wasm-pack], `{{project-name}}-wasm` is only
available as a library.

- Add the following to the `[dependencies]` section of your `Cargo.toml` file
  for using `{{project-name}}-wasm` crate/workspace:

```toml
{{project-name}}-wasm = "0.1.0"
```
{% endif %}
## Testing the Project

- Run tests for crate/workspace `{{project-name}}`:
{% if bench %}
  ```console
  cd {{project-name}} && cargo test --all-features
  ```
{% else %}
  ```console
  cd {{project-name}} && cargo test
  ```
{% endif %}
- To run tests for crate/workspace `{{project-name}}-wasm`, follow
  the instructions in [{{project-name}}-wasm](./{{project-name}}-wasm#testing-the-project),
  which leverages [wasm-pack][wasm-pack].
{% if bench %}
## Benchmarking the Project

For benchmarking and measuring performance, this workspaces provides
a Rust-specific benchmarking package leveraging [criterion][criterion] and a
`test_utils` feature flag for integrating [proptest][proptest] within the
suite for working with [strategies][strategies] and sampling from randomly
generated values.

- Run benchmarks

  ```console
  cargo bench -p {{project-name}}-benches
  ```

*Note*: Currently, this workspace only supports Rust-native benchmarking, as
`wasm-bindgen` support for criterion is still [an open issue][criterion-bindgen].
However, with some extra work, benchmarks can be compiled to [wasi][wasi] and
run with [wasmer][wasmer]/[wasmtime][wasmtime] or in the brower with
[webassembly.sh][wasmsh]. Please catch-up with wasm support for criterion on their
[user-guide][criterion-user-guide].
{% endif %}{% if docker %}
## Running {{project-name}} on Docker

We recommend setting your [Docker Engine][docker-engine] configuration
with `experimental` and `buildkit` set to `true`, for example:

``` json
{
  "builder": {
    "gc": {
      "defaultKeepStorage": "20GB",
      "enabled": true
    }
  },
  "experimental": true,
  "features": {
    "buildkit": true
  }
}
```

- Build a multi-plaform Docker image via [buildx][buildx] (from top-level):

  ```console
  docker buildx build --platform=linux/amd64,linux/arm64 --file docker/Dockerfile -t {{project-name}} --progress=plain .
  ```

- Run a Docker image (depending on your platform):

  ```console
  docker run --platform=linux/amd64 -t {{project-name}}
  ```

*Note*: The current [Dockerfile](./docker/Dockerfile) just builds the Rust-native
binary on Docker. We can eventually take advntage of baking-in a front-end with
the Wasm library, etc.
{% endif %}
## Setting-up {{project-name}}-wasm

The Wasm targetted version of this project relies on [wasm-pack][wasm-pack]
for building, testing, and publishing artifacts sutiable for
[Node.js][node-js], web broswers, or bundlers like [webpack][webpack].

Please read more on working with `wasm-pack` directly in
[{{project-name}}-wasm](./{{project-name}}-wasm#set-up).

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a [contributing guide](./CONTRIBUTING.md) to help you get involved. We
also adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).
{% if nix %}
### Nix
This repository contains a [Nix flake][nix-flake] that initiates both the Rust
toolchain set in [rust-toolchain.toml](./rust-toolchain.toml) and a
[pre-commit hook](#pre-commit-hook). It also installs helpful cargo binaries for
development. Please install [nix][nix] and [direnv][direnv] to get started.

Run `nix develop` or `direnv allow` to load the `devShell` flake output,
according to your preference.

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend by default.
{% else %}
### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.
{% endif %}
### Pre-commit Hook

This library recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.
{% if nix == false %}
- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.
{% endif %}
- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow
{% if nix %}
- We recommend leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.
- We recommend using [cargo-udeps][cargo-udeps] for removing unused dependencies
  before commits and pull-requests.
{% else %}
- We recommend installing and leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.
{% endif %}
### Conventional Commits

This project *lightly* follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help
{% if have_discord %}
For usage questions, usecases, or issues reach out to us in our [Discord channel]({{discordlink}}).
{% else %}
For usage questions, usecases, or issues please open an issue in our repository.
{% endif %}
We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License
{% if license == "Apache" %}
This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].
{% elsif license == "MIT" %}
This project is licensed under the [MIT License](./LICENSE),
or [http://opensource.org/licenses/MIT][mit].
{% else %}
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0][apache])
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT][mit])

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
{% endif %}

[apache]: https://www.apache.org/licenses/LICENSE-2.0{% if docker %}
[buildx]: https://github.com/docker/buildx{% endif %}
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/{% if bench %}
[criterion]: https://github.com/bheisler/criterion.rs
[criterion-bindgen]: https://github.com/bheisler/criterion.rs/issues/270{% endif %}{% if docker%}
[docker-engine]: https://docs.docker.com/engine/{% endif %}{% if nix %}
[direnv]:https://direnv.net/{% endif %}
[irust]: https://github.com/sigmaSd/IRust
[mit]: http://opensource.org/licenses/MIT{% if nix %}
[nix]:https://nixos.org/download.html
[nix-flake]: https://nixos.wiki/wiki/Flakes{% endif %}
[node-js]: https://nodejs.dev/en/
[pre-commit]: https://pre-commit.com/{% if bench %}
[proptest]: https://github.com/proptest-rs/proptest
[strategies]: https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html
[criterion-user-guide]: https://github.com/bheisler/criterion.rs/blob/version-0.4/book/src/user_guide/wasi.md
[wasi]: https://wasi.dev/
[wasmer]: https://wasmer.io/
[wasmtime]: https://docs.wasmtime.dev/
[wasmsh]: https://webassembly.sh/{% endif %}
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[webpack]: https://webpack.js.org/
