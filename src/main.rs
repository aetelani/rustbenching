
const ITERS: usize = 100000000;
fn main() {
    let _ = ITERS;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::ITERS;
        use std::time::Instant;
        let mut start = Instant::now();
        let mut result = pi_calc::calc_pi_fold_inline(ITERS);
        let mut duration = start.elapsed().as_millis();
        assert_eq!(result, 3.141592663589326);
        println!("Leibniz Pi perf testing with {ITERS} iteration with fold_inline took {duration}ms");

        start = Instant::now();
        result = pi_calc::calc_pi_foreach_inline(ITERS);
        duration = start.elapsed().as_millis();
        assert_eq!(result, 3.141592663589326);
        println!("Leibniz Pi perf testing with {ITERS} iteration with foreach_inline took {duration}ms");
    }
}
