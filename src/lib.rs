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
        create!($message, $caption, Icon::Nothing)
    }};

    ($message: expr, $caption: expr, $icon: expr) => {{
        create!($message, $caption, $icon, Buttons::OK)
    }};

    ($message: expr, $caption: expr, $icon: expr, $buttons: expr) => {{
        show($message, $caption, $icon, $buttons)
    }};
}

#[test]
fn test() {
    let _result = create!(
        "hello, world",
        "boxy",
        Icon::Error,
        Buttons::AbortRetryIgnore
    )
    .unwrap();
}
