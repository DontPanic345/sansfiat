//! mulberry32, bit-for-bit the same as the JS prototypes' `rand(state)`.

pub fn rand(state: &mut u32) -> f64 {
    *state = state.wrapping_add(0x6d2b79f5);
    let mut t = *state;
    t = (t ^ (t >> 15)).wrapping_mul(t | 1);
    t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 61));
    (t ^ (t >> 14)) as f64 / 4294967296.0
}

pub fn rand_index(state: &mut u32, n: usize) -> usize {
    (rand(state) * n as f64) as usize
}
