use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);

fn parse_input(input: &str) -> (HashSet<Coord>, Coord) {
    let mut S: (usize, usize) = (0, 0);
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                S = (x, y);
            } else if c == '^' {
                splitters.insert((x, y));
            }
        }
    }

    (splitters, S)
}

#[test]
fn part1() {
    let input = include_str!("input.txt");
    let (splitters, (s_x, _s_y)) = parse_input(input);
    let mut beams_x: HashSet<usize> = HashSet::from_iter([s_x]);
    let mut no_of_splits = 0;

    for y in 1..input.lines().count() {
        for x in beams_x.clone() {
            if splitters.contains(&(x, y)) {
                beams_x.remove(&x);
                beams_x.insert(x - 1);
                beams_x.insert(x + 1);
                no_of_splits += 1;
            }
        }
    }

    assert_eq!(no_of_splits, 1633);
}

#[test]
fn part2() {
    let input = include_str!("input.txt");
    let (splitters, start) = parse_input(input);

    fn step(
        (x, y): Coord,
        splitters: &HashSet<Coord>,
        max_y: usize,
        memo: &mut HashMap<Coord, usize>,
    ) -> usize {
        if y >= max_y {
            return 1; //found one path to the bottom
        }
        if let Some(m) = memo.get(&(x, y)) {
            return *m;
        }

        if splitters.contains(&(x, y)) {
            let sum = 
                step((x - 1, y + 1), splitters, max_y, memo) 
                + step((x + 1, y + 1), splitters, max_y, memo);
            memo.insert((x, y), sum);
            return sum;
        } else {
            let down = (x, y + 1);
            return step(down, splitters, max_y, memo);
        }
    }

    let solution = step(start, &splitters, input.lines().count(), &mut HashMap::new());

    assert_eq!(solution, 34339203133559);
}
