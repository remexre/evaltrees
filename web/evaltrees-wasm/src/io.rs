use std::io::{Error, ErrorKind, Write};
use std::str::from_utf8;

use write;

/// Provides an I/O facade to the xterm.js window.
pub struct IoFacade {
    input_buffers: Vec<String>,
    output_buffer: String,
}

impl IoFacade {
    /// Creates the IoFacade.
    pub fn new() -> IoFacade {
        IoFacade {
            input_buffers: Vec::new(),
            output_buffer: String::new(),
        }
    }

    fn maybe_flush(&mut self) {
        while let Some(idx) = self.output_buffer.find('\n') {
            write(&self.output_buffer[..idx]);
            self.output_buffer.drain(..idx);
        }
    }

    /// Puts a string into the read buffer.
    pub fn put(&mut self, s: &str) {
        let mut buf = self.input_buffers.pop().unwrap_or_else(String::new);
        for c in s.chars() {
            if c == '\n' || c == '\r' {
                self.input_buffers.push(buf);
                buf = String::new();
            } else {
                buf.push(c);
            }
        }
        if buf != "" {
            self.input_buffers.push(buf);
        }
    }
}

impl Write for IoFacade {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match from_utf8(buf) {
            Ok(s) => {
                self.output_buffer += s;
                self.maybe_flush();
                Ok(buf.len())
            }
            Err(e) => Err(Error::new(ErrorKind::InvalidData, Box::new(e))),
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        write(&self.output_buffer);
        self.output_buffer = String::new();
        Ok(())
    }
}
