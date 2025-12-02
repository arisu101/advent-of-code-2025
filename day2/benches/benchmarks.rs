use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Debug)]
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &str) {
        let mut inv = 0_u64;
        for (start, end) in Self::parse_ranges(file) {
            for i in start..=end {
                if Self::check_invalid_first(i) {
                    inv += i;
                }
            }
        }
    }

    #[inline]
    pub fn check_invalid_first(nums: u64) -> bool {
        let s = nums.to_string();
        let check = s.as_bytes();
        let n = check.len();
        if n % 2 != 0 {
            return false;
        }

        check[..n / 2] == check[n / 2..]
    }

    // #[inline]
    // pub fn check_invalid_second(nums: u64) -> bool {
    //     let s = nums.to_string();
    //     let check = s.as_bytes();
    //     let n = check.len();
    //
    //     if n % 2 == 0 && check[..n / 2] == check[n / 2..] {
    //         return true;
    //     }
    //
    //     for size in 1..=n / 2 {
    //         if n % size != 0 {
    //             continue;
    //         }
    //
    //         let first_chunck = &check[..size];
    //         let mut pos = size;
    //         let mut ok = true;
    //         while pos < n {
    //             if &check[pos..pos + size] != first_chunck {
    //                 ok = false;
    //                 break;
    //             }
    //             pos += size;
    //         }
    //         if ok {
    //             return true;
    //         }
    //     }
    //
    //     false
    // }

    //Second Approach
    #[inline]
    pub fn check_invalid_second(nums: u64) -> bool {
        let s = nums.to_string();
        let check = s.as_bytes();

        let n = check.len();

        if check[..n / 2] == check[n / 2..] {
            return true;
        }

        for size in 1..=n / 2 {
            let chuncks: Vec<&[u8]> = check.chunks(size).collect();
            if chuncks.windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }

        false
    }

    pub fn second_part(file: &str) {
        let mut inv = 0_u64;
        for (start, end) in Self::parse_ranges(file) {
            for i in start..=end {
                if Self::check_invalid_second(i) {
                    inv += i;
                }
            }
        }
    }

    pub fn parse_ranges(file: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
        file.split(',').map(|r| {
            let mut parts = r.trim().split('-');
            (
                parts.next().unwrap().parse::<u64>().unwrap(),
                parts.next().unwrap().parse::<u64>().unwrap(),
            )
        })
    }
}

fn benchmark_second_part(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input.txt").unwrap();
    c.bench_function("second_part", |b| {
        b.iter(|| Solution::second_part(&file));
    });
}

fn benchmark_first_part(c: &mut Criterion) {
    let file = std::fs::read_to_string("inputs/input.txt").unwrap();
    c.bench_function("first_part", |b| {
        b.iter(|| Solution::first_part(&file));
    });
}

criterion_group!(benches, benchmark_first_part, benchmark_second_part);
criterion_main!(benches);

// #[allow(dead_code)]
// #[allow(unused_variables)]
// #[allow(unused_assignments)]
