use std::fs;
use regex::Regex;

struct PassEntry {
    left: usize,
    right: usize,
    character: String,
    password: String,
}

fn parse_to_pass_entry(line: &str) -> PassEntry {
    let split = line.split(": ").collect::<Vec<&str>>();

    let re = Regex::new(r"(?P<left>\d+)-(?P<right>\d+) (?P<char>[a-z])").unwrap();
    let caps = re.captures(split[0]).unwrap();
    let character = caps.name("char").map_or("", |m| m.as_str()).to_string();
    let left:usize = caps.name("left").map_or("", |m| m.as_str()).parse().unwrap();
    let right:usize = caps.name("right").map_or("", |m| m.as_str()).parse().unwrap();

    return PassEntry {
        left,
        right,
        character,
        password: split[1].to_string()
    }
}

fn assert_character_amount(entry: &PassEntry) -> bool {
    let amount_of_matches:usize = entry.password.matches(&entry.character).into_iter().count();
    (entry.left..=entry.right).contains(&amount_of_matches)
}

fn assert_char(string: &String, character: &String, position: usize) -> bool {
    string.chars().nth(position - 1).unwrap() == character.chars().next().unwrap()
}

fn assect_character_position(entry: &PassEntry) -> bool {
    let has_left = assert_char(&entry.password, &entry.character, entry.left);
    let has_right = assert_char(&entry.password, &entry.character, entry.right);

    return has_right ^ has_left;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    let part1 = input.lines()
        .map(|line| parse_to_pass_entry(line))
        .filter(|pass_entry| assert_character_amount(pass_entry))
        .count();

    let part2 = input.lines()
        .map(|line| parse_to_pass_entry(line))
        .filter(|pass_entry| assect_character_position(pass_entry))
        .count();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
