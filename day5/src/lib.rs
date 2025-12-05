#[derive(Debug)]
#[allow(dead_code)]
pub struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn first_part(file: &str) {
        let (id_ranges, ids) = Self::parse_input(file);
        let mut fresh = 0u64;
        for (start, end) in id_ranges {
            for &id in &ids {
                if id >= start && id <= end {
                    fresh += 1;
                }
            }
        }
        // dbg!(&fresh);
    }

    pub fn second_part(file: &str) {
        let (id_ranges, _) = Self::parse_input(file);
        let mut fresh = 0u64;
        for (start, end) in &id_ranges {
            fresh += end - start + 1;
        }

        // dbg!(&fresh);
    }

    pub fn parse_input(file: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
        //remove id ranges and id  with ranges sorted so to remove all the duplicate
        //ranges or interval
        file.split_once("\n\n")
            .map(|(id_ranges, ids)| {
                let mut ranges: Vec<(u64, u64)> = id_ranges
                    .lines()
                    .map(|range| {
                        let point = range.find('-').unwrap();
                        (
                            range[..point].parse::<u64>().unwrap(),
                            range[point + 1..].parse::<u64>().unwrap(),
                        )
                    })
                    .collect();
                //sort the start range of a single range
                ranges.sort_unstable_by_key(|r| r.0);

                //combine the overlapping ranges
                let mut j = 0;
                for i in 1..ranges.len() {
                    if ranges[j].1 >= ranges[i].0 {
                        ranges[j].1 = ranges[j].1.max(ranges[i].1);
                    } else {
                        j += 1;
                        ranges[j] = ranges[i];
                    }
                }
                ranges.truncate(j + 1);

                (
                    //return the non-overlapping ranges
                    ranges,
                    ids.lines().map(|id| id.parse::<u64>().unwrap()).collect(),
                )
            })
            .unwrap()
    }
}
