use std::{io, process::Child};

#[cfg(target_os = "macos")]
pub fn open(url: &str) -> io::Result<Child> {
    std::process::Command::new("open").arg(url).spawn()
}

#[cfg(target_os = "linux")]
pub fn open(url: &str) -> io::Result<Child> {
    std::process::Command::new("xdg-open").arg(url).spawn()
}

#[cfg(target_os = "windows")]
pub fn open(url: &str) -> io::Result<Child> {
    std::process::Command::new("rundll32")
        .arg("url.dll,FileProtocolHandler")
        .arg(url)
        .spawn()
}
