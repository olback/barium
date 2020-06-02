mod out;
mod resources;
mod glade;
mod version;
mod windows;

fn main() {

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=assets/*");

    let version = version::get_version();
    let glade_data = glade::GladeData {
        version: &version,
        authors: env!("CARGO_PKG_AUTHORS")
    };

    out::output_dir();
    version::write_version(&version);

    let glade_files = glade::process(&glade_data);
    resources::generate_xml(&glade_files);
    resources::generate_resources();

    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        windows::generate_manifest();
        windows::generate_rc(&version);
        windows::compile_rc();
    }

}
