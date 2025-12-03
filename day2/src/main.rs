use day2::Solution;


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    //let file = std::fs::read_to_string("../inputs/test_inputs.txt")?; //1227775554  //4174379265

    let file = std::fs::read_to_string("../inputs/input.txt")?; //9188031749  //11323661261

    println!("\nFirst Part: ");
    Solution::first_part(&file);

    println!("\nSecond Part: ");
    Solution::second_part(&file);

    Ok(())
}
