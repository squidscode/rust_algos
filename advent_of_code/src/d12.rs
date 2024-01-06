use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Clone)]
struct Record {
    gears: Vec<char>,
    groups: Vec<usize>
}

pub fn solve() {
    let contents = fs::read_to_string("resources/d12.txt").unwrap();
    let mut records = vec![];
    for line in contents.lines() {
        let mut sp = line.split_ascii_whitespace().into_iter();
        records.push(Record {
            gears: sp.next().unwrap().chars().collect(),
            groups: sp.next().unwrap().split(',').map(|c| {c.parse().unwrap()}).collect()
        })
    }
    println!("records: {records:?}");
    records.iter_mut().for_each(|rec| {
        let original_gears = rec.gears.clone();
        let original_groups = rec.groups.clone();
        for _ in 0..4 {
            rec.gears.push('?');
            rec.gears.extend(original_gears.iter());
        }
        for _ in 0..4 {
            rec.groups.extend(original_groups.iter());
        }
    });
    let mut permutation_sums = 0usize;
    // let mut cache = BTreeMap::<(String,String), usize>::new();
    records.into_iter().enumerate().for_each(|(i, rec)| {
        let ans = split_permute(rec.clone());
        println!("({}) {:?} {:?} --> {}", i+1, rec.gears, rec.groups, ans);
        permutation_sums += ans;
        // std::process::exit(0);
    });
    println!("permuation_sums: {permutation_sums}");
}

pub fn solvev1() {
    let contents = fs::read_to_string("resources/d12_ex.txt").unwrap();
    let mut records = vec![];
    for line in contents.lines() {
        let mut sp = line.split_ascii_whitespace().into_iter();
        records.push(Record {
            gears: sp.next().unwrap().chars().collect(),
            groups: sp.next().unwrap().split(',').map(|c| {c.parse().unwrap()}).collect()
        })
    }
    println!("records: {records:?}");
    let mut permutation_sums = 0usize;
    records.into_iter().enumerate().for_each(|(i, rec)| {
        let ans = split_permute(rec.clone());
        println!("({}) {:?} {:?} --> {}", i+1, rec.gears, rec.groups, ans);
        permutation_sums += ans;
    });
    println!("permuation_sums: {permutation_sums}");
}

fn rec_to_key(rec: &Record) -> (String, String) {
    return (rec.gears.iter().fold("".to_string(), |mut a, b| {a.push(*b); a}), 
        rec.groups.iter().fold("".to_string(), |mut a, b| {a.push_str(b.to_string().as_str()); a.push(','); a}));
}



fn split_permute(rec: Record) -> usize {
    if rec.groups.len() <= 2 {return permute(rec);}
    let n = rec.gears.len();
    let mid = rec.groups.len() / 2;
    let mid_len = rec.groups[mid];
    let left = rec.groups.iter().take(mid).map(|v| *v);
    let right = rec.groups.iter().skip(mid+1).map(|v| *v);
    let left_offset = left.clone().sum::<usize>() + left.clone().count();
    let right_offset = right.clone().sum::<usize>() + right.clone().count() + mid_len - 1;
    let mut permutations = 0usize;
    // println!("\nsplit_permute({}, {:?}) called.", rec.gears.iter().map(|f| *f).fold(String::new(), |mut a, b| {a.push(b); a}),
    //     rec.groups.iter().map(|v| *v).collect::<Vec<_>>());
    // println!("mid: {}, mid_len: {}, left: {:?}; right: {:?}", mid, mid_len, left.clone().collect::<Vec<usize>>(), 
    //     right.clone().collect::<Vec<usize>>());
    // println!("range: {}", rec.gears.)
    // println!("left_offset: {}; right_offset: {};\n", left_offset, right_offset);
    for offset in left_offset..(n - right_offset.min(n)) {
        // println!("rec.gears[offset - 1] = {}; middle: {:?}; rec.gears[offset+mid_len] = {};", rec.gears[offset - 1], rec.gears.iter().skip(offset).take(mid_len).collect::<Vec<_>>(), rec.gears[offset+mid_len]);
        if rec.gears[offset - 1] != '#' && rec.gears.iter().skip(offset).take(mid_len).all(|c| *c != '.') && rec.gears[offset+mid_len] != '#' {
            permutations += split_permute(Record { 
                gears: rec.gears.clone().into_iter().take(offset-1).collect(),
                groups: left.clone().collect() 
            }) * split_permute(Record {
                gears: rec.gears.clone().into_iter().skip(offset+mid_len+1).collect(),
                groups: right.clone().collect()
            });
        }
    }
    return permutations;
}





/*
 * Notice that if we eat the first '.' characters we will be left with a string starting with '#' or '?'.
 * 
 * Then, we have the following:
 * ===> "???#?.REST" indicates we can see the beginning sequence as independent, ie. 
 *      permute("???#?.REST", [x_1, x_2, x_3, ..., x_n]) = 
 *          + permute("???#?", []) * permute("REST", [x_1, ...])
 *          + permute("???#?", [x_1]) * permute("REST", [x_2, ...])
 *          + permute("???#?", [x_1, x_2]) * permute("REST", [x_3, ...])
 *          + permute("???#?", [x_1, x_2, x_3]) * permute("REST", [x_4, ...])
 *          + 0 * permute("REST", [x_2, ...])
 *          STOP!
*/

fn smart_permute(rec: Record, cache: &mut BTreeMap<(String, String), usize>) -> usize {
    let mut chars = rec.gears.clone();
    while chars.first().map_or(false, |c| *c == '.') {
        chars.remove(0);
    }
    if chars.len() == 0 {return if rec.groups.len() == 0 {1} else {0};}
    if chars.iter().filter(|c| **c != '.').count() < rec.groups.iter().sum::<usize>() {
        // println!("CHARS: {:?}, rec.groups: {:?}", chars.iter().fold("".to_string(), |mut a, b| {a.push(*b); a}), rec.groups);
        // println!("num: {}, sum: {}", chars.iter().take_while(|c| **c != '.').count(), rec.groups.iter().sum::<usize>());
        return 0;
    }

    let block_size = chars.iter().take_while(|c| **c != '.').count();

    let block = chars.iter().take(block_size).map(|f| *f).collect::<Vec<_>>();
    let rest = chars.iter().skip(block_size+1).map(|f| *f).collect::<Vec<_>>();

    if rest.len() == 0 {return dp_permute(Record {gears: block.clone(), groups: rec.groups}, cache)}

    let mut total = 0usize;
    let mut iteration = 0usize;
    let mut permute_block = permute(Record {
        gears: chars.iter().take(block_size).map(|f| *f).collect(),
        groups: rec.groups.iter().take(iteration).map(|v| *v).collect()
    });
    while rec.groups.iter().take(iteration).map(|v| *v).sum::<usize>() + iteration <= block_size + 1 {
        // println!("{} {:?} = {}", chars.iter().take(block_size).map(|f| *f).fold(String::new(), |mut a, b| {a.push(b); a}), rec.groups.iter().take(iteration).map(|v| *v).collect::<Vec<_>>(), permute_block);
        let v = if permute_block != 0 {smart_permute(Record {
            gears: rest.clone(),
            groups: rec.groups.iter().skip(iteration).map(|v| *v).collect()
        }, cache)} else {0};
        // println!("permute({}, {:?}) = {}", chars.iter().take(block_size).map(|f| *f).fold(String::new(), |mut a, b| {a.push(b); a}),
        //     rec.groups.iter().take(iteration).map(|v| *v).collect::<Vec<_>>(),
        //     permute_block);
        // println!("* smart_permute({}, {:?}) = {}", 
        //     chars.iter().skip(block_size+1).map(|f| *f).fold(String::new(), |mut a, b| {a.push(b); a}),
        //     rec.groups.iter().skip(iteration).map(|v| *v).collect::<Vec<_>>(), 
        //     v);
        total += permute_block * v;
        iteration += 1;
        permute_block = dp_permute(Record {
            gears: block.clone(),
            groups: rec.groups.iter().take(iteration).map(|v| *v).collect()
        }, cache);
    }
    return total;
}








// Despite this being faster than the usual permute, this is still TOO SLOW for this problem.
fn dp_permute(rec: Record, mut cache: &mut BTreeMap<(String, String), usize>) -> usize {
    let key = rec_to_key(&rec);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    // println!("Permute called on {:?}, {:?}", rec.gears, rec.groups);
    let mut chars = rec.gears.clone();
    while chars.first().map_or(false, |c| *c == '.') {
        chars.remove(0);
    }
    if chars.len() == 0 {
        return if rec.groups.len() == 0 {1} else {0};
    }
    if rec.groups.len() == 0 {
        return if chars.iter().all(|f| {*f != '#'}) {1} else {0};
    }
    let n_unknown = chars.iter().fold(0usize, |mut a,b| {a += if *b == '?' {1} else {0}; a});
    let n_damaged = chars.iter().fold(0usize, |mut a,b| {a += if *b == '#' {1} else {0}; a});
    let sum_of_groups: usize = rec.groups.iter().sum();

    if n_unknown + n_damaged < sum_of_groups {
        return 0;
    }

    // chars.first() is either '?' or '#'
    let mut new_groups = rec.groups.clone();
    let mut l = new_groups.remove(0);
    let mut block = (0..l).map(|i| {'#'}).collect::<Vec<char>>();

    if chars.len() <= block.len() { return if new_groups.len() == 0 && chars.len() == block.len() && chars.iter().all(|c| {*c != '.'}) {1} else {0} }
    
    let mut permutations = 0usize;
    // SLIDE!
    let mut splits = vec![];
    for (offset, record_iter) in (0..(chars.len() - block.len() + 1)).map(|i| { 
        (i + block.len(), chars.iter().skip(i).take(block.len()), chars.iter().take(i))
    }).filter(|p| {p.2.clone().all(|c| {*c != '#'})}).map(|p| {(p.0, p.1)}) {
        // println!("LOOP: {:?}", (offset, record_iter.clone().collect::<Vec<&char>>()));
        if record_iter.zip(block.iter()).all(|p| {
            if *p.1 == '#' {
                return *p.0 == '?' || *p.0 == '#';
            } else {
                unreachable!("THIS IS NOT POSSIBLE!!");
            }
        }) && (offset == chars.len() || chars[offset] == '.' || chars[offset] == '?') {
            let v = permute(Record {
                gears: chars.iter().skip(offset + 1).map(|c| {*c}).collect(),
                groups: new_groups.clone()
            });
            splits.push((chars.iter().skip(offset + 1).map(|c| {*c}).fold("".to_string(), |mut a,b| {a.push(b); a}),new_groups.clone(), v));
            // println!("--> {v}");
            permutations += v;
        }
    }

    // println!("{:?}, {:?} --> {}", rec.gears, rec.groups, permutations);
    // println!("splits: {splits:?}");
    cache.insert(key, permutations);
    return permutations;
}


fn permute(rec: Record) -> usize {
    // println!("Permute called on {:?}, {:?}", rec.gears, rec.groups);
    let mut chars = rec.gears.clone();
    while chars.first().map_or(false, |c| *c == '.') {
        chars.remove(0);
    }
    if chars.len() == 0 {
        return if rec.groups.len() == 0 {1} else {0};
    }
    if rec.groups.len() == 0 {
        return if chars.iter().all(|f| {*f != '#'}) {1} else {0};
    }
    let n_unknown = chars.iter().fold(0usize, |mut a,b| {a += if *b == '?' {1} else {0}; a});
    let n_damaged = chars.iter().fold(0usize, |mut a,b| {a += if *b == '#' {1} else {0}; a});
    let sum_of_groups: usize = rec.groups.iter().sum();

    if n_unknown + n_damaged < sum_of_groups {
        return 0;
    }

    // chars.first() is either '?' or '#'
    let mut new_groups = rec.groups.clone();
    let mut l = new_groups.remove(0);
    let mut block = (0..l).map(|i| {'#'}).collect::<Vec<char>>();

    if chars.len() <= block.len() { return if new_groups.len() == 0 && chars.len() == block.len() && chars.iter().all(|c| {*c != '.'}) {1} else {0} }
    
    let mut permutations = 0usize;
    // SLIDE!
    let mut splits = vec![];
    for (offset, record_iter) in (0..(chars.len() - block.len() + 1)).map(|i| { 
        (i + block.len(), chars.iter().skip(i).take(block.len()), chars.iter().take(i))
    }).filter(|p| {p.2.clone().all(|c| {*c != '#'})}).map(|p| {(p.0, p.1)}) {
        // println!("LOOP: {:?}", (offset, record_iter.clone().collect::<Vec<&char>>()));
        if record_iter.zip(block.iter()).all(|p| {
            if *p.1 == '#' {
                return *p.0 == '?' || *p.0 == '#';
            } else {
                unreachable!("THIS IS NOT POSSIBLE!!");
            }
        }) && (offset == chars.len() || chars[offset] == '.' || chars[offset] == '?') {
            let v = permute(Record {
                gears: chars.iter().skip(offset + 1).map(|c| {*c}).collect(),
                groups: new_groups.clone()
            });
            splits.push((chars.iter().skip(offset + 1).map(|c| {*c}).fold("".to_string(), |mut a,b| {a.push(b); a}),new_groups.clone(), v));
            // println!("--> {v}");
            permutations += v;
        }
    }

    // println!("{:?}, {:?} --> {}", rec.gears, rec.groups, permutations);
    // println!("splits: {splits:?}");
    return permutations;
}

fn brute_permute(rec: Record) -> usize {
    let mut chars = rec.gears.clone();
    let total = rec.groups.iter().fold(0, |a, b| {a+b});

    unimplemented!()
}