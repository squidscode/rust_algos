use std::fs;

macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}

pub fn solve() {
    let str_contents = fs::read_to_string("resources/d2.txt")
        .expect("Cannot read path to string.");
    let mut total = 0u32;
    for str in str_contents.lines() {
        let mut sep = str.split(|c| {
            c == ':' || c == ';'
        });
        let game_num_str = sep.next().unwrap_or_else(|| {std::process::exit(0)});
        let game_num = scan!(game_num_str, ' ', String, u32)
            .1.unwrap();
        let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
        for game_str in sep {
            let tr = game_str.trim();
            print!("game #{}", game_num);
            for ball_str in tr.split(", ") {
                let (num_balls, ball_color) = scan!(ball_str, char::is_whitespace, u32, String);
                match ball_color.as_ref().unwrap().as_str() {
                    "red" => {
                        r = r.max(num_balls.unwrap());
                    },
                    "green" => {
                        g = g.max(num_balls.unwrap());
                    },
                    "blue" => {
                        b = b.max(num_balls.unwrap());
                    },
                    _ => panic!()
                };
                print!(" [{}, {}],", num_balls.unwrap(), ball_color.unwrap());
            }
            println!();
        }
        println!("tup: ({}, {}, {})", r, g, b);
        total += r * g * b;
    }
    println!("total: {}", total);
}

pub fn solvev1() {
    let str_contents = fs::read_to_string("resources/d2.txt")
        .expect("Cannot read path to string.");
    let mut total = 0u32;
    for str in str_contents.lines() {
        let mut sep = str.split(|c| {
            c == ':' || c == ';'
        });
        let game_num_str = sep.next().unwrap_or_else(|| {std::process::exit(0)});
        let game_num = scan!(game_num_str, ' ', String, u32)
            .1.unwrap();
        let mut valid_game: bool = true;
        for game_str in sep {
            let tr = game_str.trim();
            print!("game #{}", game_num);
            for ball_str in tr.split(", ") {
                let (num_balls, ball_color) = scan!(ball_str, char::is_whitespace, u32, String);
                let max_balls = match ball_color.as_ref().unwrap().as_str() {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!()
                };
                valid_game = valid_game && (num_balls.unwrap() <= max_balls);
                print!(" [{}, {}],", num_balls.unwrap(), ball_color.unwrap());
            }
            println!();
        }
        if valid_game {
            total += game_num;
        }
    }
    println!("total: {}", total);
}