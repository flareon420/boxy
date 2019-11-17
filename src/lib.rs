#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub use win32::show;

#[derive(Copy, Clone, Debug)]
pub enum Buttons {
    OK,
    OKCancel,
    YesNo,
    Quit,
}

#[derive(Copy, Clone, Debug)]
pub enum Icon {
    Info,
    Warning,
    Error,
    Prompt,

    Nothing,
}

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
