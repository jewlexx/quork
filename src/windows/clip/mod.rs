extern "C" {
    fn Clipboard() -> cty::c_int;
}

pub enum ClipboardError {
    ClipboardInUse,
}

pub fn open_clipboard() -> Result<cty::c_int, ClipboardError> {
    let res = unsafe { Clipboard() };

    if res == -1 {
        Err(ClipboardError::ClipboardInUse)
    } else {
        Ok(res)
    }
}
