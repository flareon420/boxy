#[cfg(target_os = "windows")] mod win32;
#[cfg(target_os = "windows")] pub use win32::show;

#[macro_export]
macro_rules! create {
    ($message: expr) => {{
        create!($message, "boxy")
    }};

    ($message: expr, $caption: expr) => {{
        let message: &str = $message.into();
        let caption: &str = $caption.into();
        show(message, caption)
    }};
}

#[test]
fn test() {
    create!("hello, world", "boxy");
}
