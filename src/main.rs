use std::time::Instant;

use shader_try::fibonachi;
use rayon::prelude::*;

fn main() {
    let top = 2_u128.pow(10);
    let src_range = 1..top;
    let start = Instant::now();
    let result = src_range
        .clone()
        .into_par_iter()
        .map(fibonachi)
        .collect::<Vec<_>>();
    let took = start.elapsed();
    let mut max = 0;
    for (src, out) in src_range.zip(result.iter().copied()) {
        match out {
            Some(out) if out > max => {
                max = out;
                println!("{}: {}", src, out);
            }
            Some(_) => (),
            None => {
                println!("{}: overflowed", src);
                break;
            }
        }
    }
    println!("Took: {:?}", took);
    let start = Instant::now();
    for _ in 0..2_u128.pow(8) {
        let mut a = 1;
        for _ in 0..1000000 {
            a += 1;
        }
    }
    let took = start.elapsed();
    println!("Took: {:?}", took);

}
