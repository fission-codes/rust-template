let license = variable::get("license");

while switch license.to_upper() {
  "APACHE" => {
    file::delete("LICENSE-MIT");
    file::rename("LICENSE-APACHE", "LICENSE");
    false
  }
  "MIT" => {
    file::delete("LICENSE-APACHE");
    file::rename("LICENSE-MIT", "LICENSE");
    false
  }
  "DUAL" => false,
  "USE EXISTING" => {
    file::delete("LICENSE-MIT");
    file::delete("LICENSE-APACHE");
    false
  },
  _ => true,
} {
  license = variable::prompt("What license to use?", "dual", [
    "Apache",
    "MIT",
    "dual",
    "use existing"
  ]);
}
variable::set("license", license);

if variable::is_set("docker") && variable::get("docker") {
  let dockerbuild = variable::get("dockerbuild");
  while switch dockerbuild.to_upper() {
    "GLIBC" => {
      file::delete("Dockerfile.musl");
      file::rename("Dockerfile.glibc", "Dockerfile");
      false
  }
    "MUSL" => {
      file::delete("Dockerfile.glibc");
      file::rename("Dockerfile.musl", "Dockerfile");
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

// Set for Copyright Info
variable::set("year", "2023");
let name = variable::get("github-name").to_title_case();
variable::set("copyright-owner", name);
