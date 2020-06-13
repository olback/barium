use std::{
    env,
    fs,
    path::Path,
    process::Command,
};
use last_git_commit::LastGitCommit;

pub fn generate_rc(version: &String) {

    const RC_IN: &str = "windows/barium-server.rc.in";
    const RC_OUT: &str = "windows/barium-server.rc";

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
    .args(&["windows/barium-server.rc", &format!("{}/program.o", out_dir)])
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

pub fn get_version() -> String {

    let lgc = LastGitCommit::new().set_path("../").build().unwrap();
    let cargo_version = env!("CARGO_PKG_VERSION");

    format!("{}-{}-{}", cargo_version, lgc.branch(), lgc.id().short())

}

fn main() {

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=*");
    println!("cargo:rerun-if-changed=*/**");

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        generate_rc(&get_version());
        compile_rc();
    }

}
