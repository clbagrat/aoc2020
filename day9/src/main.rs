use std::fs;
use itertools::Itertools;

fn check(target: &usize, preamble: &[usize]) -> bool {
    let f_option = preamble.iter()
        .tuple_combinations()
        .find(|&(a,b)| a + b == *target);


    f_option != None
}

fn get_sum_tuple(target: &usize, set: &[usize]) -> Option<(usize, usize)> {
    let mut cur = 0;
    let mut a = vec![];
    for n in set {
        cur += n;
        a.push(n);
        if cur == *target {
            a.sort();
            println!("{:?}", a);
            return Some((**a.first().unwrap(), **a.last().unwrap()))
        }
        if cur > *target {
            return None
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    let numbers:Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();

    let mut broken_index = 0;
    let mut broken_number = 0;
    for i in 25..numbers.len() {
        if !check(&numbers[i], &numbers[i-25..i]) {
            broken_index = i;
            broken_number = *&numbers[i];
            break;
        }
    }

    println!("part1 index:{} broken number: {:?}", broken_index, broken_number);

    for i in 0..numbers.len() {
        let tuple = get_sum_tuple(&broken_number, &numbers[i..numbers.len()]);
        if tuple != None {
            println!("part2 {:?}", tuple.unwrap().0 + tuple.unwrap().1);
            break;
        }
    }


}
