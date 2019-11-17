use crate::{Buttons, Icon, Selection};

use std::{ffi::OsString, os::windows::ffi::OsStrExt, ptr};
use winapi::um::winuser::{
    MessageBoxW, IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES,
    MB_ABORTRETRYIGNORE, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_ICONWARNING, MB_OK,
    MB_OKCANCEL, MB_RETRYCANCEL, MB_YESNO, MB_YESNOCANCEL,
};

/// Helper function for converting &str to Windows's expected wchar_t + NULL format (LPCWSTR).
fn to_wtf16(s: &str) -> Box<[u16]> {
    OsString::from(s)
        .encode_wide() // encode as windows wchar_t ("potentially ill-formed utf16" / wtf16)
        .chain(Some(0_u16)) // append null terminator
        .collect::<Vec<_>>() // collect to vector
        .into_boxed_slice() // but we really don't need a vector
}

pub fn show<S1, S2>(message: S1, caption: S2, icon: Icon, buttons: Buttons) -> Result<Selection, ()>
where
    S1: AsRef<str>,
    S2: AsRef<str>, // they aren't necessarily the same type
{
    let message = to_wtf16(message.as_ref());
    let caption = to_wtf16(caption.as_ref());

    let mut style = match icon {
        Icon::Info => MB_ICONINFORMATION,
        Icon::Warning => MB_ICONWARNING,
        Icon::Error => MB_ICONERROR,
        Icon::Prompt => MB_ICONQUESTION,
        Icon::Nothing => 0,
    };

    style |= match buttons {
        Buttons::AbortRetryIgnore => MB_ABORTRETRYIGNORE,
        Buttons::OK => MB_OK,
        Buttons::OKCancel => MB_OKCANCEL,
        Buttons::RetryCancel => MB_RETRYCANCEL,
        Buttons::YesNo => MB_YESNO,
        Buttons::YesNoCancel => MB_YESNOCANCEL,
        // TODO: Add more? Maybe.
    };

    // call winapi function
    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),  // hWnd: HWND (NULL handle = self)
            message.as_ptr(), // lpText: LPCWSTR
            caption.as_ptr(), // lpCaption: LPCWSTR
            style,            // uType: UINT
        )
    };

    Ok(match result {
        IDABORT => Selection::Abort,
        IDCANCEL => Selection::Cancel,
        IDCONTINUE => Selection::Continue,
        IDIGNORE => Selection::Ignore,
        IDNO => Selection::No,
        IDOK => Selection::OK,
        IDRETRY => Selection::Retry,
        IDTRYAGAIN => Selection::TryAgain,
        IDYES => Selection::Yes,

        0 => return Err(()), // TODO: GetLastError
        unk => panic!("unexpected winapi return value: {}", unk),
    })
}
