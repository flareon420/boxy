use std::{ffi::OsString, os::windows::ffi::OsStrExt, ptr};
use winapi::um::winuser::MessageBoxW;

fn to_wtf16(s: &str) -> Box<[u16]> {
    OsString::from(s)
        .encode_wide() // encode as windows wchar_t ("potentially ill-formed utf16" / wtf16)
        .chain(Some(0_u16)) // append null terminator
        .collect::<Vec<_>>() // collect to vector
        .into_boxed_slice() // but we really don't need a vector
}

pub fn show(message: &str, caption: &str) {
    let message = to_wtf16(message);
    let caption = to_wtf16(caption);

    // call winapi function
    unsafe {
        MessageBoxW(
            ptr::null_mut(),  // hWnd: HWND (NULL handle = self)
            message.as_ptr(), // lpText: LPCWSTR
            caption.as_ptr(), // lpCaption: LPCWSTR
            0,                // uType: UINT
        );
    }
}
