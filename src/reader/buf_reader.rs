use no_std_io::io::{self, Read};

pub(crate) const DEFAULT_BUF_SIZE: usize = 8192;

pub(crate) struct BufReader<R, const S: usize = DEFAULT_BUF_SIZE> {
    inner: R,
    buf: [u8; S],
    pos: usize,
    cap: usize,
}

impl<R: Read, const S: usize> BufReader<R, S> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: [0; S],
            pos: 0,
            cap: 0,
        }
    }

    pub fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos >= self.cap {
            self.cap = self.inner.read(&mut self.buf)?;
            self.pos = 0;
        }
        Ok(&self.buf[self.pos..self.cap])
    }

    pub fn consume(&mut self, amt: usize) {
        self.pos = core::cmp::min(self.pos + amt, self.cap);
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}
