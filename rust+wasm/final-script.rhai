if variable::is_set("axum") && variable::get("axum") {
    let project = variable::get("project-name");

    let cargo_from = project + "/Cargo.axum.toml";
    let cargo_to = project + "/Cargo.toml";

    let src_from = project + "/src.axum";
    let src_to = project + "/src";

    let test_from = project + "/tests/integration_test.axum.rs";
    let test_to = project + "/tests/integration_test.rs";

    file::rename(src_from, src_to);
    file::rename(test_from, test_to);
    file::rename(cargo_from, cargo_to);
    file::rename("README.axum.md", "README.md");
}
