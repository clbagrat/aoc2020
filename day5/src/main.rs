use std::fs;
use std::cmp;
use itertools::Itertools;

#[derive(Debug)]
struct Seat {
    row: isize,
    col: isize
}

impl Seat {
    fn get_id(&self) -> isize {
        calc_seat_id(self.row, self.col)
    }
}

fn calc_seat_id (row: isize, col: isize) -> isize {
    row * 8 + col
}

fn get_val(route: &str) -> isize{
    let bin = route
        .replace("B", "1")
        .replace("F", "0")
        .replace("R", "1")
        .replace("L", "0");

    isize::from_str_radix(&bin, 2).unwrap()
}

fn get_seat(seat_code: &str) -> Seat {
    let row_code = &seat_code[..7];
    let col_code = &seat_code[7..];
    let row = get_val(row_code);
    let col = get_val(col_code);

    return Seat {
        row,
        col
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    println!("part1 {:?}",
        input
            .lines()
            .map(get_seat)
            .map(|seat| seat.get_id())
            .collect::<Vec<isize>>()
            .iter()
            .fold(0, |acc, &b| cmp::max(acc, b))
    );

    let mut a = input
        .lines()
        .map(get_seat)
        .filter(|seat| seat.row != 1)
        .collect::<Vec<Seat>>();

        a.sort_by_key(|seat| seat.row);

        for (row, group) in &a.iter().group_by(|seat| seat.row) {
            let sum = group.fold(0, |acc, seat| acc + seat.col);
            let target_sum = (0..=7).sum();
            if sum < target_sum {
                println!("part2 {}", calc_seat_id(row, target_sum - sum));
                break;
            }
        }
}
