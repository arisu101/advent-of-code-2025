use day4::Solution;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("../inputs/tinput.txt")?; //13  //43

    let file = std::fs::read_to_string("../inputs/input.txt")?; //1460  //9243

    println!("\nFirst Part: ");
    Solution::first_part(&file);

    println!("\nSecond Part: ");
    Solution::second_part(&file);

    Ok(())
}
