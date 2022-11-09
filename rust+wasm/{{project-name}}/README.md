<div align="center">
  <a href="https://github.com/{{github-name}}/{{project-name}}" target="_blank">
    <img src="https://raw.githubusercontent.com/{{github-name}}/{{project-name}}/main/assets/a_logo.png" alt="{{project-name}} Logo" width="100"></img>
  </a>

  <h1 align="center">{{project-name}}</h1>

  <p>
    <a href="https://crates.io/crates/{{project-name}}">
      <img src="https://img.shields.io/crates/v/{{project-name}}?label=crates" alt="Crate">
    </a>{% if codecov %}
    <a href="https://codecov.io/gh/{{github-name}}/{{project-name}}">
      <img src="https://codecov.io/gh/{{github-name}}/{{project-name}}/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>{% endif %}{% if github_actions %}
    <a href="https://github.com/{{github-name}}/{{project-name}}/actions?query=">
      <img src="https://github.com/{{github-name}}/{{project-name}}/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a> {% endif %}{% if license == "Apache" %}
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

Description.

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
