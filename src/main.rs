use std::io;

mod boiler;

#[macro_use]
extern crate lazy_static;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
    tty::IsTty,
};
use dialoguer::{theme::ColorfulTheme, Select};

use crate::boiler::BoilerOpts;

lazy_static! {
    static ref THEME: ColorfulTheme = ColorfulTheme::default();
}

fn main() -> anyhow::Result<()> {
    if !io::stdout().is_tty() {
        return Err(anyhow::anyhow!("is not tty"));
    }

    let boiler_opts = ["File", "Project"];

    Select::with_theme(&*THEME)
        .with_prompt("What type of boilerplate would you like to generate?")
        .default(0)
        .items(&boiler_opts)
        .interact()
        .unwrap();

    Ok(())
}
