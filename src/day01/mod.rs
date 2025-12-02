use core::panic;


fn read_input(input: &str) -> Vec<i32> {
    input.lines()
        .map(|it| {
            let num : i32 = it[1..].parse().unwrap();
            match it.chars().nth(0) {
                Some('L') => -1*num,
                Some('R') => num,
                _ => panic!("unknown line {it}"),
            }
        }).collect()
}

#[test]
fn part1() {
    let turns = read_input(include_str!("input.txt"));

    let mut pos = 50;
    let mut pw = 0;
    for t in turns {
        pos += t; 

        if pos % 100 == 0 {
            pw += 1;
        }
    }

    println!("pw: {pw}");
}

#[test]
fn part2() {
    let turns = read_input(include_str!("input.txt"));

    let mut pos = 50;
    let mut pw = 0;

    for mut t in turns {
        if pos % 100 == 0 {
            pos += t.signum(); //move into next segment so we don't double count
            t =- t.signum(); // account for that step
        }

        let last_segment = (pos as f64 / 100.0).floor() as i32;
        pos += t;
        let segment = (pos as f64 / 100.0).floor() as i32;

        let mut steps = (last_segment - segment).abs();
        if steps == 0 && pos % 100 == 0 {
            steps += 1;
        }
        pw += steps;
    }
    println!("pw: {pw}");
}