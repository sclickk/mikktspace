#[derive(Clone)]
pub struct Group {
    pub range: std::ops::Range<usize>,
    pub vertex_representitive: usize,
    pub orient_preservering: bool,
}

impl Group {
    pub(crate) const fn zero() -> Self {
        Self {
            range: 0..0,
            vertex_representitive: 0,
            orient_preservering: false,
        }
    }

    pub(crate) fn iter<'a>(&self, buffer: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
        buffer[self.range.clone()].iter().copied()
    }
}

#[derive(Clone)]
pub struct SubGroup {
    pub members: Vec<usize>,
}

impl SubGroup {
    pub(crate) const fn zero() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}