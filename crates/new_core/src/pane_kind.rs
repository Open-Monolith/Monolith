use strum_macros::{EnumIter, Display};


#[derive(EnumIter, Display, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PaneKind {
    #[strum(to_string="Viewport")]
    Viewport,

    #[strum(to_string="Explorer")]
    Explorer,

    #[strum(to_string="Properties")]  
    Properties,

    #[strum(to_string="Console")]
    Console,

    #[strum(to_string="Scripting")]
    Scripting,

    #[strum(to_string="Assets")]
    Assets,

    #[strum(to_string="Branch")]
    Branch,
}