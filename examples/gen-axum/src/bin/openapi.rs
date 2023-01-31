use std::{fs::File, io::prelude::*, path::PathBuf};
use utoipa::OpenApi;
use gen_axum::docs::ApiDoc;

fn main() {
    let json_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docs/specs/latest.json");
    let json_path_show = json_path.as_path().display().to_string();

    let mut file = match File::create(json_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error creating file: {err:?}");
            std::process::exit(1)
        }
    };

    let json = match ApiDoc::openapi().to_pretty_json() {
        Ok(mut json) => {
            json.push('\n');
            json
        }
        Err(err) => {
            eprintln!("error generating OpenAPI json: {err:?}");
            std::process::exit(1)
        }
    };

    match file.write_all(json.as_bytes()) {
        Ok(_) => println!("OpenAPI json written to path: {json_path_show}\n\n{json}"),
        Err(err) => {
            eprintln!("error writing to file. {err:?}");
            std::process::exit(1)
        }
    }
}
