use std::fs;
use std::collections::HashSet;
use array2d::Array2D;

fn main() {
    let rules1 = vec![
        (48..=793), (800..=971),
        (36..=235), (247..=974),
        (25..=850), (862..=964),
        (40..=173), (193..=959),
        (42..=895), (902..=955),
        (43..=692), (715..=951),
        (38..=528), (549..=967),
        (43..=133), (141..=963),
        (40..=651), (675..=951),
        (48..=801), (811..=973),
        (50..=562), (587..=955),
        (43..=520), (527..=968),
        (44..=745), (752..=950),
        (41..=929), (941..=963),
        (37..=828), (838..=958),
        (47..=475), (491..=972),
        (38..=198), (210..=965),
        (33..=66), (74..=949),
        (35..=492), (507..=962),
        (35..=358), (381..=965)
    ];
    let mut all_numbers_flat: HashSet<usize> = HashSet::new();

    let input = fs::read_to_string("input.txt").unwrap();

    for rule in rules1 {
        for i in rule {
            all_numbers_flat.insert(i);
        }
    }

    let lines = input.lines();
    let part1 = &lines
        .map(|l| {
            l.split(",").map(|num| num.parse::<usize>().unwrap()).into_iter().collect::<Vec<usize>>()
        })
        .map(|line| {
            *line.iter().find(|s| !all_numbers_flat.contains(&s)).unwrap_or(&0)
        })
        .fold(0 , |mut acc, cur| {
            acc += cur;
            acc
        });

    println!("part1 {}", part1);

    /////////////////////////////////////////////

    let rules2 = vec![
        ((48..=793), (800..=971)),
        ((36..=235), (247..=974)),
        ((25..=850), (862..=964)),
        ((40..=173), (193..=959)),
        ((42..=895), (902..=955)),
        ((43..=692), (715..=951)),
        ((38..=528), (549..=967)),
        ((43..=133), (141..=963)),
        ((40..=651), (675..=951)),
        ((48..=801), (811..=973)),
        ((50..=562), (587..=955)),
        ((43..=520), (527..=968)),
        ((44..=745), (752..=950)),
        ((41..=929), (941..=963)),
        ((37..=828), (838..=958)),
        ((47..=475), (491..=972)),
        ((38..=198), (210..=965)),
        ((33..=66), (74..=949)),
        ((35..=492), (507..=962)),
        ((35..=358), (381..=965))
    ];

    let filtered_lines = input.lines()
        .map(|l| {
            l.split(",").map(|num| num.parse::<usize>().unwrap()).into_iter().collect::<Vec<usize>>()
        })
        .filter(|line| {
            line.iter().all(|s| all_numbers_flat.contains(&s))
        })
        .collect::<Vec<Vec<usize>>>();

    let array = Array2D::from_rows(&filtered_lines);

    let mut col_row = vec![];

    for a in 0..rules2.len() {
        for b in 0..rules2.len() {
            if array.column_iter(b).all(|num| {
                rules2[a].0.contains(num) || rules2[a].1.contains(num)
            }) {
                if col_row.len() <= a {
                    col_row.push(vec![b]);
                } else {
                    col_row[a].push(b);
                }
            }
        }
    }

    let mut sorted:Vec<(usize, &Vec<usize>)> = col_row.iter().enumerate().collect();

    sorted.sort_by(|a, b| {
        (a.1.len()).cmp(&(b.1.len()))
    });

    let mut taken = HashSet::new();

    let mut result = sorted.iter().map(|s| {
        let target = s.1.iter().find(|ms| !taken.contains(ms)).unwrap();
        taken.insert(target);
        (s.0, target)
    }).collect::<Vec<(usize, &usize)>>();

    result.sort_by(|a, b| a.0.cmp(&b.0));
    let my_ticket:Vec<usize> = vec![157,59,163,149,83,131,107,89,109,113,151,53,127,97,79,103,101,173,167,61];

    println!("part2 {:#?}", result[0..6].iter().map(|r| my_ticket[*r.1]).fold(1, |acc, cur| cur * acc));

}
