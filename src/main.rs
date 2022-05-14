use std::io;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
    tty::IsTty,
};

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    if !stdout.is_tty() {
        panic!("is not tty")
    }

    execute!(
        stdout,
        Clear(ClearType::All),
        SetTitle("Boiler"),
        ResetColor
    )?;

    Ok(())
}
