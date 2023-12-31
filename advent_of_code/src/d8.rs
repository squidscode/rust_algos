use std::fs;
use std::collections::BTreeMap;
use std::ops::Index;

pub fn solve() {
    let s = std::fs::read_to_string("resources/d8.txt").unwrap();
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let lrs = first_line.chars().collect::<Vec<char>>();
    lines.next(); // newline

    let mut node_map = <BTreeMap<&str, (&str, &str)>>::new();
    for line in lines.into_iter() {
        let mut splits = line.split_ascii_whitespace();
        let key = splits.next().unwrap(); // root
        assert!(splits.next().unwrap() == "=");
        let slice: &[_] = &['(', ',', ')'];
        let lft = splits.next().unwrap().trim_matches(slice);
        let rgt = splits.next().unwrap().trim_matches(slice);
        assert!(lft.len() == 3);
        assert!(rgt.len() == 3);
        node_map.insert(key, (lft, rgt));
    }
    println!("node_map: {node_map:?}");
    
    let mut current_node = node_map.keys().map(|str| {*str})
        .filter(|s| { String::from(*s).ends_with('A') })
        .collect::<Vec<&str>>();
    println!("Current node has {} values", current_node.len());
    let mut ind = 0;
    let mut steps = 0usize;
    let original_nodes = current_node.clone();
    let mut history = current_node.iter().map(|s| {vec![(0usize, *s)]}).collect::<Vec<_>>();
    let mut history_complete_bits = current_node.iter().map(|_| {0}).collect::<Vec<_>>();
    let mut history_next = current_node.iter().map(|s| {(0, *s)}).collect::<Vec<_>>();
    let mut indices = current_node.iter().map(|s| {0usize}).collect::<Vec<_>>();
    while !current_node.iter().all(|s| {s.to_string().ends_with('Z')}) 
        && !history_complete_bits.iter().all(|bit| {*bit == 1}) 
        {
        if steps % 100_000 == 0 { /* Log every now and then */
            println!("# of steps: {steps}");
        }
        let pairs = current_node.iter().map(
            |f| {node_map.get(f).expect("current node must exist in map.")}
        );
        current_node = pairs.map(|pair| {
            match lrs[ind] {
                'L' => pair.0,
                'R' => pair.1,
                _ => panic!()
            }
        }).collect();
        current_node.iter().enumerate().for_each(
            |(i, s)| {
                if history_complete_bits[i] == 1 {
                    let offset = indices[i];
                    if history[i][(((steps+1) - offset) % (history[i].len() - offset)) + offset] != (ind, s) {
                        let r = (((steps+1) - offset) % (history[i].len() - offset)) + offset;
                        println!("ERROR: {:?} != {:?}", history[i][r], (ind, s));
                    }
                    return;
                }
                if !history[i].contains(&(ind, s)) {
                    history[i].push((ind, s));
                } else {
                    if history_complete_bits[i] == 0 {
                        indices[i] = history[i].iter().position(|f| {f == &(ind, *s)}).unwrap();
                        history_next[i] = (ind, s);
                    }
                    history_complete_bits[i] = 1;
                }
            }
        );
        ind += 1;
        ind %= lrs.len();
        steps += 1;
    }
    println!("history: {history:?}");
    let bit_strings = history.iter().map(|s| {
        let mut string = String::new();
        for p in s {
            string += if p.1.to_string().ends_with('Z') {"1"} else {"0"};
        }
        return string;
    }).collect::<Vec<String>>();
    for s in &bit_strings {
        println!("length: {}\nString: {}", s.len(), s);
    }
    println!("history_next: {history_next:?}");
    let indices = history_next.iter().enumerate().map(|(i, p)| {
        history[i].iter().position(|history_pair| {history_pair == p}).unwrap()
    }).collect::<Vec<_>>();
    println!("indices: {indices:?}");
    let cycle_length = bit_strings.iter().map(|f| f.len()).zip(&indices).map(|f| {
        f.0 - f.1
    }).collect::<Vec<usize>>();
    println!("cycle length : {:?}", cycle_length);
    bit_strings.iter().enumerate().zip(&indices).for_each(|((i, str), offset)| {
        println!("((STEPS - {}) % ({} - {})) + {} == {}", offset, str.len(), offset, offset, 
            str.chars().position(|c| {c == '1'}).unwrap());
    });
}

pub fn solve_v1() {
    let s = std::fs::read_to_string("resources/d8.txt").unwrap();
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let lrs = first_line.chars().collect::<Vec<char>>();
    lines.next(); // newline

    let mut node_map = <BTreeMap<&str, (&str, &str)>>::new();
    for line in lines.into_iter() {
        let mut splits = line.split_ascii_whitespace();
        let key = splits.next().unwrap(); // root
        assert!(splits.next().unwrap() == "=");
        let slice: &[_] = &['(', ',', ')'];
        let lft = splits.next().unwrap().trim_matches(slice);
        let rgt = splits.next().unwrap().trim_matches(slice);
        assert!(lft.len() == 3);
        assert!(rgt.len() == 3);
        node_map.insert(key, (lft, rgt));
    }
    println!("node_map: {node_map:?}");

    let mut current_node = "AAA";
    let mut ind = 0;
    let mut steps = 0u64;
    while current_node != "ZZZ" {
        let pair = node_map.get(&current_node).expect("current node must exist in map.");
        current_node = match lrs[ind] {
            'L' => pair.0,
            'R' => pair.1,
            _ => panic!()
        };
        ind += 1;
        ind %= lrs.len();
        steps += 1;
    }
    println!("steps: {steps}");
}