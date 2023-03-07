/// Extend `std::io::Write` with wincon styling
///
/// Generally, you will want to use [`Console`][crate::Console] instead
pub trait WinconStream {
    /// Change the foreground/background
    ///
    /// A common pitfall is to forget to flush writes to
    /// stdout before setting new text attributes.
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()>;

    /// Get the current foreground/background colors
    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)>;
}

#[cfg(windows)]
impl WinconStream for std::io::Stdout {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

#[cfg(windows)]
impl<'s> WinconStream for std::io::StdoutLock<'s> {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

#[cfg(windows)]
impl WinconStream for std::io::Stderr {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

#[cfg(windows)]
impl<'s> WinconStream for std::io::StderrLock<'s> {
    fn set_colors(
        &mut self,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        inner::set_colors(self, fg, bg)
    }

    fn get_colors(
        &self,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        inner::get_colors(self)
    }
}

#[cfg(windows)]
mod inner {
    use std::os::windows::io::{AsHandle, AsRawHandle};

    pub(super) fn set_colors<S: AsHandle>(
        stream: &mut S,
        fg: Option<anstyle::AnsiColor>,
        bg: Option<anstyle::AnsiColor>,
    ) -> std::io::Result<()> {
        let handle = stream.as_handle();
        let handle = handle.as_raw_handle();
        if let (Some(fg), Some(bg)) = (fg, bg) {
            let attributes = crate::windows::set_colors(fg, bg);
            crate::windows::set_console_text_attributes(handle, attributes)
        } else {
            Ok(())
        }
    }

    pub(super) fn get_colors<S: AsHandle>(
        stream: &S,
    ) -> std::io::Result<(Option<anstyle::AnsiColor>, Option<anstyle::AnsiColor>)> {
        let handle = stream.as_handle();
        let handle = handle.as_raw_handle();
        let info = crate::windows::get_screen_buffer_info(handle)?;
        let (fg, bg) = crate::windows::get_colors(&info);
        Ok((Some(fg), Some(bg)))
    }
}
