use std::fs;
use std::collections::HashMap;

fn merge(mask: &str, binary_num: &str) -> String {
    let mask_arr : Vec<&str> = mask.split("").collect();
    let binary_array : Vec<&str> = binary_num.split("").collect();
    let mut res = vec![];

    for i in 1..mask_arr.len() {
        res.push(
            if mask_arr[i] != "X" {mask_arr[i]} else {binary_array[i]}
        )
    }

    String::from(res.join(""))
}


fn combinations(mask: &str, binary_mem_num: &str) -> Vec<String> {
    let mask_arr : Vec<&str> = mask.split("").collect();
    let binary_array : Vec<&str> = binary_mem_num.split("").collect();
    let mut line_vec = vec![];
    let mut all_combinations = vec![];
    for i in 1..mask_arr.len() {
        match mask_arr[i] {
            "1" => {line_vec.push("1")},
            "X" => {line_vec.push("X")},
            _ => {line_vec.push(binary_array[i])}
        }
    }
    let line = line_vec.join("");
    let amount = line.matches("X").count();
    let split = line.split("X").collect::<Vec<&str>>();
    for i in 0..(2 as i32).pow(amount as u32) {
        let a = format!("{:032b}", i);
        let mask_vec = &a[a.len()-amount..].split("").filter(|x| x != &"").collect::<Vec<&str>>();
        let mut res = vec![];
        for p in 0..split.len() {
            res.push(split[p]);
            if p != split.len() -1 {
                res.push(mask_vec[p]);
            }
        }
        all_combinations.push(res.join(""));
    }

    all_combinations
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut memory = HashMap::new();
    let mut current_mask = "";

    for line in lines {
        let split = line.split(" = ").collect::<Vec<&str>>();
        if split[0] == "mask" {
            current_mask = split[1];
        } else {
            let mem_key = &split[0][split[0].find("[").unwrap()..split[0].find("]").unwrap()];
            memory.insert(mem_key, merge(&current_mask, &format!("{:036b}", &split[1].parse::<i32>().unwrap())));
        }
    }

    println!("part1 {:?}", memory.values().fold(0, |mut acc, bin| {
        acc += isize::from_str_radix(&bin, 2).unwrap();
        acc
    }));

    //////////////////////////////////////////////////////////////

    let lines = input.lines();
    let mut memory = HashMap::new();
    let mut current_mask = "";

    for line in lines {
        let split = line.split(" = ").collect::<Vec<&str>>();
        if split[0] == "mask" {
            current_mask = split[1];
        } else {
            let mem_key = &split[0][split[0].find("[").unwrap()+1..split[0].find("]").unwrap()];
            // println!("{}", mem_key);
            for c in combinations(current_mask, &format!("{:036b}", mem_key.parse::<i64>().unwrap())) {
                memory.insert(isize::from_str_radix(&c, 2).unwrap(), split[1].parse::<i32>().unwrap());
            }
        }
    }

    println!("part1 {:?}", memory.values().fold(0 as i128, |mut acc, bin| {
        acc += *bin as i128;
        acc
    }));
}
