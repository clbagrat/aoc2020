use std::fs;

const ROW_LENGTH: usize = 91;
const COL_LENGTH: usize = 95;

fn get_seat_index (row: usize, col: usize) -> usize {
    row * ROW_LENGTH + col
}

fn get_neighbours (index: usize, input: &Vec<char>) -> [char;8] {
    let col = index % ROW_LENGTH;
    let row = index / ROW_LENGTH;
    // let input:Vec<char> = all_seats.chars().collect();
    [
        //topleft
        if col == 0 || row == 0 {'.'} else {input[get_seat_index(row - 1, col - 1)]},
        //topcenter
        if row == 0 {'.'} else {input[get_seat_index(row - 1, col)]},
        //topright
        if col == ROW_LENGTH - 1 || row == 0 {'.'} else {input[get_seat_index(row - 1, col+1)]},
        //left
        if col == 0 {'.'} else {input[get_seat_index(row, col-1)]},
        //right
        if col == ROW_LENGTH - 1 {'.'} else{input[get_seat_index(row, col+1)]},
        //bottomleft
        if col == 0 || row == COL_LENGTH - 1 {'.'} else {input[get_seat_index(row + 1, col - 1)]},
        //bottomcenter
        if row == COL_LENGTH - 1 {'.'} else {input[get_seat_index(row + 1, col)]},
        //bottomright
        if col == ROW_LENGTH - 1 || row == COL_LENGTH - 1 {'.'} else {input[get_seat_index(row + 1, col+1)]},
    ]
}

fn raycast (index:usize, dir: (isize, isize), all_seats: &Vec<char>) -> char {
    let mut col = (index as isize) % (ROW_LENGTH as isize);
    let mut row = index as isize / ROW_LENGTH as isize;

    loop {
        col += dir.0;
        row += dir.1;
        if !(0..ROW_LENGTH).contains(&(col as usize)) {return '.'}
        if !(0..COL_LENGTH).contains(&(row as usize)) {return '.'}

        let ch = all_seats[get_seat_index(row as usize, col as usize)];

        if ch != '.' {return ch}
    }
;
}

fn get_visible (index: usize, all_seats: &Vec<char>) -> [char;8] {

    [
        raycast(index, (-1, -1), all_seats),
        raycast(index, (0, -1), all_seats),
        raycast(index, (1, -1), all_seats),
        raycast(index, (-1, 0), all_seats),
        raycast(index, (1, 0), all_seats),
        raycast(index, (-1, 1), all_seats),
        raycast(index, (0, 1), all_seats),
        raycast(index, (1, 1), all_seats),
    ]
}

fn check_if_taken (neighbours: &[char; 8]) -> bool {
    !neighbours.contains(&'#')
}

fn check_if_free (neighbours: &[char; 8], num: usize) -> bool {
    neighbours.to_vec().iter().filter(|x| **x == '#').count() >= num
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");
    let mut last_input = input.replace("\n", "").chars().collect::<Vec<char>>();
    loop {
        let new_input = last_input.iter().enumerate().map(|(i, x)| {
            let neighbours = get_neighbours(i, &last_input);
            match x {
                'L' => if check_if_taken(&neighbours) {'#'} else {'L'},
                '#' => if check_if_free(&neighbours, 4) {'L'} else {'#'},
                _ => *x
            }
        }).collect();

        if new_input == last_input {
            break;
        }
        else {
            last_input = new_input;
        }
    }

    println!("part1 {}", last_input.iter().filter(|x| **x == '#').count());

    /////////////////////////////////////////////////////////////////////////////

    let mut last_input = input.replace("\n", "").chars().collect::<Vec<char>>();
    loop {
        let new_input = last_input.iter().enumerate().map(|(i, x)| {
            let neighbours = get_visible(i, &last_input);
            match x {
                'L' => if check_if_taken(&neighbours) {'#'} else {'L'},
                '#' => if check_if_free(&neighbours, 5) {'L'} else {'#'},
                _ => *x
            }
        }).collect();

        if new_input == last_input {
            break;
        }
        else {
            last_input = new_input;
        }
    }

    println!("part2 {}", last_input.iter().filter(|x| **x == '#').count());
}
