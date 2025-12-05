use day5::Solution;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("../inputs/tinput.txt")?; //3  //14

    let file = std::fs::read_to_string("../inputs/input.txt")?; //601  //367899984917516

    println!("\nFirst Part: ");
    Solution::first_part(&file);

    println!("\nSecond Part: ");
    Solution::second_part(&file);

    Ok(())
}
