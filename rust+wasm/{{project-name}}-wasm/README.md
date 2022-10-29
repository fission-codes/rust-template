<div align="center">
  <a href="https://github.com/{{github-name}}/{{project-name}}" target="_blank">
    <img src="https://raw.githubusercontent.com/{{github-name}}/{{project-name}}/main/assets/a_logo.png" alt="{{project-name}} Logo" width="100"></img>
  </a>

  <h1 align="center">{{project-name}}-wasm</h1>

  <p>
    <a href="https://crates.io/crates/{{project-name}}-wasm">
      <img src="https://img.shields.io/crates/v/{{project-name}}-wasm?label=crates" alt="Crate">
    </a>
    <a href="https://npmjs.com/package/{{project-name}}">
      <img src="https://img.shields.io/npm/v/{{project-name}}" alt="Npm">
    </a>{% if codecov %}
    <a href="https://codecov.io/gh/{{github-name}}/{{project-name}}">
      <img src="https://codecov.io/gh/{{github-name}}/{{project-name}}/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>{% endif %} {% if github-actions %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/actions?query=">
      <img src="https://github.com/{{github-name}}/{{project-name}}/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>{% endif %} {% if license == "Apache" %}
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
    </a>{% if discord != "" %}
    <a href="{{discord}}">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>{% endif %}
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

Description.

## Outline

- [Set-up](#set-up)
- [Build for Javascript](#build-for-javascript)
- [Testing the Project](#testing-the-project)
- [Publishing a Package](#publishing-a-package)
- [License](#license)

## Set-up
{% if nix == false %}
- Install [`wasm-pack`][wasm-pack]

  ```console
  cargo install wasm-pack
  ```
{% endif %}
We'll use [`wasm-pack`][wasm-pack] for building, testing, and publishing
our wasm project.

### Build for Javascript

The `wasm-pack build` command will compile the code in this directory into
wasm and generate a `pkg` folder by default, containing the wasm binary, a
Javascript-wrapper file, the {{project-name}}-wasm README (and version), and a
`package.json` file.

- Targetting node:

  ```console
  wasm-pack build --target nodejs
  ```

- Targetting browswers:

  ```console
  wasm-pack build --target web
  ```

- Targetting bundlers like [webpack][webpack]:

  ```console
  wasm-pack build --target bundler
  ```

## Testing the Project

For running tests in the current directly, use this command:

```console
wasm-pack test
```

- Run tests expected to execute in [Node.js][node-js]:

```console
wasm-pack test --node
```

- Run browser tests in a headless browwer:

```console
wasm-pack test --headless --firefox --chrome --safari
```

*Note*: Make sure you have the appropriate browser installed when running
locally.

## Publishing a Package

Once you've [built the package](#build-for-javascript), which lives under
`pkg` by default (or a sub-directory of your choosing), you can pack and
publish it via (given credentials):

```console
wasm-pack publish
```

## License
{% if license == "Apache" %}
This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].
{% elsif license == "MIT" %}
This project is licensed under the [MIT License](./LICENSE),
or [http://opensource.org/licenses/MIT][mit].
{% else %}
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
{% endif %}

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[mit]: http://opensource.org/licenses/MIT
[node-js]: https://nodejs.dev/en/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[webpack]: https://webpack.js.org/
