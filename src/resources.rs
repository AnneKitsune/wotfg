use crate::*;

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Auth {
    pub id: String,
}

#[derive(Eq, PartialEq, Default, Debug)]
pub struct QuickItemSelect {
    pub selected: Option<u32>,
}

#[derive(Eq, PartialEq, Default, Debug)]
pub struct SelectedCraft {
    pub selected: Option<u32>,
}
