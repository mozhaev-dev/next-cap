pub mod linux;
pub mod macos;
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux as implementation;

#[cfg(target_os = "windows")]
pub use windows as implementation;

#[cfg(target_os = "macos")]
pub use macos as implementation;
