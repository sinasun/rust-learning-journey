use std::time::Instant;

fn factorial(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        factorial(n - 1, n * acc)
    }
}

fn bench_tail_recursion() {
    for i in 0..9_999_999 {
        let _res = factorial(i % 999_999, 1);
    }
}

fn main() {
    let start_time = Instant::now();
    bench_tail_recursion();
    let elapsed_time = start_time.elapsed().as_nanos();
    println!("Execution time: {} nanoseconds", elapsed_time);
}
