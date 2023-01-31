if variable::is_set("axum") && variable::get("axum") {
    file::rename("src.axum", "src");
    file::rename("tests/integration_test.axum.rs", "tests/integration_test.rs");
    file::rename("Cargo.axum.toml", "Cargo.toml");
    file::rename("README.axum.md", "README.md");
}
