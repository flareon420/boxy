#[cfg(target_os = "windows")]
mod win32;
#[cfg(target_os = "windows")]
pub use win32::show;

#[derive(Copy, Clone, Debug)]
pub enum Buttons {
    AbortRetryIgnore, // TODO: Maybe don't support this.
    OK,
    OKCancel,
    RetryCancel,
    YesNo,
    YesNoCancel,
}

#[derive(Copy, Clone, Debug)]
pub enum Icon {
    Info,
    Warning,
    Error,
    Prompt,

    Nothing,
}

#[derive(Copy, Clone, Debug)]
pub enum Selection {
    Abort,
    Cancel,
    Continue,
    Ignore,
    No,
    OK,
    Retry,
    TryAgain,
    Yes,
}

#[macro_export]
macro_rules! create {
    ($message: expr) => {{
        create!($message, "boxy")
    }};

    ($message: expr, $caption: expr) => {{
        show($message, $caption)
    }};
}

#[test]
fn test() {
    let _result = create!("hello, world", "boxy").unwrap();
}
