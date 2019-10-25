use rand::rngs::SmallRng;
use rand::{FromEntropy, Rng};
use std::thread;
use std::time::{Duration, Instant};

fn calc(start: u32, end: u32) -> u32 {
    let mut rng = SmallRng::from_entropy();
    let mut sum = 0u32;

    for _ in start..end {
        sum = sum.wrapping_add(rng.gen::<u32>().count_ones());
    }

    sum
}

fn calc_mt(count: usize) -> u32 {
    const TOTAL: u32 = 10000000;

    let mut threads = Vec::with_capacity(count);
    let single = TOTAL / count as u32;

    for i in 0..count as u32 {
        let start = i * single;
        let end = if i == count as u32 - 1 {
            TOTAL
        } else {
            (i + 1) * single
        };

        threads.push(thread::spawn(move || calc(start, end)));
    }

    threads
        .into_iter()
        .fold(0, |acc, thread| acc.wrapping_add(thread.join().unwrap()))
}

fn main() {
    let start = Instant::now();
    let sum = calc_mt(num_cpus::get());
    let elapsed = start.elapsed();

    println!("{:?}, {}", elapsed, sum);
}
