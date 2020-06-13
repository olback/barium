use std::{
    process::Command,
    path::PathBuf,
    fs
};
use regex::Regex;

#[derive(Debug)]
struct ResourceFile {
    alias: Option<String>,
    preprocess: Option<String>,
    value: String
}

// <file alias="ui/about-friend-dialog" preprocess="xml-stripblanks">out/ui/about-friend-dialog.glade</file>
impl ToString for ResourceFile {

    fn to_string(&self) -> String {

        let mut attrs = Vec::<String>::new();

        if let Some(a) = self.alias.clone() {
            attrs.push(format!("alias=\"{}\"", a));
        }

        if let Some(p) = self.preprocess.clone() {
            attrs.push(format!("preprocess=\"{}\"", p));
        }

        if attrs.is_empty() {
            format!("<file>{}</file>", self.value)
        } else {
            format!("<file {}>{}</file>", attrs.join(" "), self.value)
        }

    }

}

pub fn generate_xml(glade_files: &Vec<PathBuf>) {

    const INPUT: &str = "assets/net.olback.Barium.gresource.xml";
    const TARGET: &str = "out/net.olback.Barium.gresource.xml";

    let mut glade_resources = Vec::<ResourceFile>::new();

    for gf in glade_files {

        let gfs = gf.to_string_lossy();
        let ext = gf.extension().unwrap().to_string_lossy();
        let rf = ResourceFile {
            alias: Some(gfs.replace(&format!(".{}", ext), "").replace("out/", "")),
            preprocess: Some("xml-stripblanks".into()),
            value: gfs.to_string()
        };

        glade_resources.push(rf);

    }

    let out = glade_resources.iter().map(|r| r.to_string()).collect::<Vec<_>>().join("\n        ");
    let re_ui = Regex::new(r"(?P<r>\{ui-files\})").unwrap();
    let glade_xml_data = fs::read_to_string(INPUT).unwrap();
    let after = re_ui.replace_all(&glade_xml_data, out.as_str());

    fs::write(TARGET, after.to_owned().as_bytes()).unwrap();

}

pub fn generate_resources() {

    const COMMAND: &str = "glib-compile-resources";
    const INPUT: &str = "out/net.olback.Barium.gresource.xml";
    const TARGET: &str = "barium.gresource";

    let exists = Command::new("which").arg(COMMAND).output().unwrap();
    if !exists.status.success() {
        panic!(format!("Command '{}' not found", COMMAND));
    }

    let resources = Command::new(COMMAND)
    .args(&[INPUT, &format!("--target=out/{}", TARGET)])
    .output()
    .unwrap();

    if !resources.status.success() {
        panic!(format!("Failed to generate resources: {}", String::from_utf8_lossy(&resources.stderr)))
    }

}
