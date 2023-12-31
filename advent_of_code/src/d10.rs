use std::fs;
use std::collections::BTreeSet;
use std::cmp::{min, max};

pub fn solve() {
    let path = "resources/d10.txt";
    let contents = fs::read_to_string(path).unwrap();
    let mut maze = Vec::<Vec<char>>::new();
    for line in contents.lines() {
        maze.push(line.chars().collect());
    }
    let n = maze.len(); let m = maze.first().unwrap().len();
    println!("{maze:?}");
    let (mut srow, mut scol) = (0, 0);
    for (row, line) in maze.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'S' {
                println!("{row}, {col}");
                srow = row;
                scol = col;
            }
        }
    }
    let mut distance_values = maze.iter().map(|vec| {
        vec.iter().map(|c| {0usize}).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    let mut q: Vec<(usize, usize)> = vec![(srow, scol)];
    let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new(); seen.insert((srow, scol));
    let mut farthest_point: (usize, usize) = (0, 0);
    while q.len() != 0 {
        let nxt = *q.first().unwrap(); 
        q.remove(0);
        let mut p = (nxt.0.max(1) - 1, nxt.1);
        let mut c = maze[p.0][p.1];
        let cur_c = maze[nxt.0][nxt.1];
        if (has_south_connection(c)) && (has_north_connection(cur_c)) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("1. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_south_connection(c)) && (has_north_connection(cur_c)) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0, nxt.1.max(1) - 1);
        c = maze[p.0][p.1];
        if (has_east_connection(c)) && (has_west_connection(cur_c)) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("2. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_east_connection(c)) && (has_west_connection(cur_c))  && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0.min(n - 2) + 1, nxt.1);
        c = maze[p.0][p.1];
        if (has_north_connection(c)) && has_south_connection(cur_c) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("3. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_north_connection(c)) && has_south_connection(cur_c) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0, nxt.1.min(m - 2) + 1);
        c = maze[p.0][p.1];
        if (has_west_connection(c)) && has_east_connection(cur_c) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("4. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_west_connection(c)) && has_east_connection(cur_c) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }
    }
    distance_values.iter().for_each(|v| {
        v.iter().for_each(|c| {
            print!("{c: >5}");
        });
        println!();
    });

    distance_values.iter().for_each(|v| {
        v.iter().for_each(|c| {
            print!("{c: >5}");
        });
        println!();
    });

    // now we calculate the boundary for the loop
    let mut boundary: Vec<Vec<bool>> = maze.iter().map(|f| {f.iter().map(|c| {false}).collect()}).collect();
    q.push(farthest_point);
    boundary[farthest_point.0][farthest_point.1] = true;
    while q.len() != 0 {
        let nxt = q.pop().unwrap();
        let pos = vec![(nxt.0.max(1) - 1, nxt.1), (nxt.0, nxt.1.max(1) - 1),
            (nxt.0.min(n - 2) + 1, nxt.1), (nxt.0, nxt.1.min(m - 2) + 1)];
        for p in pos {
            if distance_values[nxt.0][nxt.1] == distance_values[p.0][p.1] + 1 {
                boundary[p.0][p.1] = true;
                q.push(p);
            }
        }
    }

    /*
        J, L, F, 7, |, -
        TOP_LEFT_RULES: 
            - if L, then ---- color = !color[right]
            - if F, then ---- color = !color[bot-right]
            - if J, then ---- color = !color[bot-right]
            - if 7, then ---- color = !color[bot]
            - if |, then ---- color = color[bot]
            - if -, then ---- color = color[right]
     */

    // now we mark interior by using top-left coloring rules.
    let mut interior: Vec<Vec<bool>> = maze.iter().map(|f| {f.iter().map(|c| {false}).collect()}).collect();
    let start_replacement = match (distance_values[srow-1][scol] == 1, distance_values[srow][scol + 1] == 1, distance_values[srow+1][scol] == 1, distance_values[srow][scol-1] == 1) {
        (true, true, false, false) => 'L',
        (true, false, true, false) => '|',
        (true, false, false, true) => 'J',
        (false, true, true, false) => 'F',
        (false, true, false, true) => '-',
        (false, false, true, true) => '7',
        _ => panic!("Uh oh, I messed up the logic here!")
    };
    for (row, vec) in boundary.iter().enumerate().rev() {
        for (col, b) in vec.iter().enumerate().rev() {
            if row == n - 1 || col == m - 1 {continue;}
            if boundary[row][col] {
                let mut c = maze[row][col];
                if c == 'S' {c = start_replacement;}
                interior[row][col] = match c {
                    'L' => !interior[row][col+1],
                    'F' => !interior[row+1][col+1],
                    'J' => !interior[row+1][col+1],
                    '7' => !interior[row+1][col],
                    '|' => interior[row+1][col],
                    '-' => interior[row][col+1],
                    _ => panic!()
                };
            } else {
                interior[row][col] = interior[row+1][col];
            }
        }
    }

    // remove all boundary values
    for (row, vec) in boundary.iter().enumerate() {
        for (col, b) in vec.iter().enumerate() {
            if *b {
                interior[row][col] = false;
            }
        }
    }

    println!("Boundary:");
    boundary.iter().enumerate().for_each(|v| {
        v.1.iter().enumerate().for_each(|c| {
            if *c.1 {print!("{}", maze[v.0][c.0]);} else {print!(" ");}
        });
        println!();
    });

    println!("Interior:");
    interior.iter().for_each(|v| {
        v.iter().for_each(|c| {
            if *c {print!("I");} else {print!(" ");}
        });
        println!();
    });

    let mut total_interior = 0usize;
    interior.iter().for_each(|v| {v.iter().for_each(|b| {if *b {total_interior += 1;}})});
    println!("total interior: {total_interior}");
}

pub fn solvev1() {
    let contents = fs::read_to_string("resources/d10.txt").unwrap();
    let mut maze = Vec::<Vec<char>>::new();
    for line in contents.lines() {
        maze.push(line.chars().collect());
    }
    let n = maze.len(); let m = maze.first().unwrap().len();
    println!("{maze:?}");
    let (mut srow, mut scol) = (0, 0);
    for (row, line) in maze.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == 'S' {
                println!("{row}, {col}");
                srow = row;
                scol = col;
            }
        }
    }
    let mut distance_values = maze.iter().map(|vec| {
        vec.iter().map(|c| {0usize}).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();
    let mut q: Vec<(usize, usize)> = vec![(srow, scol)];
    let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new(); seen.insert((srow, scol));
    let mut farthest_point: (usize, usize) = (0, 0);
    while q.len() != 0 {
        let nxt = *q.first().unwrap(); 
        q.remove(0);
        let mut p = (nxt.0.max(1) - 1, nxt.1);
        let mut c = maze[p.0][p.1];
        let cur_c = maze[nxt.0][nxt.1];
        if (has_south_connection(c)) && (has_north_connection(cur_c)) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("1. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_south_connection(c)) && (has_north_connection(cur_c)) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0, nxt.1.max(1) - 1);
        c = maze[p.0][p.1];
        if (has_east_connection(c)) && (has_west_connection(cur_c)) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("2. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_east_connection(c)) && (has_west_connection(cur_c))  && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0.min(n - 2) + 1, nxt.1);
        c = maze[p.0][p.1];
        if (has_north_connection(c)) && has_south_connection(cur_c) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("3. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_north_connection(c)) && has_south_connection(cur_c) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }

        p = (nxt.0, nxt.1.min(m - 2) + 1);
        c = maze[p.0][p.1];
        if (has_west_connection(c)) && has_east_connection(cur_c) && seen.contains(&p) && distance_values[p.0][p.1] == distance_values[nxt.0][nxt.1] + 1 {
            println!("4. Loop found at {p:?} with distance value = {}", distance_values[p.0][p.1]);
            farthest_point = p;
        }
        if (has_west_connection(c)) && has_east_connection(cur_c) && !seen.contains(&p) {
            distance_values[p.0][p.1] = distance_values[nxt.0][nxt.1] + 1;
            seen.insert(p);
            q.push(p);
        }
    }
    distance_values.iter().for_each(|v| {
        v.iter().for_each(|c| {
            print!("{c: >5}");
        });
        println!();
    });
}

pub fn has_north_connection(c: char) -> bool {
    return (c == '|' || c == 'L' || c == 'J' || c == 'S');
}

pub fn has_east_connection(c: char) -> bool {
    return (c == '-' || c == 'L' || c == 'F' || c == 'S');
}

pub fn has_south_connection(c: char) -> bool {
    return (c == '|' || c == 'F' || c == '7' || c == 'S');
}

pub fn has_west_connection(c: char) -> bool {
    return (c == '-' || c == 'J' || c == '7' || c == 'S');
}