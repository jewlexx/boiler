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
