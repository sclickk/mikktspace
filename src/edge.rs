#[derive(Clone)]
pub struct Edge {
    pub i0: usize,
    pub i1: usize,
    pub f: usize,
}

impl Edge {
    #[inline]
    pub(crate) const fn zero() -> Self {
        Self { i0: 0, i1: 0, f: 0 }
    }

    #[inline]
    pub(crate) fn array(&self, channel: usize) -> usize {
        match channel {
            0 => self.i0,
            1 => self.i1,
            2 => self.f,
            _ => unreachable!("wtf?"),
        }
    }
}
