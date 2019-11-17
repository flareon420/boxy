#[cfg(target_os = "windows")] mod win32;
#[cfg(target_os = "windows")] pub use win32::show;

#[test]
fn test() {
    show("Hello, world!", "boxy");
}
