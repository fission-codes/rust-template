let license = variable::get("license");

while switch license.to_upper() {
  "APACHE" => {
    file::delete("LICENSE-MIT");
    file::delete("{{project-name}}/LICENSE-MIT");
    file::delete("{{project-name}}-wasm/LICENSE-MIT");
    file::rename("LICENSE-APACHE", "LICENSE");
    file::rename("{{project-name}}/LICENSE-APACHE", "{{project-name}}/LICENSE");
    file::rename("{{project-name}}-wasm/LICENSE-APACHE", "{{project-name}}/LICENSE");
    false
  }
  "MIT" => {
    file::delete("LICENSE-APACHE");
    file::delete("{{project-name}}/LICENSE-APACHE");
    file::delete("{{project-name}}-wasm/LICENSE-APACHE");
    file::rename("LICENSE-MIT", "LICENSE");
    file::rename("{{project-name}}/LICENSE-MIT", "{{project-name}}/LICENSE");
    file::rename("{{project-name}}-wasm/LICENSE-MIT", "{{project-name}}-wasm/LICENSE");
    false
  }
  "DUAL" => false,
  _ => true,
} {
  license = variable::prompt("What license to use?", "dual", [
    "Apache",
    "MIT",
    "dual"
  ]);
}
variable::set("license", license);

if variable::is_set("docker") && variable::get("docker") {
  let dockerbuild = variable::get("dockerbuild");
  while switch dockerbuild.to_upper() {
    "GLIBC" => {
      file::delete("docker/Dockerfile.musl");
      file::rename("docker/Dockerfile.glibc", "docker/Dockerfile");
      false
  }
    "MUSL" => {
      file::delete("docker/Dockerfile.glibc");
      file::rename("docker/Dockerfile.musl", "docker/Dockerfile");
      false
  }
    _ => true,
} {
    dockerbuild = variable::prompt("For docker, do you want a glibc or musl build?", "musl", [
      "musl",
      "glibc"
    ]);
  }
  variable::set("dockerbuild", dockerbuild);
}

// Set for Copyright Info (Apache)
variable::set("year", "2022");
let name = variable::get("github-name").to_title_case();
variable::set("copyright-owner", name);
