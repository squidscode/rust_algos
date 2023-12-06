use std::fs;
use std::collections::{BTreeMap, BTreeSet};
// 13 points
// 30 points for part 2

pub fn solve() {
    let contents = fs::read_to_string("resources/d4.txt")
        .expect("Cannot open file to read");
    let mut instance_map = BTreeMap::<usize, usize>::new();
    instance_map.insert(0, 1);
    for (i, line) in contents.lines().enumerate() {
        let mut partitions = line.split(&[':', '|']);
        partitions.next(); /* Ignore the first partition */
        let winners: BTreeSet<usize> = partitions
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter_map(|str| {str.trim().parse::<usize>().ok()})
            .collect();
        let mut wins = 0usize;
        partitions.next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|str| {str.parse::<usize>()})
            .for_each(|v| {
                if v.is_ok() && winners.contains(&v.unwrap()) {
                    wins += 1;
                }
            });
        let mut multiplier = instance_map.get(&i);
        if multiplier.is_none() {
            instance_map.insert(i, 1);
            multiplier = Some(&1);
        }
        let multiplier = *multiplier.unwrap();
        for offset in 1..wins+1 {
            instance_map.insert(
                i + offset,
                *instance_map.get(&(i + offset)).unwrap_or(&1usize) + multiplier
            );
        }
    }
    let mut total = 0usize;
    instance_map.iter()
        .for_each(|(_, v)| {
            total += *v;
        });
    println!("instance map: {:?}", instance_map);
    println!("total: {:?}", total);
}

pub fn solvev1() {
    let contents = fs::read_to_string("resources/d4.txt")
        .expect("Cannot open file to read");
    let mut total_points = 0usize;
    for line in contents.lines() {
        let mut partitions = line.split(&[':', '|']);
        partitions.next(); /* Ignore the first partition */
        let winners: BTreeSet<usize> = partitions
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter_map(|str| {str.trim().parse::<usize>().ok()})
            .collect();
        let mut points_for_card = 0usize;
        partitions.next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|str| {str.parse::<usize>()})
            .for_each(|v| {
                if v.is_ok() && winners.contains(&v.unwrap()) {
                    points_for_card = 1.max(2 * points_for_card);
                }
            });
        total_points += points_for_card;
    }
    println!("total points: {}", total_points);
}