use std::collections::HashSet;
use std::fmt::Debug;

pub struct BitSet {
    vec_bits: Vec<usize>,
    vec_bits_capacity: usize,
    set_indices_active_bits: HashSet<usize>,
}

impl BitSet {
    pub fn new(bits_count: usize) -> Self {
        let word_count = bits_count / 64 + (bits_count % 64 > 0) as usize;
        Self {
            vec_bits: {
                let mut vec = Vec::with_capacity(word_count);
                vec.resize(word_count, 0);
                vec
            },
            vec_bits_capacity: word_count * 8 * size_of::<usize>(),
            set_indices_active_bits: HashSet::new()
        }
    }
    fn check_index(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.vec_bits_capacity {
            return None;
        }
        Some((index / (8 * size_of::<usize>()), index % (8 * size_of::<usize>())))
    }
    /// Turns a specific bit ON using bitwise OR (|) and bit-shifting (<<).
    pub fn set(&mut self, index: usize) -> bool {
        match self.check_index(index) {
            Some((word_index, bit_index)) => {
                self.vec_bits[word_index] |= 1 << bit_index;
                self.set_indices_active_bits.insert(index);
                true
            },
            None => false
        }
    }
    /// Turns a specific bit OFF using bitwise AND (&) and bitwise NOT (!).
    pub fn clear(&mut self, index: usize) -> bool {
        match self.check_index(index) {
            Some((word_index, bit_index)) => {
                self.vec_bits[word_index] &= !(1 << bit_index);
                self.set_indices_active_bits.remove(&index);
                true
            },
            None => false
        }
    }
    /// Checks if a specific bit is set.
    pub fn test(&self, index: usize) -> Option<bool> {
        match self.check_index(index) {
            Some((word_index, bit_index)) => {
                Some(self.vec_bits[word_index] & (1 << bit_index) != 0)
            },
            None => None
        }
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for word in self.vec_bits.iter() {
            write!(f, "{:08b},", *word)?;
        }
        write!(f, "\n\nActive bit indices: ")?;
        for index in self.set_indices_active_bits.iter() {
            write!(f, "{},", *index)?;
        }
        Ok(())
    }
}

impl IntoIterator for BitSet {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut indices: Vec<usize> = self.set_indices_active_bits.into_iter().collect();
        indices.sort();
        indices.into_iter()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset_basic_operations() {
        let mut bs = BitSet::new(128);

        // Initially all bits should be false
        assert_eq!(bs.test(0), Some(false));
        assert_eq!(bs.test(64), Some(false));

        // Set specific bits
        assert!(bs.set(0));
        assert!(bs.set(65));
        assert!(bs.set(127));

        // Test active bits
        assert_eq!(bs.test(0), Some(true));
        assert_eq!(bs.test(65), Some(true));
        assert_eq!(bs.test(127), Some(true));
        assert_eq!(bs.test(64), Some(false));

        // Test out-of-bounds handling
        assert_eq!(bs.test(128), None);
        assert!(!bs.set(128));
        assert!(!bs.clear(128));

        // Clear a bit and verify
        assert!(bs.clear(65));
        assert_eq!(bs.test(65), Some(false));
    }

    #[test]
    fn test_bitset_into_iter() {
        let mut bs = BitSet::new(100);
        bs.set(10);
        bs.set(3);
        bs.set(77);

        let active_indices: Vec<usize> = bs.into_iter().collect();

        // Since we sorted them in into_iter, they will match in order
        assert_eq!(active_indices, vec![3, 10, 77]);
    }
}