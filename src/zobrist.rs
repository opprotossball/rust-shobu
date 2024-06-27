use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct Zobrist {
    pub piece_vals: [[u64; 16]; 2]
}

impl Zobrist {
    
    pub fn new() -> Self  {
        let mut rand = StdRng::seed_from_u64(2137);
        Zobrist {
            piece_vals: rand.gen()
        }
    }
}