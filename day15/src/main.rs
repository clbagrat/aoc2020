use std::collections::HashMap;

fn speak( last_spoken: i32, memory: &HashMap<i32, Vec<i32>>) -> i32 {
    if memory.contains_key(&last_spoken) {
        let history = memory.get(&last_spoken).unwrap();
        if history.len() == 1 {0} else {(history[0]-history[1]).abs()}
    } else {
        0
    }
}

fn insert_to_memory(mut memory: HashMap<i32, Vec<i32>>, number: i32, turn: i32) -> HashMap<i32, Vec<i32>> {
    if memory.contains_key(&number) {
        let v = memory.get_mut(&number).unwrap();
        v.insert(0, turn);
        if v.len() > 2 {
            v.pop();
        }
    } else {
        memory.insert(number, vec![turn]);
    }

    memory
}

fn main() {
    let input = [6,4,12,1,20,0,16];
    let mut memory = HashMap::new();
    let mut turn = 1;

    for i in 0..input.len() {
        memory = insert_to_memory(memory, input[i], turn);
        turn += 1;
    }
    let mut last_spoken = input[input.len() -1];
    for t in turn..=2020 {
        last_spoken = speak(last_spoken, &memory);
        memory = insert_to_memory(memory, last_spoken, t);
    }

    println!("{:#?}", last_spoken);

    /////////////////
    // for part 2 i just waited a little bit longer :D :D
    /////////////////


    // for t in turn..=2020 {
    //     last_spoken = speak(last_spoken, &memory);
    //     memory = insert_to_memory(memory, last_spoken, t);
    // }
    // println!("{:#?}", last_spoken);
}
