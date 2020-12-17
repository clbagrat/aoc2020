use std::fs;
use std::collections::HashSet;

#[macro_use] extern crate itertools;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();


    let mut space = HashSet::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let chars = line.chars().collect::<Vec<char>>();
        for c in 0..chars.len() {
            if chars[c] == '#' {
                space.insert((c as i32, i as i32, 0 as i32));
            }
        }
    }

    for _ in 0..6 {
        let mut new_space = HashSet::new();
        for (x, y, z) in &space {
            for (d_x, d_y, d_z) in iproduct!(-1..=1, -1..=1, -1..=1) {
                let cur = (x+d_x, y+d_y, z+d_z);
                let is_active = space.contains(&cur);
                let active_neighbours = iproduct!(cur.0 - 1..=cur.0 + 1, cur.1 - 1..=cur.1 + 1, cur.2 - 1..=cur.2 + 1).filter(|coord| {
                    coord != &cur && space.contains(coord)
                }).count();

                if is_active && [2,3].contains(&active_neighbours) {
                    new_space.insert(cur);
                }
                if !is_active && active_neighbours == 3 {
                    new_space.insert(cur);
                }
            }
        }
        space = new_space;
    }
    println!("part1 {:#?}", space.len());

    ////////////////////////////////////////

    let mut space = HashSet::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let chars = line.chars().collect::<Vec<char>>();
        for c in 0..chars.len() {
            if chars[c] == '#' {
                space.insert((c as i32, i as i32, 0 as i32, 0 as i32));
            }
        }
    }

    for _ in 0..6 {
        let mut new_space = HashSet::new();
        for (x, y, z, w) in &space {
            for (d_x, d_y, d_z, d_w) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
                let cur = (x+d_x, y+d_y, z+d_z, w+d_w);
                let is_active = space.contains(&cur);
                let active_neighbours = iproduct!(
                    cur.0 - 1..=cur.0 + 1,
                    cur.1 - 1..=cur.1 + 1,
                    cur.2 - 1..=cur.2 + 1,
                    cur.3 - 1..=cur.3 + 1
                ).filter(|coord| {
                    coord != &cur && space.contains(coord)
                }).count();

                if is_active && [2,3].contains(&active_neighbours) {
                    new_space.insert(cur);
                }
                if !is_active && active_neighbours == 3 {
                    new_space.insert(cur);
                }
            }
        }
        space = new_space;
    }
    println!("part2 {:#?}", space.len());
}
