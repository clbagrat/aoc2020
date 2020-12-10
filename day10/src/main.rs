use std::fs;
use std::collections::HashMap;


fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    let mut numbers:Vec<usize> = input.lines().map(|line| line.parse::<usize>().unwrap()).collect();
    numbers.push(0);
    numbers.sort();
    let mut last = 0;
    let mut groups = HashMap::new();
    groups.insert(0, vec![]);
    groups.insert(1, vec![]);
    groups.insert(2, vec![]);
    groups.insert(3, vec![]);
    for n in numbers.iter() {
        groups.get_mut(&(n-last)).unwrap().push(n);
        last = *n;
    }
    println!("part1 {:#?}", groups.get(&1).unwrap().len() * (groups.get(&3).unwrap().len() + 1));



    let mut groups_of_usages:HashMap<&usize, usize> = HashMap::new();

    for n in 0..numbers.len() {
        let number = &numbers[n];
        let mut res = 0;

        if n == 0 {
            res = 1;
        }

        if n > 0 {
            let a = numbers[n-1];

            if number - a <= 3 {
                res += groups_of_usages.get(&a).unwrap();
            }
        }

        if n > 1 {
            let a = numbers[n-2];

            if number - a <= 3 {
                res += groups_of_usages.get(&a).unwrap();
            }
        }

        if n > 2 {
            let a = numbers[n-3];

            if number - a <= 3 {
                res += groups_of_usages.get(&a).unwrap();
            }
        }

        groups_of_usages.insert(number, res);
    }


    println!("part2 {:#?}", groups_of_usages.get(&numbers[numbers.len()-1]));

}
