pub struct Wyhash {
    seed: u64,
}

impl Wyhash {
    pub fn new(seed: u64) -> Self {
        Wyhash { seed }
    }

    pub fn next(&mut self) -> u64 {
        // update seed
        self.seed = self.seed.wrapping_add(0x60bee2bee120fc15);
        let mut result = self.seed;
        // Wyhash bit operation
        result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
        result ^ (result >> 31)
    }
}
