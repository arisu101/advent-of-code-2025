use day2::Solution;
fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    Solution::first_part(divan::black_box(include_str!("../inputs/input.txt")));
}

#[divan::bench]
fn part2() {
    Solution::second_part(divan::black_box(include_str!("../inputs/input.txt")));
}
