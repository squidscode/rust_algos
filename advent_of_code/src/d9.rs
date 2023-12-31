use std::fs;

pub fn solve() {
    let contents = fs::read_to_string("resources/d9.txt").unwrap();
    let mut sequences = Vec::<Vec<i64>>::new();
    for line in contents.lines() {
        let num_splits = line.split_ascii_whitespace();
        sequences.push(vec![]);
        for num_str in num_splits {
            sequences.last_mut().unwrap().push(num_str.parse().unwrap());
        }
    }
    println!("{sequences:?}");
    let mut ans = 0i64;
    for seq in &sequences {
        let mut deltas = Vec::<Vec<i64>>::new();
        deltas.push(vec![]);
        for i in 0..seq.len()-1 {
            deltas[0].push(seq[i+1] - seq[i]);
        }
        while !deltas.last().unwrap().iter().all(|num| {*num == 0}) {
            let lst = deltas.last().unwrap();
            let mut new_vec = vec![];
            for i in 0..lst.len()-1 {
                new_vec.push(lst[i+1] - lst[i]);
            }
            deltas.push(new_vec);
        }
        println!("seq: {seq:?}\ndeltas: {deltas:?}");
        let mut diff = 0i64;
        for vect in deltas.iter().rev() {
            /* a[i+1] - a[i] = diff --> a[i] = a[i+1] - diff */
            diff = vect.first().unwrap() - diff;
        }
        diff = seq.first().unwrap() - diff;
        println!("diff: {diff}");
        ans += diff;
    }
    println!("ans: {ans}");
    
}

pub fn solvev1() {
    let contents = fs::read_to_string("resources/d9.txt").unwrap();
    let mut sequences = Vec::<Vec<i64>>::new();
    for line in contents.lines() {
        let num_splits = line.split_ascii_whitespace();
        sequences.push(vec![]);
        for num_str in num_splits {
            sequences.last_mut().unwrap().push(num_str.parse().unwrap());
        }
    }
    println!("{sequences:?}");
    let mut ans = 0i64;
    for seq in &sequences {
        let mut deltas = Vec::<Vec<i64>>::new();
        deltas.push(vec![]);
        for i in 0..seq.len()-1 {
            deltas[0].push(seq[i+1] - seq[i]);
        }
        while !deltas.last().unwrap().iter().all(|num| {*num == 0}) {
            let lst = deltas.last().unwrap();
            let mut new_vec = vec![];
            for i in 0..lst.len()-1 {
                new_vec.push(lst[i+1] - lst[i]);
            }
            deltas.push(new_vec);
        }
        println!("seq: {seq:?}\ndeltas: {deltas:?}");
        let mut diff = 0i64;
        for vect in deltas.iter().rev() {
            /* a[i+1] - a[i] = diff --> a[i+1] = diff + a[i] */
            diff = diff + vect.last().unwrap();
        }
        diff = diff + seq.last().unwrap();
        println!("diff: {diff}");
        ans += diff;
    }
    println!("ans: {ans}");
    
}