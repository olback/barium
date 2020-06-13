use {
    crate::error::BariumResult,
    gtk::Builder
};

#[derive(Debug, Clone)]
pub struct InitialSetup { }

impl InitialSetup {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        Ok(Self { })

    }

}


