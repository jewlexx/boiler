use dialoguer::Select;
use esize::ENum;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::consts::THEME;

#[derive(Debug, Copy, Clone, ENum, Display, EnumIter, FromPrimitive)]
pub enum BoilerTypes {
    File,
    Project,
}

impl BoilerTypes {
    pub fn get_type() -> Result<Self, std::io::Error> {
        let opts: Vec<String> = BoilerTypes::iter().map(|x| x.to_string()).collect();

        Select::with_theme(&*THEME)
            .with_prompt("What type of boilerplate would you like to generate?")
            .default(0)
            .items(&opts)
            .interact()
            .map(|x| FromPrimitive::from_usize(x).expect("invalid option"))
    }
}
