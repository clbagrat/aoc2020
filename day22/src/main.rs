
use std::collections::VecDeque;

// const PLAYER_1: [usize; 2] = [43, 19];
// const PLAYER_2: [usize; 3] = [2, 29, 14];

// const PLAYER_1: [usize; 5] = [9, 2, 6, 3, 1];
// const PLAYER_2: [usize; 5] = [5, 8, 4, 7, 10];

const PLAYER_1: [usize; 25] = [26, 14, 6, 34, 37, 9, 17, 39, 4, 5, 1, 8, 49, 16, 18, 47, 20, 31, 23, 19, 35, 41, 28, 15, 44];
const PLAYER_2: [usize; 25] = [7, 2, 10, 25, 29, 46, 40, 45, 11, 50, 42, 24, 38, 13, 36, 22, 33, 3, 43, 21, 48, 30, 32, 12, 27];

fn turn(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>, mut log: Vec<String>) -> (VecDeque<usize>, VecDeque<usize>, i8) {
    let log_entry = format!(
        "p1:({}) p2:({})",
        p1.iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join(","),
        p2.iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join(",")
    );
    if log.contains(&log_entry) {
        println!("LOG");
        return (p1, p2, 1)
    }
    log.push(log_entry);
    println!("ROUND {}", log.len());
    println!("Player 1{:?}", p1);
    println!("Player 2{:?}", p2);

    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    let winner: i8;

    if c1 <= p1.len() && c2 <= p2.len() {
        let mut new_p1 = p1.clone();
        let mut new_p2 = p2.clone();
        if new_p1.len() > c1 {
            new_p1.resize(c1, 0);
        }
        if new_p2.len() > c2 {
            new_p2.resize(c2, 0);
        }
        println!("SUB_GAME!");
        winner = turn(new_p1, new_p2, vec![]).2;
        println!("SUB_GAME WINNER {}", winner);
    } else {
        winner = if c1 > c2  {1} else {2};
    }

    println!("ROUND WINNER {}", winner);
    if winner == 1 {
        p1.push_back(c1);
        p1.push_back(c2);
    } else {
        p2.push_back(c2);
        p2.push_back(c1);
    }

    if p1.is_empty() || p2.is_empty() {
        let sub_game_winner = if p1.is_empty() {2} else {1};
        return (p1, p2, sub_game_winner);
    } else {
        return turn(p1, p2, log);
    }
}

fn main() {
    let (p1, p2, _) = turn(
        PLAYER_1.iter().map(|i| *i).collect::<VecDeque<usize>>(),
        PLAYER_2.iter().map(|i| *i).collect::<VecDeque<usize>>(),
        vec![]
    );
    let winner = if p2.is_empty() {p1} else {p2};

    println!("Part1 {:?}", winner.iter().rev().enumerate().map(|(i, num)| (i + 1) as usize * num).fold(0, |acc, cur| acc+cur));
}
