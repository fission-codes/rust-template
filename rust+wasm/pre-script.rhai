let license = variable::get("license").to_upper();

while switch license {
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
  license = variable::prompt("What license to use?", "Apache", [
    "Apache",
    "MIT",
    "dual"
  ]);
}
variable::set("license", license);
