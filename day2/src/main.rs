use std::time::Instant;

#[derive(Debug)]
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &str) {
        let start_time = Instant::now();
        let mut inv = 0_u64;
        for (start, end) in Self::parse_ranges(file) {
            for i in start..=end {
                if Self::check_invalid_first(i) {
                    inv += i;
                }
            }
        }
        dbg!(&start_time.elapsed());
        dbg!(&inv);
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

    #[inline]
    pub fn check_invalid_second(nums: u64) -> bool {
        let s = nums.to_string();
        let check = s.as_bytes();
        let n = check.len();

        if n % 2 == 0 && check[..n / 2] == check[n / 2..] {
            return true;
        }

        for size in 1..=n / 2 {
            if n % size != 0 {
                continue;
            }

            let first_chunck = &check[..size];
            let mut pos = size;
            let mut ok = true;
            while pos < n {
                if &check[pos..pos + size] != first_chunck {
                    ok = false;
                    break;
                }
                pos += size;
            }
            if ok {
                return true;
            }
        }

        false
    }

    // Second Approach
    // #[inline]
    // pub fn check_invalid_second(nums: u64) -> bool {
    //     let s = nums.to_string();
    //     let check = s.as_bytes();
    //
    //     let n = check.len();
    //
    //     if check[..n / 2] == check[n / 2..] {
    //         return true;
    //     }
    //
    //     for size in 1..=n / 2 {
    //         let chuncks: Vec<&[u8]> = check.chunks(size).collect();
    //         if chuncks.windows(2).all(|w| w[0] == w[1]) {
    //             return true;
    //         }
    //     }
    //
    //     false
    // }

    pub fn second_part(file: &str) {
        let start_time = Instant::now();
        let mut inv = 0_u64;
        for (start, end) in Self::parse_ranges(file) {
            for i in start..=end {
                if Self::check_invalid_second(i) {
                    inv += i;
                }
            }
        }
        dbg!(&start_time.elapsed());
        dbg!(&inv);
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

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("../inputs/test_inputs.txt")?; //4174379265

    let file = std::fs::read_to_string("../inputs/input.txt")?; //11323661261

    //Solution::first_part(&file);
    Solution::second_part(&file);

    Ok(())
}
