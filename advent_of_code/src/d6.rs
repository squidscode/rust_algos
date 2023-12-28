use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("resources/d6.txt")
        .expect("Cannot open file to read");
    let mut lines = contents.lines();
    let line_1 = lines.next().unwrap();
    let line_2 = lines.next().unwrap();
    let mut l1 = line_1.split_ascii_whitespace();
    let mut l2 = line_2.split_ascii_whitespace();
    l1.next(); l2.next();
    let mut ans = 1u64;
    let mut time: String = String::from("");
    let mut distance: String = String::from("");
    for races in l1.zip(l2) {
        time += races.0;
        distance += races.1;
    }

    let time = str::parse::<u64>(time.as_str()).unwrap();
    let distance = str::parse::<u64>(distance.as_str()).unwrap();
    let mut freq = 0u64;
    for i in 0..time+1 {
        if i * (time - i) > distance {
            freq += 1;
        }
    }
    println!("time: {time} distance: {distance} freq {}", freq);
}



pub fn solvev1() {
    let contents = fs::read_to_string("resources/d6.txt")
        .expect("Cannot open file to read");
    let mut lines = contents.lines();
    let line_1 = lines.next().unwrap();
    let line_2 = lines.next().unwrap();
    let mut l1 = line_1.split_ascii_whitespace();
    let mut l2 = line_2.split_ascii_whitespace();
    l1.next(); l2.next();
    let mut ans = 1u64;
    for races in l1.zip(l2) {
        let time = str::parse::<u64>(races.0).unwrap();
        let distance = str::parse::<u64>(races.1).unwrap();
        let mut freq = 0u64;
        for i in 0..time+1 {
            if i * (time - i) > distance {
                freq += 1;
            }
        }
        println!("freq {}", freq);
        ans *= freq;
    }
    println!("ans {}", ans);
}

