/**
 *  Simplified
    let a: Vec<_> = (1..10).collect();
    let b: Vec<_> = (3..5).collect();

    let a_set: HashSet<_> = a.iter().copied().collect();

    part 1
    assert!(b.iter().all(|item| a_set.contains(item)));
    
    part 2
    assert!(b.iter().any(|item| a_set.contains(item)));
 */

use std::{ collections::HashSet, fs };

pub fn parser(input: &str) -> u32 {
    let mut counter: u32 = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(",").unwrap();

        let (start_a, stop_a) = a.split_once("-").unwrap();
        let (start_b, stop_b) = b.split_once("-").unwrap();

        let task_a: Vec<_> = (start_a.parse::<i32>().unwrap()..=stop_a
            .parse::<i32>()
            .unwrap()).collect();

        let task_b: Vec<_> = (start_b.parse::<i32>().unwrap()..=stop_b
            .parse::<i32>()
            .unwrap()).collect();

        let a_set: HashSet<_> = task_a.iter().copied().collect();

        ///// PART 1
        // let b_set: HashSet<_> = task_b.iter().copied().collect();

        ///// PART 1
        // if task_b.iter().any(|item| a_set.contains(item)) {
        //     counter += 1;
        //     continue;
        // }
        // if task_a.iter().all(|item| b_set.contains(item)) {
        //     counter += 1;
        // }

        ////// PART 2
        if task_b.iter().any(|item| a_set.contains(item)) {
            counter += 1;
        }
    }
    counter
}

fn main() {
    let file_path = format!("{}/src/test.txt", env!("CARGO_MANIFEST_DIR"));
    let input = &fs::read_to_string(&file_path).unwrap();
    println!("{}", parser(input));
}