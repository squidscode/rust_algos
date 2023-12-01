use std::env;
use std::fs;
use std::string;

pub fn solve() {
    let file_path = "resources/day1_input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total: usize = 0;

    let alpha_digits = vec!["_", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let z = '0' as usize;
    // println!("{:?}", "hello".rfind("0"));
    for line in lines { 
        let mut lft = line.find(&['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        let mut rgt = line.rfind(&['1', '2', '3', '4', '5', '6', '7', '8', '9']);
        let mut lv = line.chars().nth(lft.unwrap()).unwrap() as usize - z;
        let mut rv = line.chars().nth(rgt.unwrap()).unwrap() as usize - z;
        
        for ind in 1..alpha_digits.len() {
            let (l, r) = (line.find(alpha_digits[ind]), line.rfind(alpha_digits[ind]));
            if l.is_some() && l.unwrap() < lft.unwrap() {
                lft = l;
                lv = ind;
            }
            if r.is_some() && r.unwrap() > rgt.unwrap() {
                rgt = r;
                rv = ind;
            }
        }
        println!("{} .. {:?} {:?}", line, lv, rv);
        total += 10 * lv + rv;
        
    }
    println!("total {}", total);

    // for line in lines {
    //     let matches: Vec<usize> = re
    //         .find_iter(line)
    //         .map(|m| m.as_str())
    //         .map(|s| {
    //             
    //         })
    //         .collect();
    //     println!("{} == {:?}", line, matches);
    //     let val = 10 * matches.first().unwrap() + matches.last().unwrap();
    //     total += val;
    // }
    // println!("total: {}", total);
}
