use std::fs;
use last_git_commit::{
    LastGitCommit,
    Id
};

pub fn get_version() -> String {

    let commit = LastGitCommit::new(Some("../"), Some("master")).unwrap().id.short();
    let cargo_version = env!("CARGO_PKG_VERSION");

    format!("{}-{}", cargo_version, commit)

}

pub fn write_version(version: &String) {

    fs::write("out/version.txt", version).unwrap();

}
