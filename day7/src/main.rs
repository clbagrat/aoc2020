use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


fn calc_price(hash: &HashMap<&str, Vec<(String, String)>>, color: &str) -> usize {
    let mut price = 0;
    if !hash.contains_key(color) {
        return 0;
    }

    for (amount, c) in hash.get(color).unwrap() {
        let amount_num: usize = amount.parse().unwrap();
        price += amount_num;
        price += amount_num * calc_price(hash, c);
    }

    return price;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail");

    let mut bottom_to_top = HashMap::new();
    let mut top_to_bottom = HashMap::new();

    for l in input.lines().into_iter() {
        let mut parsed = l.split("bags contain");
        let parent_color = parsed.next().unwrap().trim();
        let children_colors = parsed.next().unwrap().trim();
        let re = Regex::new(r"(\d) (.+?) bags?[,.]").unwrap();

        for c in re.captures_iter(children_colors) {
            let amount = String::from(&c[1]);
            let color = String::from(&c[2]);
            if !bottom_to_top.contains_key(&color) {
                bottom_to_top.insert(color, vec![(amount, parent_color)]);
            } else {
                bottom_to_top.get_mut(&color).and_then(|v| Some(v.push((amount, parent_color))));
            }

            let amount = String::from(&c[1]);
            let color = String::from(&c[2]);

            if !top_to_bottom.contains_key(parent_color) {
                top_to_bottom.insert(parent_color, vec![(amount, color)]);
            } else {
                top_to_bottom.get_mut(parent_color).unwrap().push((amount, color));
            }
        }
    }

    let mut colors_met = HashSet::new();
    let mut colors_to_check = vec!["shiny gold"];


    loop {
        let target_color = colors_to_check.pop().unwrap();
        for (key, value) in bottom_to_top.iter() {
            if key == target_color {
                for (_amount, met) in value {
                    colors_met.insert(met);
                    colors_to_check.push(met);
                }
            }
        }

        if colors_to_check.len() == 0 {
            break;
        }
    }

    println!("part 1, dumb loop {}", colors_met.len());

    println!("part 2, recursive {:#?}", calc_price(&mut top_to_bottom, "shiny gold"));
}
