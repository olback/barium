use {
    std::panic::PanicInfo,
    gtk::{Align, Dialog, DialogFlags, Label, ResponseType, Window, prelude::*}
};

pub fn panic_handler(panic_info: &PanicInfo) {

    println!("Panic info: {:#?}", panic_info);

    drop(gtk::init());

    let dialog = Dialog::new_with_buttons(
        Some("Barium crashed"),
        None::<&Window>,
        DialogFlags::empty(),
        &[("Close", ResponseType::None)]
    );
    dialog.set_resizable(false);
    dialog.set_icon_name(Some("net.olback.Barium"));
    dialog.set_keep_above(true);
    dialog.grab_focus();

    let content_area = dialog.get_content_area();
    let error_label = Label::new(None);
    let panic_label = Label::new(Some(format!("{:#?}", panic_info).as_str()));
    error_label.set_markup("<b>Barium encountered an error and has exited</b>");
    panic_label.set_hexpand(true);
    panic_label.set_halign(Align::Start);
    panic_label.set_selectable(true);
    content_area.add(&error_label);
    content_area.add(&panic_label);
    content_area.set_margin_top(18);
    content_area.set_margin_bottom(18);
    content_area.set_margin_start(18);
    content_area.set_margin_end(18);
    content_area.set_spacing(18);
    content_area.show_all();

    drop(dialog.run());

    std::process::exit(1);

}
