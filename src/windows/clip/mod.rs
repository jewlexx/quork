use clipboard_win::{formats, Clipboard as WinClip, Getter, Setter, SysResult, SystemError};
use error_code::{ErrorCode, SystemCategory};

#[derive(Debug, Clone, strum::Display, thiserror::Error)]
pub enum ClipboardError {
    ClipboardInUse,
    EncodingError(&'static str),
    SysError(SystemCategory),
}

impl<T> From<ErrorCode<T>> for ClipboardError {
    fn from(r: ErrorCode<SystemCategory>) -> Self {}
}

pub struct Clipboard {
    clipboard: WinClip,
}

impl Clipboard {
    pub fn open(attempts: Option<usize>) -> Result<Self, ClipboardError> {
        let clip = WinClip::new_attempts(attempts.unwrap_or(1));

        if let Ok(clipboard) = clip {
            Ok(Self { clipboard })
        } else {
            Err(ClipboardError::ClipboardInUse)
        }
    }

    pub fn write_utf8<S: std::fmt::Display>(&self, text: S) -> Result<(), ClipboardError> {
        let string = String::from_utf8(text.to_string().into_bytes());

        if let Ok(utf8_string) = string {
            let e = formats::Unicode.write_clipboard(&utf8_string).err();
            Ok(())
        } else {
            Err(ClipboardError::EncodingError("Should be UTF-8"))
        }
    }
}
