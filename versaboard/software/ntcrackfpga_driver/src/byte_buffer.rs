use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use critical_section;


#[derive(Clone, Copy, Debug)]
pub struct ByteBuffer<const MAX_SIZE: usize> {
    bytes: [u8; MAX_SIZE],
    byte_count: usize,
}
impl<const MAX_SIZE: usize> ByteBuffer<MAX_SIZE> {
    pub const fn new() -> Self {
        Self {
            bytes: [0u8; MAX_SIZE],
            byte_count: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.byte_count
    }

    pub const fn max_size(&self) -> usize {
        MAX_SIZE
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.len() == self.max_size() {
            false
        } else {
            self.bytes[self.byte_count] = byte;
            self.byte_count += 1;
            true
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len() == 0 {
            None
        } else {
            let ret = self.bytes[self.byte_count-1];
            self.byte_count -= 1;
            Some(ret)
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[0..self.len()]
    }

    #[inline]
    pub fn clear(&mut self) {
        self.byte_count = 0;
    }

    /// Returns a copy of this buffer while emptying the original.
    pub fn critical_take(&mut self) -> Self {
        // we can allocate this outside of the critical section
        let mut buf = [0u8; MAX_SIZE];
        let new_buf = critical_section::with(|_| {
            buf.copy_from_slice(&self.bytes);
            let nb = Self {
                bytes: buf,
                byte_count: self.byte_count,
            };
            self.byte_count = 0;
            nb
        });
        new_buf
    }
}
impl<const MAX_SIZE: usize> PartialEq for ByteBuffer<MAX_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes[0..self.byte_count] == other.bytes[0..other.byte_count]
    }
}
impl<const MAX_SIZE: usize> Eq for ByteBuffer<MAX_SIZE> {
}
impl<const MAX_SIZE: usize> PartialOrd for ByteBuffer<MAX_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bytes[0..self.byte_count].partial_cmp(&other.bytes[0..other.byte_count])
    }
}
impl<const MAX_SIZE: usize> Ord for ByteBuffer<MAX_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes[0..self.byte_count].cmp(&other.bytes[0..other.byte_count])
    }
}
impl<const MAX_SIZE: usize> Hash for ByteBuffer<MAX_SIZE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes[0..self.byte_count].hash(state);
    }
}
