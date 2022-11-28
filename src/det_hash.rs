use std::hash::{BuildHasher, Hasher};

/// A determininstic hash for debugging
pub struct DetHash {
    val: u64,
    mul: u64,
}

impl Hasher for DetHash {
    fn finish(&self) -> u64 {
        self.val
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in bytes {
            self.val = self.val.overflowing_mul(self.mul).0 + i.clone() as u64;
        }
    }
}

pub struct DetBuildHash {
    init : u64,
    mul: u64,
}

impl BuildHasher for DetBuildHash {
    type Hasher = DetHash;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher{val: self.init, mul: self.mul}
    }
}

impl Default for DetBuildHash {
    fn default() -> Self {
        Self{init: 12938561897, mul: 97587301263}
    }
}

impl Clone for DetBuildHash {
    fn clone(&self) -> Self {
        Self{init: self.init, mul: self.mul}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.init = source.init;
        self.mul = source.mul;
    }
}