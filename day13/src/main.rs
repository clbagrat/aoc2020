
use std::{fs, time, thread};



fn is_ok(time:i64, bus:i64) -> bool {
    time % bus == 0
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let my_time = lines.next().unwrap().parse::<i32>().unwrap();
    let mut bus_schedule:Vec<i32> = lines.next().unwrap().split(",").filter(|id| *id != "x").map(|id| id.parse::<i32>().unwrap()).collect();

    bus_schedule.sort_by(|a, b| {
        (my_time%b).cmp(&(my_time%a))
    });
    let my_bus = bus_schedule[0];
    println!("part1 {}", ((my_time/my_bus + 1) * my_bus - my_time) * my_bus);

    ////////////////////////////////////////////////////////////////////////////////

    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let bus_schedule:Vec<&str> = lines.nth(1).unwrap().split(",").collect();
    let mut time = 0;
    let mut jump: i64 = bus_schedule[0].parse().unwrap();

    for i in 1..bus_schedule.len()
    {
        let bus = bus_schedule[i];

        if bus == "x" {continue}

        let bus_id = bus.parse::<i64>().unwrap();
        loop
        {
            time += jump;
            if (time + i as i64) % bus_id == 0
            {
                jump *= bus_id;
                break;
            }
        }
    }

    println!("part2 {}", time);
}
