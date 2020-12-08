use std::fs;
use std::collections::HashSet;

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

fn run_program (instructions: &Vec<(&str, i32)>) -> Option<(bool, i32)>{
    let mut current_index = 0;
    let mut current_acc = 0;
    let mut visited_indexes = HashSet::new();

    loop {
        if visited_indexes.contains(&current_index) {
            return Some((false, current_acc));
        }

        if instructions.len() <= current_index {
            return Some((true, current_acc));
        }

        visited_indexes.insert(current_index);

        let (operation, value) = &instructions[current_index];
        if operation == &"acc" {
            current_acc += value;
        }

        if operation == &"jmp" {
            current_index = add(current_index, i32::from(*value));
        } else {
            current_index += 1;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    let instructions = input.lines()
        .fold(vec![], |mut acc, line| {
            let mut split = line.split(" ");
            let op = split.next().unwrap();
            let val = split.next().unwrap().parse::<i32>().unwrap();
            acc.push((op, val));
            acc
        });

    println!("part1 acc {:?}", run_program(&instructions));

    for i in 0..instructions.len() {
        let mut copy = instructions.to_vec();
        let (op, val) = instructions[i];
        if op == "nop" {
            copy[i] = ("jmp", val);
        } else if op == "jmp" {
            copy[i] = ("nop", val);
        }

        let (is_terminated, last_value) = run_program(&copy).unwrap();

        if is_terminated {
            println!("part2 acc {:?}", last_value)
        }
    }

}
