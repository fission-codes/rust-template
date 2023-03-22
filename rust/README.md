<div align="center">
  <a href="https://github.com/{{github-name}}/{{repo-name}}" target="_blank">
    <img src="https://raw.githubusercontent.com/{{github-name}}/{{repo-name}}/main/assets/a_logo.png" alt="{{repo-name}} Logo" width="100"></img>
  </a>

  <h1 align="center">{{project-name}}</h1>

  <p>
    <a href="https://crates.io/crates/{{project-name}}">
      <img src="https://img.shields.io/crates/v/{{repo-name}}?label=crates" alt="Crate">
    </a>{% if codecov %}
    <a href="https://codecov.io/gh/{{github-name}}/{{repo-name}}">
      <img src="https://codecov.io/gh/{{github-name}}/{{repo-name}}/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>{% endif %}{% if github_actions %}
    <a href="https://github.com/{{github-name}}/{{repo-name}}/actions?query=">
      <img src="https://github.com/{{github-name}}/{{repo-name}}/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>{% endif %}{% if license == "Apache" %}
    <a href="https://github.com/{{github-name}}/{{repo-name}}/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>{% elsif license == "MIT" %}
    <a href="https://github.com/{{github-name}}/{{repo-name}}/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
    </a>{% else %}
    <a href="https://github.com/{{github-name}}/{{repo-name}}/blob/main/LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://github.com/{{github-name}}/{{repo-name}}/blob/main/LICENSE-MIT">
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

- {% if crate_type == "lib" %}[Usage](#usage){% else %}[Installation](#installation){% endif %}
- [Testing the Project](#testing-the-project){% if bench %}
- [Benchmarking the Project](#benchmarking-the-project){% endif %}{% if docker %}
- [Running {{project-name}} on Docker](#running-{{project-name}}-on-docker){% endif %}{% if contributing %}
- [Contributing](#contributing){% endif %}
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)
{% if crate_type == "lib" %}
## Usage

Add the following to the `[dependencies]` section of your `Cargo.toml` file:

```toml
{{project-name}} = "0.1.0"
```
{% else %}
## Installation

### Using `cargo`

```console
cargo install {{project-name}}
```
{% endif %}
## Testing the Project

- Run tests

  ```console
  cargo test
  ```
{% if bench %}
## Benchmarking the Project

For benchmarking and measuring performance, this project leverages
[criterion][criterion] and a `test_utils` feature flag
for integrating [proptest][proptest] within the the suite for working with
[strategies][strategies] and sampling from randomly generated values.

- Run benchmarks

  ```console
  cargo bench --features test_utils
  ```
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

- Build a multi-plaform Docker image via [buildx][buildx]:

  ```console
  docker buildx build --platform=linux/amd64,linux/arm64 -t {{project-name}} --progress=plain .
  ```

- Run a Docker image (depending on your platform):

  ```console
  docker run --platform=linux/amd64 -t {{project-name}}
  ```
{% endif %}{% if contributing %}
## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
{% if contributing %}We have a [contributing guide](./CONTRIBUTING.md) to help you get involved.{% endif %} We
also adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).
{% endif %}{% if nix %}
### Nix

This repository contains a [Nix flake][nix-flake] that initiates both the Rust
toolchain set in [rust-toolchain.toml](./rust-toolchain.toml) and a
[pre-commit hook](#pre-commit-hook). It also installs helpful cargo binaries for
development. Please install [nix][nix] and [direnv][direnv] to get started.

Run `nix develop` or `direnv allow` to load the `devShell` flake output,
according to your preference.

### Formatting

For formatting Rust in particular, we automatically format on `nightly`, as it
uses specific nightly features we recommend by default.
{% else  %}
### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.
{% endif %}
### Pre-commit Hook

This project recommends using [pre-commit][pre-commit] for running pre-commit
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
{% elsif license == "dual" %}
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
{% if license == "Apache" %}
[apache]: https://www.apache.org/licenses/LICENSE-2.0{% endif %}{% if license == "dual" %}
[apache]: https://www.apache.org/licenses/LICENSE-2.0{% endif %}{% if docker %}
[buildx]: https://github.com/docker/buildx{% endif %}
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/{% if bench %}
[criterion]: https://github.com/bheisler/criterion.rs{% endif %}{% if docker %}
[docker-engine]: https://docs.docker.com/engine/{% endif %}{% if nix %}
[direnv]:https://direnv.net/{% endif %}
[irust]: https://github.com/sigmaSd/IRust{% if license == "MIT" %}
[mit]: http://opensource.org/licenses/MIT{% endif %}{% if license == "dual" %}
[mit]: http://opensource.org/licenses/MIT{% endif %}{% if nix %}
[nix]:https://nixos.org/download.html
[nix-flake]: https://nixos.wiki/wiki/Flakes{% endif %}
[pre-commit]: https://pre-commit.com/{% if bench %}
[proptest]: https://github.com/proptest-rs/proptest
[strategies]: https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html{% endif %}
