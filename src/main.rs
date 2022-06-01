// Old code that I will use in future prolly
// mod boiler;
// mod consts;
// mod errors;

// #[macro_use]
// mod macros;

// #[macro_use]
// extern crate lazy_static;

// use consts::*;
// use crossterm::{
//     execute,
//     style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
//     terminal::{Clear, ClearType, SetTitle},
//     tty::IsTty,
// };
// use dialoguer::Select;

// use boiler::BoilerTypes;
// use strum::IntoEnumIterator;

// fn main() -> anyhow::Result<()> {
//     if !IS_TTY.to_owned() {
//         return Err(anyhow::anyhow!("is not tty"));
//     }

//     clear_tty!();

//     let boiler_type = BoilerTypes::get_type()?;

//     Ok(())
// }

use std::{env, fs};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let path = env::args_os()
        .next()
        .context("Please provide a path to the desired project destination")?;
    let cwd = env::current_dir().context("Failed to parse current dir")?;

    let proj_path = cwd.join(path);

    fs::create_dir_all(&proj_path)?;

    fs::File::create(proj_path.join("__main__.py"))?;

    println!("Project created at: {}", proj_path.display());

    Ok(())
}
