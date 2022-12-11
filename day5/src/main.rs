use std::{ str, fs };

pub fn rearrange(mut stacks: Vec<Vec<char>>, instruction: &str) -> Vec<Vec<char>> {
    let inst = instruction.split(" ").collect::<Vec<&str>>();
    dbg!(&inst);
    let item_cnt: i32 = inst[1].parse::<i32>().unwrap();
    let src: usize = inst[3].parse::<usize>().unwrap() - 1;
    let dst: usize = inst[5].parse::<usize>().unwrap() - 1;
    dbg!(instruction);
    dbg!(&stacks);

    println!("items to move {}, from {} to {}", item_cnt, src, dst);

    // for _n in 0..item_cnt {
    //     let target = stacks[src].pop().unwrap();
    //     stacks[dst].push(target);
    // }
    let src_len = stacks[src].len() - 1;

    let tmpvec: Vec<_> = stacks[src].split_off(src_len - (item_cnt as usize));
    let srcstack = stacks[src].to_owned();
    stacks[dst].extend(srcstack);
    stacks[src] = tmpvec;
    dbg!(&stacks);
    stacks
}

pub fn final_stacks(stacks: Vec<Vec<char>>) -> String {
    let mut tops: Vec<char> = vec![];
    for mut v in stacks {
        tops.push(v.pop().unwrap());
    }

    tops.into_iter().collect()
}

fn main() {
    let stream: &str =
        &"[D]
[N] [C]
[Z] [M] [P]
1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".to_string();

    // let file_path = format!("{}/src/orders.txt", env!("CARGO_MANIFEST_DIR"));
    // let stream = &fs::read_to_string(&file_path).unwrap();

    let (stacks, moves) = stream.split_at(stream.find("\n\n").unwrap());
    dbg!(&stacks);
    dbg!(&moves);

    let sub_len = 4;

    // let stackcount = (stacks.find("\n").unwrap() + 1) / sub_len;
    let stackcount = 3;
    dbg!(stackcount);

    let subs = stacks
        .as_bytes()
        .chunks(sub_len)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    dbg!(&subs);
    let mut stack_container: Vec<Vec<char>> = vec![vec![]; stackcount];
    // }

    let mut cnt = 0;
    let mut c;
    for sub in subs {
        c = sub.chars().nth(1).unwrap();
        if !(c == ' ') && !c.is_numeric() {
            stack_container[cnt].push(c);
        }
        if cnt == stackcount - 1 {
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    for n in 0..stackcount {
        stack_container[n].reverse();
    }

    dbg!(&stack_container);

    for line in moves.lines() {
        dbg!(line);
        if line.is_empty() {
            continue;
        }
        stack_container = rearrange(stack_container, line);
        dbg!(&stack_container);
    }

    let res = final_stacks(stack_container);
    println!("res: {}", res);
    assert!(res == "CMZ".to_string());
}