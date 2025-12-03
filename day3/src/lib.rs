#[derive(Debug)]
#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &str) {
        let sum: u32 = file
            .lines()
            .map(|line| {
                let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
                let (max_index, max1) = digits
                    .iter()
                    .enumerate()
                    .take(digits.len() - 1)
                    .max_by_key(|&(i, &val)| (val, -(i as isize)))
                    .unwrap();
                (max1.to_string() + &digits.iter().skip(max_index + 1).max().unwrap().to_string())
                    .parse::<u32>()
                    .unwrap()
            })
            .sum();
        dbg!(&sum);
    }

    pub fn second_part(file: &str) {
        let sum: u64 = file
            .lines()
            .map(|line| {
                let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
                let n = digits.len();
                let mut res = String::new();

                let mut max_index = 0;

                for check in 0..12 {
                    let rem = n - (12 - check);
                    let (index, val) = digits[max_index..=rem]
                        .iter()
                        .enumerate()
                        .max_by_key(|&(i, &val)| (val, -(i as isize)))
                        .map(|(i, &val)| (max_index + i, val))
                        .unwrap();
                    res.push(char::from(b'0' + val));
                    max_index = index + 1;
                }

                res.parse::<u64>().unwrap()
            })
            .sum();
        dbg!(&sum);
    }
}
