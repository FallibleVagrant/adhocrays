use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;
use std::env;

use std::io::Write;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    if fs::create_dir(Path::new(&dir).join("lib")).is_err_and(|e| e.kind() != ErrorKind::AlreadyExists) {
        panic!("Could not create lib/ directory.");
    }
    Command::new("make").args(&["PLATFORM=PLATFORM_DESKTOP", "-C", &format!("{}/raylib/src", dir)]).status().unwrap();
    Command::new("cp").args(&[&format!("{}/raylib/src/libraylib.a", dir), &format!("{}/lib/", dir)]).status().unwrap();
    Command::new("cp").args(&[&format!("{}/raylib/src/raylib.h", dir), &format!("{}/lib/", dir)]).status().unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("lib").display());
}
