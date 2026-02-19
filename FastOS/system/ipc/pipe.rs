// ============================================================
// FastOS — Pipe IPC
// ============================================================
// Unidirectional byte stream between processes.
// ============================================================

/// Pipe buffer size
pub const PIPE_BUFFER_SIZE: usize = 4096;

/// Pipe structure
pub struct Pipe {
    buffer: [u8; PIPE_BUFFER_SIZE],
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl Pipe {
    pub const fn new() -> Self {
        Pipe {
            buffer: [0; PIPE_BUFFER_SIZE],
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            if self.count >= PIPE_BUFFER_SIZE { break; }
            self.buffer[self.write_pos] = byte;
            self.write_pos = (self.write_pos + 1) % PIPE_BUFFER_SIZE;
            self.count += 1;
            written += 1;
        }
        written
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        for byte in buf.iter_mut() {
            if self.count == 0 { break; }
            *byte = self.buffer[self.read_pos];
            self.read_pos = (self.read_pos + 1) % PIPE_BUFFER_SIZE;
            self.count -= 1;
            read += 1;
        }
        read
    }

    pub fn available(&self) -> usize { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
    pub fn is_full(&self) -> bool { self.count >= PIPE_BUFFER_SIZE }
}
