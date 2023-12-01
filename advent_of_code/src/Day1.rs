use std::env;
use std::fs;
use std::string;

pub fn solve() {
    let file_path = "resources/day1_input.txt";
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total: u32 = 0;

    for line in lines {
        // println!("{}", line);
        let mut nums: Vec<u32> = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                nums.push(c as u32 - '0' as u32);
            }
        }
        let val = 10 * nums.first().unwrap() + nums.last().unwrap();
        total += val;
    }
    println!("total: {}", total);
}
