use std::fs;

fn check_unique(s: &str, s_len: usize) -> bool {
    let mut unique = true;
    dbg!(s);
    for n in 0..s_len {
        if s[n + 1..].contains(s.chars().nth(n).unwrap()) {
            unique = false;
        }
    }
    unique
}

pub fn marker(input: &str) -> Option<usize> {
    let mut marker: Option<usize> = None;
    for i in 0..input.len() - 4 {
        if check_unique(&input[i..i + 4], 4) {
            marker = Some(i + 4);
            break;
        }
    }
    marker
}

pub fn message(input: &str) -> Option<usize> {
    let mut marker: Option<usize> = None;
    for i in 0..input.len() - 14 {
        if check_unique(&input[i..i + 14], 14) {
            marker = Some(i + 14);
            break;
        }
    }
    marker
}
fn main() {
    let file_path = format!("{}/src/input.txt", env!("CARGO_MANIFEST_DIR"));
    let input = &fs::read_to_string(&file_path).unwrap();
    let mark = message(&input).unwrap();
    println!("Beginning: {}", mark);
    println!("Char: {}", input.chars().nth(mark).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(marker(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), Some(5));
        assert_eq!(marker(&String::from("uppdvjthqldpwncqszvftbrmjlhg")), Some(6));
        assert_eq!(marker(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), Some(10));
        assert_eq!(marker(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), Some(11));
    }

    #[test]
    fn test_p2() {
        assert_eq!(message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(message("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(message("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}