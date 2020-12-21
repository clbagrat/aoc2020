use std::fs;
use regex::Regex;

//uncomment for part1

// #[derive(Debug)]
// struct Tile {
//     id: i64,
//     sides: Vec<String>,
//     flip_sides: Vec<String>
// }

// impl Tile {
//     pub fn new(tile_source: &str) -> Self {
//         let lines = split_once(tile_source, '\n');
//         let pic = String::from(lines.1).replace("\n", "");
//         let top = &pic[0..10];
//         let bot = &pic[pic.len()-10..];
//         let right = &pic.split("").step_by(10).collect::<Vec<&str>>().join("");
//         let left = &pic.split("").filter(|s| s != &"").step_by(10).collect::<Vec<&str>>().join("");

//         Tile {
//             id: lines.0.parse::<i64>().unwrap(),
//             sides: vec![String::from(top), String::from(right), String::from(bot), String::from(left)],
//             flip_sides: vec![top.chars().rev().collect(), right.chars().rev().collect(), bot.chars().rev().collect(), left.chars().rev().collect()]
//         }
//     }
// }

// fn split_once(in_string: &str, del: char) -> (&str, &str) {
//     let mut splitter = in_string.splitn(2, del);
//     let first = splitter.next().unwrap();
//     let second = splitter.next().unwrap();
//     (first, second)
// }


// fn main() {
//     let input = fs::read_to_string("input.txt").unwrap();
//     let tiles = input.split("\n\n").map(|tile_source| Tile::new(tile_source)).collect::<Vec<Tile>>();


//     let mut all_edges = tiles.iter().flat_map(|tile| tile.sides.iter().map(|i| i)).collect::<Vec<&String>>();
//     let mut all_flipped = tiles.iter().flat_map(|tile| tile.flip_sides.iter().map(|i| i)).collect::<Vec<&String>>();
//     all_edges.append(&mut all_flipped);

//     let res = tiles.iter().filter(|tile| {
//         let count = tile.sides.iter().filter(|side| all_edges.iter().filter(|edge| edge == &side).count() > 1).count() + tile.flip_sides.iter().filter(|side| all_edges.iter().filter(|edge| edge == &side).count() > 1).count();
//         count <= 4
//     }).map(|tile| tile.id).collect::<Vec<i64>>().iter().fold(1, |acc, cur| cur * acc);

//     println!("part1 {:#?}", res);
// }


#[derive(Debug, Clone, PartialEq)]
struct Tile {
    id: i64,
    data: Vec<Vec<char>>,
    coords: (i32, i32)
}

impl Tile {
    pub fn new(tile_source: &str) -> Self {
        let lines = split_once(tile_source, '\n');
        let pic = String::from(lines.1).replace("\n", "");
        let size = (pic.len() as f64).sqrt() as usize;
        let mut data = vec![vec![]; size];
        let mut i = 0;
        for ch in pic.chars() {
            data[i/size].insert(i%size, ch);
            i += 1;
        }

        Tile {
            id: lines.0.parse::<i64>().unwrap(),
            data,
            coords: (0, 0)
        }
    }

    pub fn left(&self) -> String {
        let mut res = vec![];
        for line in self.data.iter() {
            res.push(line[0]);
        }
        res.iter().collect::<String>()
    }

    pub fn right(&self) -> String {
        let mut res = vec![];
        for line in self.data.iter() {
            res.push(line[self.data.len() - 1]);
        }
        res.iter().collect::<String>()
    }

    pub fn top(&self) -> String {
        self.data[0].iter().collect::<String>()
    }

    pub fn bottom(&self) -> String {
        self.data[self.data.len() - 1].iter().collect::<String>()
    }

    pub fn side(&self, side: &str) -> String{
        match side {
            "left" => self.left(),
            "right" => self.right(),
            "bottom" => self.bottom(),
            _ => self.top()
        }
    }

    pub fn get_content(&self) -> Vec<String> {
        let cont = self.clone().data;
        cont[1..cont.len()-1].iter().map(|v| {
            v[1..v.len()-1].iter().collect::<String>()
        }).collect::<Vec<String>>()
    }

    pub fn get_full_content(&self) -> String {
        let cont = self.clone().data;
        String::from(cont.iter().map(|v| {
            v.iter().collect::<String>()
        }).collect::<Vec<String>>().join("\n"))
    }


    pub fn rotate(tile: &Tile) -> Tile {
        let original_data = &tile.data;
        let size = original_data.len();
        let mut data = vec![vec![]; size];

        for y in 0..data.len() {
            for x in 0..data.len() {
                data[y].insert(x, original_data[size - 1 - x][y]);
            }
        }

        Tile {
            id: tile.id,
            data,
            coords: tile.coords
        }
    }

    pub fn flip(tile: &Tile) -> Tile {
        let original_data = &tile.data;
        let mut data = vec![];

        for y in 0..original_data.len() {
            data.insert(y, original_data[y].iter().clone().map(|&ch| char::from(ch)).rev().collect::<Vec<char>>())
        }

        Tile {
            id: tile.id,
            data,
            coords: tile.coords
        }
    }

    pub fn find_match(t1: &Tile, t1_side: &str, t2: &Tile, t2_side: &str) -> Option<Tile> {
        // println!("--");
        // println!("{:?} {:?}", t1, t2);
        // println!("--");
        if t1.id == t2.id {return None};
        let mut tile = t2.clone();
        for _ in 0..4 {
            if t1.clone().side(t1_side) == tile.side(t2_side) {
                return Some(tile);
            } else {
                tile = Tile::rotate(&tile.clone());
            }
        }
        tile = Tile::flip(&tile.clone());

        for _ in 0..4 {
            if t1.clone().side(t1_side) == tile.side(t2_side) {
                return Some(tile);
            } else {
                tile = Tile::rotate(&tile.clone());
            }
        }

        None
    }
}

fn split_once(in_string: &str, del: char) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, del);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut tiles = input.split("\n\n").map(|tile_source| Tile::new(tile_source)).collect::<Vec<Tile>>();


    let mut target_tile = tiles[0].clone();
    // at this moment i know left top corner (from part 1). I also adjested an input to left top will be the first (id: 3643);
    target_tile.coords = (0, 0);
    tiles.remove(0);
    let before = tiles.len();
    let mut rights = vec![target_tile.clone()];
    println!("before {}", tiles.len());
    'right: loop {
        for i in 0..tiles.len() {
            let tile = tiles[i].clone();
            let match_right = Tile::find_match(&target_tile, "right", &tile, "left");
            if match_right != None {
                let mut goal_tile = match_right.unwrap().clone();
                println!("match {} {}", target_tile.id, goal_tile.id);
                goal_tile.coords = (target_tile.coords.0+1, target_tile.coords.1);
                rights.push(goal_tile.clone());
                target_tile = goal_tile.clone();
                tiles.remove(i);
                break;
            }
            if i == tiles.len()-1 {
                break 'right;
            }
        }
    }

    println!("after rights {}", tiles.len());
    println!("{:?}", rights.iter().map(|t| t.id).collect::<Vec<i64>>());

    target_tile = rights[0].clone();
    'left: loop {
        for i in 0..tiles.len() {
            let tile = tiles[i].clone();
            println!("trying {} {}", target_tile.id, tile.id);
            let match_left = Tile::find_match(&target_tile, "left", &tile, "right");
            if match_left != None {
                let mut goal_tile = match_left.unwrap().clone();
                println!("match {} {}", target_tile.id, goal_tile.id);
                goal_tile.coords = (target_tile.coords.0-1, target_tile.coords.1);
                rights.push(goal_tile.clone());
                target_tile = goal_tile.clone();
                tiles.remove(i);
                break;
            }
            if i == tiles.len()-1 {
                break 'left;
            }
        }
        // println!("============================");
        // println!("============================");
        // println!("============================");
    }

    println!("after lefts {}", tiles.len());

    let mut bottoms = vec![];
    for k in 0..rights.len() {
        let original_right = rights[k].clone();
        target_tile = original_right.clone();
        println!("kkkk {} {}", k, rights.len());

        'bottom: loop {
            for i in 0..tiles.len() {
                let tile = tiles[i].clone();
                println!("trying {} {}", i, tiles.len());
                let match_bottom = Tile::find_match(&target_tile, "top", &tile, "bottom");
                if match_bottom != None {
                    let mut goal_tile = match_bottom.unwrap().clone();
                    println!("match {} {}", target_tile.id, goal_tile.id);
                    goal_tile.coords = (target_tile.coords.0, target_tile.coords.1 + 1);
                    bottoms.push(goal_tile.clone());
                    target_tile = goal_tile.clone();
                    tiles.remove(i);
                    println!("asdasdasd");
                    break;
                }
                if i == tiles.len()-1  {
                    println!("DONE");
                    break 'bottom;
                }
            }
            if tiles.is_empty() {
                break;
            }
        }
    }

    println!("after bottoms {}", tiles.len());

    println!("{:?}", bottoms.iter().map(|t| t.coords).collect::<Vec<(i32, i32)>>());

    let mut matrix:Vec<Vec<Vec<String>>> = vec![];
    let mut ref_vec = vec![];
    ref_vec.resize(((before + 1) as f64).sqrt() as usize, vec![]);
    matrix.resize(((before + 1) as f64).sqrt() as usize, ref_vec);



    for right in rights {
        matrix[right.coords.1 as usize][right.coords.0 as usize] = right.get_content();
    }
    for bottom in bottoms {
        matrix[bottom.coords.1 as usize][bottom.coords.0 as usize] = bottom.get_content();
    }

    let mut parags = vec![String::from("144")];
    for v in matrix {
        println!("{:?}", v);
        let count = *&v[0].len() as isize;
        let mut text = vec![];
        for i in 0..count {
            let mut line = vec![];
            for tile in &v {
                line.push(tile[i as usize].clone());
            }
            text.insert(0, line.join(""));
        }
        parags.push(text.join("\n"));
    }

    // let mapSize = Math.sqrt(9) * 8;

    let size = ((before + 1) as f64).sqrt() as isize * 8;
    let pitch = size + 1;
    let mut res = Tile::new(&parags.join("\n"));

    let regex_str = format!("..................#.(.|\n){{{}}}#....##....##....###(.|\n){{{}}}.#..#..#..#..#..#...", pitch-20, pitch-20);
    let re = Regex::new(&regex_str).unwrap();



    // r"..................#.(.*)\n#....##....##....###(.*)\n#..#..#..#..#..#...(.*)\n"

    // println!("{}", parags.join("\n"));
    for _ in 0..3 {
        res = Tile::rotate(&res);
        if re.is_match(&res.get_full_content()) {

            println!("{}", regex_str);
            do_shit(res.get_full_content(), re.captures_iter(&res.get_full_content()).count())
        }
        println!("==========");
    }
    res = Tile::flip(&res);
    if re.is_match(&res.get_full_content()) {
        println!("{}", regex_str);
        do_shit(res.get_full_content(), re.captures_iter(&res.get_full_content()).count())
    }
    println!("==========");
    for _ in 0..3 {
        res = Tile::rotate(&res);
        if re.is_match(&res.get_full_content()) {
            println!("{}", regex_str);
            do_shit(res.get_full_content(), re.captures_iter(&res.get_full_content()).count())
        }
        println!("==========");
    }
}

fn do_shit(st: String, monster_count: usize) {


    println!("{}", st);
    println!("{}", monster_count);
    println!("part2 {}", st.split("").filter(|c| c == &"#").count() - (monster_count * 15));
}