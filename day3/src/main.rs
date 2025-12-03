use day3::Solution;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("../inputs/test_inputs.txt")?; //357 //3121910778619

    let file = std::fs::read_to_string("../inputs/input.txt")?; //17229   //170520923035051

    println!("\nFirst Part: ");
    Solution::first_part(&file);

    println!("\nSecond Part: ");
    Solution::second_part(&file);

    Ok(())
}
