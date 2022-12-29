#[cfg(windows)]
fn main() -> std::io::Result<()> {
    let res = winres::WindowsResource::new();
    res.compile()
}

#[cfg(not(windows))]
fn main() -> std::io::Result<()> {
    Ok(())
}
