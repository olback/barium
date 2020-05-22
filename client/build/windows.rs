use std::{
    env,
    fs,
    path::Path,
    process::Command
};

pub fn generate_rc(version: &String) {

    const RC_IN: &str = "assets/windows/barium.rc";
    const RC_OUT: &str = "out/barium.rc";

    let rc_content = fs::read_to_string(RC_IN).unwrap();

    let new_rc_content = rc_content
        .replace("{description}", env!("CARGO_PKG_DESCRIPTION"))
        .replace("{version}", version)
        .replace("{version-major}", env!("CARGO_PKG_VERSION_MAJOR"))
        .replace("{version-minor}", env!("CARGO_PKG_VERSION_MINOR"))
        .replace("{version-patch}", env!("CARGO_PKG_VERSION_PATCH"));

    fs::write(RC_OUT, new_rc_content).unwrap();

}

pub fn compile_rc() {

    let out_dir = env::var("OUT_DIR").unwrap();
    match Command::new("x86_64-w64-mingw32-windres")
    .args(&["out/barium.rc", &format!("{}/program.o", out_dir)])
    .status() {
        Ok(s) => {
            if !s.success() {
                panic!("{}#{}: x86_64-w64-mingw32-windres failed", std::file!(), std::line!())
            }
        },
        Err(e) => {
            panic!("{}#{}: x86_64-w64-mingw32-windres failed {}", std::file!(), std::line!(), e)
        }
    };

    match Command::new("x86_64-w64-mingw32-gcc-ar")
    .args(&["crus", "libprogram.a", "program.o"])
    .current_dir(&Path::new(&out_dir))
    .status() {
        Ok(s) => {
            if !s.success() {
                panic!("{}#{}: x86_64-w64-mingw32-gcc-ar failed", std::file!(), std::line!())
            }
        },
        Err(e) => {
            panic!("{}#{}: x86_64-w64-mingw32-gcc-ar failed {}", std::file!(), std::line!(), e)
        }
    };

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=program");

}
