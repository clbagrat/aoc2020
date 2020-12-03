use std::fs;

fn calc (lines: &Vec<&str>, h: usize, v: usize) -> usize {
    let x = lines[0].len();
    let y = lines.len();
    let mut last_hor_pos = h;
    let mut amount_of_trees = 0;

    for n in v..(y/v) {
        let space = lines[n*v].chars().nth(last_hor_pos).unwrap().to_string();
        if space == "#" {
            amount_of_trees += 1;
        }

        last_hor_pos = (last_hor_pos + h) % x;
    }

    amount_of_trees
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");
    let lines = input.lines().collect::<Vec<&str>>();


    println!("part1 {}", calc(&lines, 3, 1));
    println!("part2 {}",
        calc(&lines, 1, 1) *
        calc(&lines, 3, 1) *
        calc(&lines, 5, 1) *
        calc(&lines, 7, 1) *
        calc(&lines, 1, 2)
    )
}
