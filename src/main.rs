mod boiler;
mod consts;

#[macro_use]
mod macros;

#[macro_use]
extern crate lazy_static;

use consts::*;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
    tty::IsTty,
};
use dialoguer::Select;

use boiler::BoilerTypes;
use strum::IntoEnumIterator;

fn main() -> anyhow::Result<()> {
    if !IS_TTY.to_owned() {
        return Err(anyhow::anyhow!("is not tty"));
    }

    clear_tty!();

    Ok(())
}
