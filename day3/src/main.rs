/**
    ASCII
    a	097	01100001 h61	A	065	01000001
    z	122	01111010 h7A	Z	090	01011010
    
    GAME
    a   001 00000001 01   A   027 00011011 1B
    z   026 00011010 1A   Z   052 00110100 h34

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.
 *  */
use std::{ collections::HashSet, fs };

pub fn special_item(stuff: &str) -> char {
    let half = stuff.len() / 2;
    let first: HashSet<char> = stuff[..half].chars().collect();
    let second: HashSet<char> = stuff[half..].chars().collect();
    let same: HashSet<_> = first.intersection(&second).collect();
    let special = **same.iter().next().unwrap();
    special
}

pub fn group_priority(backpacks: &str) -> u16 {
    let stuff = backpacks.lines().collect::<Vec<_>>();
    let mut line_count: usize = stuff.len();
    let mut res: Vec<u16> = vec![];
    while line_count >= 2 {
        let first: HashSet<char> = stuff[line_count - 1].chars().collect();
        line_count -= 1;
        let second: HashSet<char> = stuff[line_count - 1].chars().collect();
        line_count -= 1;
        let third: HashSet<char> = stuff[line_count - 1].chars().collect();
        line_count -= 1;

        let same_a: HashSet<_> = first.intersection(&second).collect();
        let same_b: HashSet<_> = first.intersection(&third).collect();
        // let same_c: HashSet<_> = second.intersection(&third).collect();
        // let same_ab: HashSet<_> = same_a.intersection(&same_b).collect();
        // let same_bc: HashSet<_> = same_b.intersection(&same_c).collect();
        let same: HashSet<_> = same_a.intersection(&same_b).collect();
        res.push(calc_priority(**same.iter().next().unwrap()));
    }
    res.iter().sum::<u16>()
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
pub fn calc_priority(ascii: &char) -> u16 {
    let mut buf = [0; 4];
    let dist: u16;
    match ascii.is_ascii_lowercase() {
        true => {
            dist = 96;
        }
        false => {
            dist = 65 - 27;
        }
    }
    ascii.encode_utf16(&mut buf)[0].checked_sub(dist).unwrap()
}

// PART 1/
// pub fn priority(items: Vec<char>) -> u16 {
//     items
//         .iter()
//         .map(|ascii| calc_priority(ascii))
//         .sum()
// }

fn main() {
    let file_path = format!("{}/src/rucksacks", env!("CARGO_MANIFEST_DIR"));
    let stuff = &fs::read_to_string(&file_path).unwrap();
    // let items = collect_special_items(stuff);
    // let res = priority(items);
    let res = group_priority(stuff);
    println!("{}", res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_intersection_w_char() {
        let a = HashSet::from(['a', 'b', 'c']);
        let b = HashSet::from(['x', 'x', 'a', 'i']);

        // Print 1, 2, 3, 4 in arbitrary order.
        for x in a.intersection(&b) {
            println!("{x}");
        }

        let intersection: HashSet<_> = a.intersection(&b).collect();
        assert_eq!(intersection, ['a'].iter().collect());
    }

    #[test]
    fn simple_rucksack() {
        let rucksacks = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        );

        let bag: Vec<&str> = rucksacks.trim().lines().collect();
        // dbg!(&bag);
        assert_eq!(special_item(bag[0]), 'p');
        assert_eq!(special_item(bag[1]), 'L');
        assert_eq!(special_item(bag[2]), 'P');
        assert_eq!(special_item(bag[3]), 'v');
        assert_eq!(special_item(bag[4]), 't');
        assert_eq!(special_item(bag[5]), 's');
    }
    #[test]
    fn prio() {
        assert_eq!(calc_priority(&'a'), 1);
        assert_eq!(calc_priority(&'A'), 27);
        assert_eq!(calc_priority(&'p'), 16);
        assert_eq!(calc_priority(&'L'), 38);
        assert_eq!(calc_priority(&'v'), 22);
        assert_eq!(calc_priority(&'t'), 20);
        assert_eq!(calc_priority(&'s'), 19);
    }

    #[test]
    fn total() {
        let input = &String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
        );

        assert_eq!(priority(collect_special_items(input)), 157)
    }
}