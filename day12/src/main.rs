use std::fs;
use std::collections::HashMap;


fn main() {
    let directions: HashMap<&str, (i32,i32)> =
        [
            ("E", (1, 0)),
            ("N", (0, 1)),
            ("W", (-1, 0)),
            ("S", (0, -1))
        ].iter().cloned().collect();

    let input = fs::read_to_string("input.txt").unwrap();

    let part1 = input
        .lines()
        .map(|line| (&line[0..1], line[1..].parse::<i32>().unwrap()))
        .fold(((0, 0), ["E", "N", "W", "S"]), |mut acc, (command, value)| {

            acc.0 = if !["L", "R"].contains(&command) {
                let dir_vector = if directions.contains_key(&command) {
                    directions.get(&command).unwrap()
                } else {
                    directions.get(&acc.1[0]).unwrap()
                };

                (acc.0.0 + dir_vector.0 * value, acc.0.1 + dir_vector.1 * value)
            } else {acc.0};

            acc.1 = if ["L", "R"].contains(&command) {
                if command == "R" {
                    acc.1.rotate_right((value as usize) / 90);
                } else {
                    acc.1.rotate_left((value as usize) / 90);
                }
                acc.1
            } else {acc.1};

            acc
        });
    println!("part 1 {:?}", part1.0.0.abs() + part1.0.1.abs());

    let part2 = input
        .lines()
        .map(|line| (&line[0..1], line[1..].parse::<i32>().unwrap()))
        .fold(((0, 0), (10, 1)), |mut acc, (command, value)| {
            if ["L", "R"].contains(&command) {
                let pi = std::f64::consts::PI;
                let angle = (value as f64) * (pi/180.0);
                let x = acc.1.0 as f64;
                let y = acc.1.1 as f64;
                if command == "R" {
                    acc.1.0 = (angle.cos() * x + angle.sin() * y).round() as i32;
                    acc.1.1 = (-angle.sin() * x + angle.cos() * y).round() as i32;
                } else {
                    acc.1.0 = (angle.cos() * x - angle.sin() * y).round() as i32;
                    acc.1.1 = (angle.sin() * x + angle.cos() * y).round() as i32;
                }
            }

            if command == "F" {
                acc.0.0 += acc.1.0 * value;
                acc.0.1 += acc.1.1 * value;
            }

            if ["N", "W", "S", "E"].contains(&command) {
                let dir_vector = directions.get(&command).unwrap();
                acc.1.0 += dir_vector.0 * value;
                acc.1.1 += dir_vector.1 * value;
            }
            acc
        });
    println!("part 2 {:?}", part2.0.0.abs() + part2.0.1.abs());
}
