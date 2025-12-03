#[derive(Debug)]
#[allow(dead_code)]
pub struct Solution {}

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
        dbg!(&inv);
    }

    #[inline]
    pub fn check_invalid_first(nums: u64) -> bool {
        let digits = nums.ilog10() + 1;
        if digits % 2 != 0 {
            return false;
        }
        let div = 10_u64.pow(digits / 2);

        nums / div == nums % div
    }

    pub fn second_part(file: &str) {
        let mut inv = 0_u64;
        for (start, end) in Self::parse_ranges(file) {
            'outer: for i in start..=end {
                let digits = i.ilog10() + 1;
                let div = 10_u64.pow(digits / 2);
                if digits % 2 == 0 && i / div == i % div {
                    inv += i;
                    continue;
                }
                for chunck in 1..=digits / 2 {
                    if digits % chunck != 0 {
                        continue;
                    }

                    let base = 10_u64.pow(chunck);
                    let mut num = i;

                    let refr = num % base;
                    num /= base;

                    let mut ok = true;
                    while num > 0 {
                        if num % base != refr {
                            ok = false;
                            break;
                        }
                        num /= base;
                    }
                    if ok {
                        inv += i;
                        continue 'outer;
                    }
                }
            }
        }
        dbg!(&inv);
    }

    // #[inline]
    // pub fn check_invalid_second(nums: u64) -> bool {
    //     let s = nums.to_string();
    //     let check = s.as_bytes();
    //     let n = check.len();
    //
    //     let digits = nums.ilog10() + 1;
    //     let div = 10_u64.pow(digits / 2);
    //
    //     if nums / div == nums % div {
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
    //
    //
    // pub fn second_part(file: &str) {
    //     let mut inv = 0_u64;
    //     for (start, end) in Self::parse_ranges(file) {
    //         for i in start..=end {
    //             if Self::check_invalid_second(i) {
    //                 inv += i;
    //             }
    //         }
    //     }
    // }

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
