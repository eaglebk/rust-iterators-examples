use std::hint::black_box;
use std::time::Instant;

fn eager_intermediate_vecs(data: &[i32]) -> i64 {
    let positives: Vec<i32> = data.iter().copied().filter(|&x| x > 0).collect();
    let doubled: Vec<i64> = positives.iter().map(|&x| (x as i64) * 2).collect();
    doubled.iter().sum()
}

fn lazy_iterator_chain(data: &[i32]) -> i64 {
    data.iter()
        .copied()
        .filter(|&x| x > 0)
        .map(|x| (x as i64) * 2)
        .sum()
}

fn manual_for_loop(data: &[i32]) -> i64 {
    let mut sum = 0i64;
    for &x in data {
        if x > 0 {
            sum += (x as i64) * 2;
        }
    }
    sum
}

fn main() {
    let n = 10_000_000usize;
    let data: Vec<i32> = (0..n as i32)
        .map(|i| if i % 3 == 0 { -i } else { i % 100 })
        .collect();

    println!("Benchmarking on {n} i32 elements (~40 MB)...\n");

    let t1 = Instant::now();
    let r1 = eager_intermediate_vecs(black_box(&data));
    let d1 = t1.elapsed();

    let t2 = Instant::now();
    let r2 = lazy_iterator_chain(black_box(&data));
    let d2 = t2.elapsed();

    let t3 = Instant::now();
    let r3 = manual_for_loop(black_box(&data));
    let d3 = t3.elapsed();

    assert_eq!(r1, r2);
    assert_eq!(r2, r3);

    println!("1. Eager intermediate Vecs : {:>8.2?} (result = {r1})", d1);
    println!("2. Lazy iterator chain     : {:>8.2?} (result = {r2})", d2);
    println!("3. Manual for loop         : {:>8.2?} (result = {r3})", d3);
}
