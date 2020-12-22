use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines: Vec<(HashSet<String>, HashSet<String>)> = input.lines().map(|line| {
        let line = line.replace(")", "");
        let mut split = line.split(" (contains ").clone();
        let ingridients = split.next();
        let alergens = split.next();

        (
            ingridients.unwrap().split(" ").map(|i| String::from(i)).collect::<HashSet<String>>(),
            alergens.unwrap().split(", ").map(|a| String::from(a)).collect::<HashSet<String>>()
        )
    }).collect();



    let mut allergens_in_ingridients = HashMap::new();
    let mut all_ingridients = vec![];
    for (ingridients, alergens) in lines {
        for al in alergens {
            if !allergens_in_ingridients.contains_key(&al) {
                allergens_in_ingridients.insert(al, ingridients.clone());
            } else {
                allergens_in_ingridients.insert(al.clone(), allergens_in_ingridients.get(&al).unwrap().intersection(&ingridients.clone()).map(|i| String::from(i)).collect());
            }
        }
        for i in ingridients.clone() {
            all_ingridients.push(i);
        }
    }

    let ingridients_with_alergens = allergens_in_ingridients.values().fold(HashSet::new(), |acc, cur| {
        cur.union(&acc.clone()).map(|i| String::from(i)).collect()
    });


    let ingridients_without_alergens = all_ingridients.iter().filter(|i| !ingridients_with_alergens.contains(*i)).map(|i| String::from(i)).collect::<HashSet<String>>();

    println!("part1 {:?}", ingridients_without_alergens.iter().fold(0, |acc, cur| {
        acc + all_ingridients.iter().filter(|i| i == &cur).count()
    }));


    let mut vec_alergens = allergens_in_ingridients.iter().enumerate().map(|(_, t)| (t.0.clone(), t.1.iter().collect::<Vec<&String>>().clone())).collect::<Vec<(String, Vec<&String>)>>();
    let mut done_ingridients = HashSet::new();

    while done_ingridients.len() != vec_alergens.len() {
        for i in 0..vec_alergens.len() {
            if vec_alergens[i].1.len() == 1  {
                done_ingridients.insert(vec_alergens[i].1[0]);
            } else {
                vec_alergens[i].1 = vec_alergens[i].1.iter().filter(|i| !done_ingridients.contains(**i)).map(|i| *i).collect::<Vec<&String>>();
            }
        }
    }

    vec_alergens.sort_by(|a,b| a.0.cmp(&b.0));

    println!("{:?}", vec_alergens);

    println!("part2 {:?}", vec_alergens.iter().fold(vec![], |mut acc, cur| {
        acc.push(cur.1[0]);
        acc
    }).iter().map(|i| String::from(*i)).collect::<Vec<String>>().join(","));
}
