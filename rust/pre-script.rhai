let license = variable::get("license").to_upper();

while switch license {
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
  _ => true,
} {
  license = variable::prompt("What license to use?", "Apache", [
    "Apache",
    "MIT",
    "dual"
  ]);
}
variable::set("license", license);
