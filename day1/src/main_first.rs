use std::fs;

fn part_1(stream: &String) {
    let s: Vec<&str> = stream.split("\n").collect();
    let mut elves: Vec<u32> = Vec::from([0]);
    for calories in &s {
        if calories.is_empty() {
            elves.push(0);
        } else {
            let c: u32 = calories.trim().parse::<u32>().unwrap() + elves.pop().unwrap();
            elves.push(c);
        }
    }
    println!("{:?}", &elves.iter().max().unwrap());
}

fn part_2(stream: &String) {
    let s: Vec<&str> = stream.split("\n").collect();
    let mut elves: Vec<u32> = Vec::from([0]);
    for calories in &s {
        if calories.is_empty() {
            elves.push(0);
        } else {
            let c: u32 = calories.trim().parse::<u32>().unwrap() + elves.pop().unwrap();
            elves.push(c);
        }
    }
    elves.sort();
    elves.reverse();
    let top_elves: &u32 = &elves[..3].iter().sum();
    println!("{}", top_elves)
}
fn main() {
    let file_path = format!("{}/src/calories.txt", env!("CARGO_MANIFEST_DIR"));
    match &fs::read_to_string(&file_path).ok() {
        Some(calories) => {
            part_1(calories);
            part_2(calories)
        }
        None => print!("Couldnt read from file: {}\r\n", &file_path),
    }
}