use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build_metadata: String,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32, build: &str) -> Self {
        Self {
            major,
            minor,
            patch,
            build_metadata: build.to_string(),
        }
    }
}

// 1. Implement PartialEq ignoring `build_metadata`
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl Eq for Version {}

// 2. Implement Hash manually to match PartialEq
impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.major.hash(state);
        self.minor.hash(state);
        self.patch.hash(state);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_custom_hash_and_eq() {
        let v1 = Version::new(2, 1, 4, "build.100");
        let v2 = Version::new(2, 1, 4, "build.200");
        let v3 = Version::new(2, 1, 5, "build.100");

        // Should be equal despite different build metadata
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);

        // Test hash consistency for HashSet usage
        let mut set = HashSet::new();
        set.insert(v1);

        // v2 should be recognized as already existing in the set because their hashes and equality match
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));
    }
}