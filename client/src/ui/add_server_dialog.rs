use {
    crate::error::BariumResult,
    gtk_resources::UIResource,
    gtk::{ApplicationWindow, Dialog, Label, Stack, ResponseType, prelude::*}
};

#[derive(Debug, UIResource)]
#[resource="/net/olback/barium/ui/add-server-dialog"]
pub struct AddServerDialog {

}

impl AddServerDialog {

    pub fn build() -> BariumResult<Self> {

        let inner = Self::load()?;

        Ok(inner)

    }

}
