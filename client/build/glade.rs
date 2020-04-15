use regex::Regex;
use std::{io, fs::{self, DirEntry}, path::{Path, PathBuf}};

// Mostly stolen from the rust docs
// https://doc.rust-lang.org/std/fs/fn.read_dir.html

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry,)) -> io::Result<()> {

    if dir.is_dir() {

        for entry in fs::read_dir(dir)? {

            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }

        }

    }

    Ok(())

}

pub fn process(version: &String) {

    let path = PathBuf::from("assets/ui");
    let re_re = Regex::new(r"(?P<r>resource:/)(?P<p>[a-z])").unwrap();
    let re_ve = Regex::new(r"(?P<r>\{version\})").unwrap();

    visit_dirs(&path, &|entry| {

        println!("{:#?} {:#?}", path, entry);

        let in_path = entry.path();

        // Fix resource paths & version
        let glade_xml_data = fs::read_to_string(&in_path).unwrap();
        let after = re_re.replace_all(&glade_xml_data, "$r//$p");
        let after = re_ve.replace_all(&after, version.as_str());

        let out_path = PathBuf::from("out").join(in_path.strip_prefix("assets").unwrap());
        let mut out_path_dir = out_path.clone();
        out_path_dir.pop();

        if !out_path_dir.exists() {
            fs::create_dir_all(&out_path_dir).unwrap();
        }

        fs::write(out_path, after.to_owned().as_bytes()).unwrap();

    });

}
