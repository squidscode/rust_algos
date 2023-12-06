use std::fs;
use std::collections::{BTreeSet, BTreeMap};

pub fn solve() {
    let str_contents = fs::read_to_string("resources/d3.txt")
    .expect("Cannot read path to string.");
    let mut total = 0usize;
    let mut ln = 0usize;
    let mut symbol_set = BTreeSet::<(usize, usize)>::new();
    for str in str_contents.lines() {
        let mut matches = str.match_indices(|c: char| {c == '*'});
        for (lft, s) in matches {
            println!("inserting {:?}", (ln, lft));
            symbol_set.insert((ln, lft));
        }
        ln += 1;
    }
    let mut gear_map = BTreeMap::<(usize, usize), Vec<usize>>::new();
    for (ln, str) in str_contents.lines().enumerate() {
        let sp = str.split_terminator(|c: char| {!c.is_numeric()});
        let mut ind = 0usize;
        for mat in sp {
            if mat != "" {
                let v = mat.parse::<usize>().unwrap();
                let (t, b) = (ln.max(1) - 1, ln + 1);
                let (l, r) = (ind.max(1) - 1, ind + mat.len());
                let mut is_part_num = false;
                for line_num in t..b+1 {
                    for row_num in l..r+1 {
                        if symbol_set.contains(&(line_num, row_num)) {
                            if !gear_map.contains_key(&(line_num, row_num)) {
                                gear_map.insert((line_num, row_num), Vec::<usize>::new());
                            }
                            gear_map.get_mut(&(line_num, row_num))
                                .unwrap()
                                .push(v);
                        }
                    }
                }
            }
            ind += mat.len();
            ind += 1;
        }
    }
    for (gear, vect) in gear_map.iter() {
        println!("for gear at {:?} the vector is {:?}", gear, vect);
        if vect.len() == 2 {
            total += vect[0] * vect[1];
        }
    }
    println!("total: {}", total);
}

pub fn solvev1() {
    let str_contents = fs::read_to_string("resources/d3.txt")
    .expect("Cannot read path to string.");
    let mut total = 0usize;
    let mut ln = 0usize;
    let mut symbol_set = BTreeSet::<(usize, usize)>::new();
    for str in str_contents.lines() {
        let mut matches = str.match_indices(|c: char| {c != '.' && !c.is_numeric()});
        for (lft, s) in matches {
            // println!("inserting {:?}", (ln, lft));
            symbol_set.insert((ln, lft));
        }
        ln += 1;
    }
    for (ln, str) in str_contents.lines().enumerate() {
        let sp = str.split_terminator(|c: char| {!c.is_numeric()});
        let mut ind = 0usize;
        for mat in sp {
            if mat != "" {
                let (t, b) = (ln.max(1) - 1, ln + 1);
                let (l, r) = (ind.max(1) - 1, ind + mat.len());
                let mut is_part_num = false;
                for line_num in t..b+1 {
                    for row_num in l..r+1 {
                        if symbol_set.contains(&(line_num, row_num)) {
                            // println!("for {} {:?} exists!", mat.parse::<usize>().unwrap(), (line_num, row_num));
                        }
                        is_part_num = is_part_num || symbol_set.contains(&(line_num, row_num));
                    }
                }
                if is_part_num {
                    let v = mat.parse::<usize>().unwrap();
                    // print!("{}, ", v);
                    total += v;
                } else {
                    let v = mat.parse::<usize>().unwrap();
                    println!("{} is NOT a part num", v);
                }
            }
            ind += mat.len();
            ind += 1;
        }
    }
    println!("total: {}", total);
}