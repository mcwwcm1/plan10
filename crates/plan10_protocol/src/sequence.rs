use std::ops::BitAnd;

#[derive(Default)]
pub struct Sequence {
    pub pos: u64,
}

impl Sequence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find(&self, other: u16) -> u64 {
        let this = self.pos.bitand(0xFFFF) as u16;
        let inord = this - other;
        let outord = other.wrapping_sub(this);

        if inord > outord {
            self.pos - outord as u64
        } else {
            self.pos - inord as u64
        }
    }
}

impl Iterator for Sequence {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pos.checked_add(1)?.bitand(0xFFFF) as u16)
    }
}

impl From<u64> for Sequence {
    fn from(n: u64) -> Self {
        Self { pos: n }
    }
}

#[cfg(test)]
mod tests {
    use crate::sequence::Sequence;

    fn generate() {
        let mut sq = Sequence::new();
        for i in 0..1000 {
            assert_eq!(sq.next(), Some(i));
        }
    }

    fn in_order() {
        let sq = Sequence::new();
        for i in 0..1000 {
            assert_eq!(sq.find(i), i as u64);
        }
    }

    fn out_of_order() {
        let sq = Sequence::from(0x10000);
        assert_eq!(sq.find(0xFFFF), 1);
        assert_eq!(sq.find(0xFFF0), 0x10);
    }
}
