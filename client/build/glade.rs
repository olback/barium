use regex::Regex;
use std::fs;

pub fn process(version: &String) {

    const GLADE_IN_PATH: &str = "assets/barium.glade";
    const GLADE_OUT_PATH: &str = "out/barium.glade";

    // Fix resource paths
    let glade_xml_data = fs::read_to_string(GLADE_IN_PATH).unwrap();
    let re = Regex::new(r"(?P<r>resource:/)(?P<p>[a-z])").unwrap();
    let after = re.replace_all(&glade_xml_data, "$r//$p");

    // Fill in versions
    let re = Regex::new(r"(?P<r>\{version\})").unwrap();
    let after = re.replace_all(&after, version.as_str());

    fs::write(GLADE_OUT_PATH, after.to_owned().as_bytes()).unwrap();

}
