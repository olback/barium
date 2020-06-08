use {
    crate::error::BariumResult,
    gtk_resources::UIResource
};

#[derive(Clone, UIResource)]
#[resource = "/net/olback/barium/ui/main-window"]
pub struct Ui {
    main_window: gtk::ApplicationWindow
}

impl Ui {

    pub fn build() -> BariumResult<Self> {

        Ok(Self::load()?)

    }

}
