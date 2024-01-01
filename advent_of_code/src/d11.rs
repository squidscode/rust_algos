use std::fs;
use std::cmp::{min, max};

pub fn solve() {
    let contents = fs::read_to_string("resources/d11.txt").unwrap();
    let mut cs = Vec::<Vec<char>>::new();
    for line in contents.lines() {
        cs.push(line.chars().collect());
    }
    let mut rows = vec![];
    let mut cols = vec![];
    
    for (row, v) in cs.iter().enumerate() {
        let mut b = true;
        for elm in v {
            b &= *elm != '#';
        }
        if b {
            rows.push(row);
        }
    }
    for col in 0..cs.first().unwrap().len() {
        let mut b = true;
        for row in 0..cs.len() {
            if cs[row][col] == '#' {}
            b &= cs[row][col] != '#';
        }
        if b {
            cols.push(col);
        }
    }
    println!("rows: {rows:?}");
    println!("cols: {cols:?}");

    // println!("Inserting rows and cols");
    // rows.iter().rev().for_each(|row| {
    //     cs.insert(*row, cs[*row].clone());
    // });
    // cols.iter().rev().for_each(|col| {
    //     cs.iter_mut().for_each(|row| {
    //         row.insert(*col, '.');
    //     });
    // });

    let mut hashes = Vec::<(usize, usize)>::new();
    for (row, v) in cs.iter().enumerate() {
        for (col, c) in v.iter().enumerate() {
            if *c == '#' {
                hashes.push((row, col));
            }
        }
    }
    println!("hashes: {hashes:?}, hashes.len(): {}", hashes.len());
    let mut distance = 0usize;
    let factor = 1_000_000usize;
    let expansion_factor = factor - 1usize;
    for i in 0..hashes.len() {
        for j in i+1..hashes.len() {
            print!("Galaxy {} to {}. ", i+1, j+1);
            let mut expansions = 0usize;
            for rn in min(hashes[i].0,hashes[j].0)..max(hashes[i].0,hashes[j].0) {
                if rows.contains(&rn) {expansions += 1}
            }
            for cn in min(hashes[i].1,hashes[j].1)..max(hashes[i].1,hashes[j].1) {
                if cols.contains(&cn) {expansions += 1}
            }
            print!("Expansions: {}. ", expansions);
            let diff = hashes[i].0.abs_diff(hashes[j].0) + hashes[i].1.abs_diff(hashes[j].1) + expansions * expansion_factor;
            println!("Diff: {diff}.");
            distance += diff;
        }
    }
    println!("distance: {distance}");
}