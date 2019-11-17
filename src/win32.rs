use crate::Selection;

use std::{ffi::OsString, os::windows::ffi::OsStrExt, ptr};
use winapi::um::winuser::{
    MessageBoxW, IDABORT, IDCANCEL, IDCONTINUE, IDIGNORE, IDNO, IDOK, IDRETRY, IDTRYAGAIN, IDYES,
};

fn to_wtf16(s: &str) -> Box<[u16]> {
    OsString::from(s)
        .encode_wide() // encode as windows wchar_t ("potentially ill-formed utf16" / wtf16)
        .chain(Some(0_u16)) // append null terminator
        .collect::<Vec<_>>() // collect to vector
        .into_boxed_slice() // but we really don't need a vector
}

pub fn show<S1, S2>(message: S1, caption: S2) -> Result<Selection, ()>
where
    S1: AsRef<str>,
    S2: AsRef<str>, // they aren't necessarily the same type
{
    let message = to_wtf16(message.as_ref());
    let caption = to_wtf16(caption.as_ref());

    // call winapi function
    let result = unsafe {
        MessageBoxW(
            ptr::null_mut(),  // hWnd: HWND (NULL handle = self)
            message.as_ptr(), // lpText: LPCWSTR
            caption.as_ptr(), // lpCaption: LPCWSTR
            0,                // uType: UINT
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
