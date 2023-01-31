print();
print();
print("\
    For CI/CD purposes, be aware there's some secrets you'll need to configure in Github, including:\n\
    `CODECOV_TOKEN` if you choose to use coverage via Codecov and have it set up;\n\
    `CARGO_REGISTRY_TOKEN` for publshing Rust packages to crates.io;\n\
    `NPM_TOKEN` for publishing a Wasm project to npm;\n\
    `DOCKERHUB_USERNAME` and `DOCKERHUB_TOKEN` for pushing containers to Docker Hub.");
print();
