use std::{env, process::ExitCode};

fn main() -> ExitCode {
    println!("cargo:rerun-if-changed=mavlink_messages/");

    let out_dir = env::var("OUT_DIR").unwrap();

    match mavlink_bindgen::generate("mavlink_messages/", out_dir) {
        Ok(r) => {
            mavlink_bindgen::format_generated_code(&r);
            // No need for `mavlink_bindgen::emit_cargo_build_messages` because of above `println!`
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
