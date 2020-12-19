use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn unravel_regex(start:String, rule_map: &HashMap<&str, Vec<&str>>) -> String {
    let refs = rule_map.get(&start[..]).unwrap();
    let mut res = vec![];
    for r in refs.iter() {
        // println!("{}", r);
        match *r {
            "\"a\"" => {res.push(String::from("a"))},
            "\"b\"" => {res.push(String::from("b"))},
            "|" => {res.push(String::from("|"))},
            //////
            ////// uncomment for part2
            //////
            // "8" => {res.push(format!("({}+)", unravel_regex(String::from("42"), rule_map)))},
            // "11" => {
            //     let r1 = unravel_regex(String::from("42"), rule_map);
            //     let r2 = unravel_regex(String::from("31"), rule_map);
            //     let mut res_11 = vec![];
            //     for i in 1..=5 {
            //         res_11.push(format!("{x}{{{n}}}{y}{{{n}}}", x= r1, y=r2, n=i));
            //     }
            //     res.push(format!("({})", res_11.join("|")));
            // },
            _ => {res.push(unravel_regex(String::from(*r), rule_map))}
        }
    }
    if refs.contains(&"|") {
        format!("({})", res.join(""))
    } else {
        res.join("")
    }

}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut split = input.split("\n\n").into_iter();
    let rules = split.next().unwrap();
    let messages = split.next().unwrap();

    let mut rule_map = HashMap::new();
    for line in rules.lines() {
        let mut rule_split = line.split(": ");
        let rule_id = rule_split.next().unwrap();
        let rule_references = rule_split.next().unwrap().split(" ").collect::<Vec<&str>>();
        rule_map.insert(rule_id, rule_references);
    }

    let regex_line = format!(r"^{}$", unravel_regex(String::from("0"), &rule_map));

    let re = Regex::new(&regex_line).unwrap();


    println!("part {:#?}", messages.lines().filter(|l| re.is_match(l)).count());
}
