macro_rules! clear_tty {
    () => {
        if std::io::stdout().is_tty() {
            execute!(std::io::stdout(), Clear(ClearType::All))?;
        }
    };
}
