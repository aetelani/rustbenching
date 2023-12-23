#[inline]
pub fn calc_pi_fold_inline(rounds: usize) -> f64 {
    (2..rounds).fold(1.0, |pi, i| {
        let x = -1.0f64 + (2.0 * (i & 0x1) as f64);
        pi + x / (2 * i - 1) as f64
    }) * 4.0
}

pub fn calc_pi_fold(rounds: usize) -> f64 {
    (2..rounds).fold(1.0, |pi, i| {
        let x = -1.0f64 + (2.0 * (i & 0x1) as f64);
        pi + x / (2 * i - 1) as f64
    }) * 4.0
}

#[inline]
pub fn calc_pi_foreach_inline(rounds: usize) -> f64 {
    let mut result: f64 = 1.0;
    (2..rounds).for_each(|i| {
        let x = -1.0f64 + (2.0 * (i & 0x1) as f64);
        result += x / (2 * i - 1) as f64;
    });
    result * 4.
}

pub fn calc_pi_foreach(rounds: usize) -> f64 {
    let mut pi: f64 = 1.0;
    (2..rounds).for_each(|i| {
        let x = -1.0f64 + (2.0 * (i & 0x1) as f64);
        pi += x / (2 * i - 1) as f64;
    });
    pi * 4.
}
