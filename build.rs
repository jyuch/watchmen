#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let mut res = winres::WindowsResource::new();
    res.set_icon("media/icon.ico");
    res.compile()
}

#[cfg(not(windows))]
fn main() -> std::io::Result<()> {
    Ok(())
}
