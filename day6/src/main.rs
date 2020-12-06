use std::fs;
use std::str::Lines;
use std::collections::HashSet;

fn calc_union_count(lines: Lines) -> usize {
    lines
        .map(|line| line.split("").collect::<HashSet<&str>>())
        .fold(HashSet::new(), {
            |acc, cur| acc.union(&cur).copied().collect::<HashSet<&str>>()
        })
        .into_iter()
        .filter(|a| a != &"")
        .collect::<Vec<&str>>()
        .len()
}

fn calc_intersection_count(lines: Lines) -> usize {
    lines
        .map(|line| line.split("").collect::<HashSet<&str>>())
        .fold(HashSet::new(), {
            |acc, cur| {
                if acc.is_empty() {
                    return cur
                } else {
                    return acc.intersection(&cur).copied().collect::<HashSet<&str>>()
                }
            }
        })
        .into_iter()
        .filter(|a| a != &"")
        .collect::<Vec<&str>>()
        .len()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    println!("part1 {:?}",
        &input
        .split("\n\n")
        .map(|group| group.lines())
        .map(|lines| calc_union_count(lines))
        .sum::<usize>()
    );

    println!("part2 {:?}",
        &input
        .split("\n\n")
        .map(|group| group.lines())
        .map(|lines| calc_intersection_count(lines))
        .sum::<usize>()
    );

}
