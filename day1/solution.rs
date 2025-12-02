use std::time::Instant;

#[derive(Debug)]
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &String) {
        let start = Instant::now();
        let (mut val, mut res) = (50_i32, 0);
        for content in file.lines() {
            let (dir, n) = content.split_at(1);
            let mut n: i32 = n.parse().unwrap();

            if n % 100 == 0 {
                continue;
            }

            if dir == "L" {
                n = -n;
            }

            val = (val + n) % 100;
            if val == 0 {
                res += 1;
            }
        }
        dbg!(&start.elapsed());
        dbg!(&res);
    }

    pub fn second_part(file: &String) {
        let start = Instant::now();
        let (mut val, mut res) = (50, 0);
        for content in file.lines() {
            let (dir, n) = content.split_at(1);
            let mut n: i32 = n.parse().unwrap();
            if n == 0 {
                continue;
            }
            let d_first = match dir {
                "R" => {
                    if val == 0 {
                        100
                    } else {
                        100 - val
                    }
                }
                "L" => {
                    if val == 0 {
                        100
                    } else {
                        val
                    }
                }
                _ => continue,
            };
            if n >= d_first {
                res += 1 + (n - d_first) / 100;
            }
            if dir == "L" {
                n = -n;
            }

            val = (val + n).rem_euclid(100);
        }
        dbg!(&start.elapsed());
        dbg!(&res);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("test_inputs.txt")?;
    let file = std::fs::read_to_string("inputs.txt")?;
    //Solution::first_part(&file);
    Solution::second_part(&file);

    Ok(())
}
