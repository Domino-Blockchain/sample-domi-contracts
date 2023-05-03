pub mod write_to {
    pub struct WriteTo<'a> {
        buffer: &'a mut [u8],
        // on write error (i.e. not enough space in buffer) this grows beyond
        // `buffer.len()`.
        used: usize,
    }
    
    impl<'a> WriteTo<'a> {
        pub fn new(buffer: &'a mut [u8]) -> Self {
            WriteTo { buffer, used: 0 }
        }
    
        pub fn as_str(self) -> Option<&'a str> {
            if self.used <= self.buffer.len() {
                // only successful concats of str - must be a valid str.
                use core::str::from_utf8_unchecked;
                Some(unsafe { from_utf8_unchecked(&self.buffer[..self.used]) })
            } else {
                None
            }
        }
    }
    
    impl<'a> core::fmt::Write for WriteTo<'a> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            if self.used > self.buffer.len() {
                return Err(core::fmt::Error);
            }
            let remaining_buf = &mut self.buffer[self.used..];
            let raw_s = s.as_bytes();
            let write_num = core::cmp::min(raw_s.len(), remaining_buf.len());
            remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
            self.used += raw_s.len();
            if write_num < raw_s.len() {
                Err(core::fmt::Error)
            } else {
                Ok(())
            }
        }
    }
    
    pub fn show<'a>(buffer: &'a mut [u8], args: core::fmt::Arguments) -> Result<&'a str, core::fmt::Error> {
        let mut w = WriteTo::new(buffer);
        core::fmt::write(&mut w, args)?;
        w.as_str().ok_or(core::fmt::Error)
    }
}