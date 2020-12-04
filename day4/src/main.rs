use std::fs;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use regex::Regex;


fn to_hash_map(pass: &str) -> HashMap<&str, &str> {
    pass.split_whitespace().flat_map(|s| {
        let mut parts = s.split(":");

        Some((parts.next().unwrap(), parts.next().unwrap()))
    }).collect::<HashMap<&str, &str>>()
}

fn is_require_valid(pass_data: &HashMap<&str, &str>, required_keys: &[&str]) -> bool {
    required_keys.iter().all(|d| pass_data.contains_key(d))
}

fn is_key_in_range(pass_data: &HashMap<&str, &str>, key: &str, range: RangeInclusive<usize>) -> bool {
    let number = pass_data.get(key).and_then(|s| s.parse::<usize>().ok()).unwrap();
    range.contains(&number)
}

fn is_height_correct(pass_data: &HashMap<&str, &str>, key: &str, cm_range: RangeInclusive<usize>, in_range: RangeInclusive<usize>) -> bool {
    let value = pass_data.get(key).unwrap();
    let number = &value[..(value.len()-2)].parse::<usize>().ok().unwrap();

    match &value[value.len()-2..] {
        "cm" => cm_range.contains(number),
        "in" => in_range.contains(number),
        _ => false
    }
}

fn is_regex_correct(pass_data: &HashMap<&str, &str>, key: &str, re_str: &str) -> bool {
    let re = Regex::new(re_str).unwrap();
    let value = pass_data.get(key).unwrap();
    re.is_match(value)
}

fn is_value_in_list(pass_data: &HashMap<&str, &str>, key: &str, list: &[&str]) -> bool {
    list.contains(pass_data.get(key).unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    println!("part1 {:?}",
        &input
        .split("\n\n")
        .filter(|line| line.len() > 0)
        .map(|pass_line| to_hash_map(pass_line))
        .filter(|pass_data| is_require_valid(pass_data, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]))
        .count()
    );

    println!("part2 {:?}",
        &input
        .split("\n\n")
        .filter(|line| line.len() > 0)
        .map(|pass_line| to_hash_map(pass_line))
        .filter(|pass_data| is_require_valid(pass_data, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]))
        .filter(|pass_data| is_key_in_range(pass_data, "byr", 1920..=2002))
        .filter(|pass_data| is_key_in_range(pass_data, "iyr", 2010..=2020))
        .filter(|pass_data| is_key_in_range(pass_data, "eyr", 2020..=2030))
        .filter(|pass_data| is_height_correct(pass_data, "hgt", 150..=193, 59..=76))
        .filter(|pass_data| is_regex_correct(pass_data, "hcl", r"^#([a-f0-9]{6})$"))
        .filter(|pass_data| is_regex_correct(pass_data, "pid", r"^([0-9]{9})$"))
        .filter(|pass_data| is_value_in_list(pass_data, "ecl", &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]))
        .count()
    );

}
