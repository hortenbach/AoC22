use std::fs;

pub fn part_1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|snip| {
            snip.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    result.to_string()
}

fn part_2(input: &String) -> String {
    let mut result = input
        .split("\n\n")
        .map(|snip| {
            snip.lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    result.sort_by(|x, y| y.cmp(x));

    let sum: u32 = result[..3].iter().take(3).sum();
    sum.to_string()
}

fn main() {
    let file_path = format!("{}/src/calories.txt", env!("CARGO_MANIFEST_DIR"));
    let calories = &fs::read_to_string(&file_path).unwrap();
    println!("part 1: {}", part_1(&calories));
    println!("part 2: {}", part_2(&calories));
}