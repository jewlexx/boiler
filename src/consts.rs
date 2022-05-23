use crossterm::tty::IsTty;
use dialoguer::theme::ColorfulTheme;

lazy_static! {
    pub static ref THEME: ColorfulTheme = ColorfulTheme::default();
    pub static ref IS_TTY: bool = std::io::stdout().is_tty();
}
