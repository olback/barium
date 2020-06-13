use {
    gio,
    glib
};

// Load & regiser resources
pub fn load() {

    let glib_resource_bytes = glib::Bytes::from_static(include_bytes!("../out/barium.gresource"));
    let resources = gio::Resource::new_from_data(&glib_resource_bytes).expect("Failed to load resources");
    gio::resources_register(&resources);

}
