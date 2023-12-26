use std::cmp::Ordering;
use std::fs;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Span {
    source: u64,
    dest: u64,
    length: u64
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.source < other.source {
            return Ordering::Less;
        } else if self.source == other.source {
            return Ordering::Equal;
        } else {
            return Ordering::Greater;
        }
    }
}

pub fn solve() {
    let contents = fs::read_to_string("resources/d5.txt")
        .expect("Cannot open file to read");
    let mut maps: Vec<BTreeSet<Span>> = vec![];
    let mut seeds: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut lines = contents.lines();
    let first_line = lines.by_ref().next().unwrap();
    let mut ind = 1;
    let splits: Vec<&str> = first_line.split(' ').collect();
    let p = |str| {
        str::parse::<u64>(str).unwrap()
    };
    while ind < splits.len() {
        seeds.insert((p(splits[ind]), p(splits[ind+1])));
        ind += 2;
    }
    println!("{:?}", seeds);
    for line in lines {
        if line.contains("map:") {
            maps.push(BTreeSet::new());
        } else {
            let (d, s, l) = super::scan!(line, ' ', u64, u64, u64);
            if s.is_none() || d.is_none() || l.is_none() { continue; }
            let (s,d,l) = (s.unwrap(), d.unwrap(), l.unwrap());
            maps.last_mut().unwrap().insert(Span {
                source: s,
                dest: d,
                length: l
            });
        }
    }
    // println!("{:?}", maps);
    println!("seeds before: {:?}", seeds);
    seeds = merge(seeds);
    println!("seeds after: {:?}", seeds);
    let mut ranges = seeds;
    for map in maps {
        let mut new_ranges: BTreeSet<(u64, u64)> = BTreeSet::new();
        for range in ranges.clone() {
            let mut intersecting_spans = Vec::<&Span>::new();
            for span in &map {
                /*
                 * There are two cases:
                 * - Either span.source <= range.0 or span.source > range.0.
                 */
                if span.source <= range.0 && span.source + span.length > range.0 {
                    intersecting_spans.push(span);
                } else if span.source > range.0 && span.source < range.0 + range.1 {
                    intersecting_spans.push(span);
                }
            }
            intersecting_spans.sort();
            // println!("range {:?} has spans {:?}", range, intersecting_spans);
            let mut lft = range.0;
            for span in intersecting_spans {
                if span.source > lft {
                    new_ranges.insert((lft, span.source - lft));
                    lft = span.source;
                }
                let lft_offset = std::cmp::min(span.source + span.length - lft, range.0 + range.1 - lft);
                new_ranges.insert((span.dest + lft - span.source, lft_offset));
                lft += lft_offset;
            }
            if lft < range.0 + range.1 {
                new_ranges.insert((lft, range.0 + range.1 - lft));
            }
        }
        println!("{:?} --> {:?}", ranges, new_ranges);
        new_ranges = merge(new_ranges);
        ranges = new_ranges;
    }
    println!("{:?}", ranges);
}

pub fn merge(mut set: BTreeSet<(u64, u64)>) -> BTreeSet<(u64, u64)> {
    let mut conflict = merge_conflict(&set);
    while let Some(conf) = conflict {
        // println!("merging {:?}", conf);
        assert!(set.remove(&conf.0));
        assert!(set.remove(&conf.1));
        let new_lft = std::cmp::max(conf.0.0 + conf.0.1, conf.1.0 + conf.1.1);
        set.insert((conf.0.0, new_lft - conf.0.0));
        conflict = merge_conflict(&set);
    }
    return set;
}

pub fn merge_conflict(set: &BTreeSet<(u64, u64)>) -> Option<((u64, u64), (u64, u64))> {
    for pair in set {
        for other_pair in set {
            if other_pair != pair && pair.0 <= other_pair.0 && other_pair.0 <= pair.0 + pair.1 {
                return Some((*pair, *other_pair));
            }
        }
    }
    return None;
}

pub fn solvev1() {
    let contents = fs::read_to_string("resources/d5.txt")
        .expect("Cannot open file to read");
    let mut maps: Vec<BTreeSet<Span>> = vec![];
    let mut seeds: BTreeSet<u64> = BTreeSet::new();
    let mut lines = contents.lines();
    let first_line = lines.by_ref().next().unwrap();
    first_line.split(' ').for_each(|str| {
        if let Ok(val) = str::parse::<u64>(str) {
            seeds.insert(val);
        }
    });
    println!("{:?}", seeds);
    for line in lines {
        if line.contains("map:") {
            maps.push(BTreeSet::new());
        } else {
            let (d, s, l) = super::scan!(line, ' ', u64, u64, u64);
            if s.is_none() || d.is_none() || l.is_none() { continue; }
            let (s,d,l) = (s.unwrap(), d.unwrap(), l.unwrap());
            maps.last_mut().unwrap().insert(Span {
                source: s,
                dest: d,
                length: l
            });
        }
    }
    // println!("{:?}", maps);
    let mut lowest: Option<u64> = None;
    for seed in seeds {
        // println!("seed: {}", seed);
        let mut current_val = seed;
        // let mut mapped = false;
        for map in &maps {
            // println!(".. {}", current_val);
            for span in map {
                // println!("checking {:?}", span);
                if span.source <= current_val && span.source + span.length > current_val {
                    current_val = span.dest + (current_val - span.source);
                    break;
                }
            }
        }
        println!("seed: {} -> {}", seed, current_val);
        if let Some(v) = lowest {
            lowest = Some(if v < current_val {v} else {current_val});
        } else {
            lowest = Some(current_val);
        }
    }
    println!("{:?}", lowest);
}