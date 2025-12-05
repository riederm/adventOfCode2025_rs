use std::collections::HashSet;

fn read_map(input: &str) -> HashSet<(i32, i32)> {
    let mut map = HashSet::new();
    for (y, l) in input.lines().enumerate() {
        for (x, _) in l.chars().enumerate().filter(|(_, c)| *c == '@') {
            map.insert((x as i32, y as i32));
        }
    }
    map
}

#[test]
fn part1_and_2() {
    let input = include_str!("input.txt");
    let neighbours = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut papers = read_map(input);
    let mut running = true;
    let mut part1 = None;
    while running {
        running = false;
        // let papers_clone = papers.clone();

        let mut removed = HashSet::new();
        for (x, y) in papers.iter() {
            let n = neighbours
                .iter()
                .map(|(dx, dy)| (x+dx, y+dy))
                .filter(|pos| papers.contains(pos) && !removed.contains(pos))
                .count();
            if n < 4 {
                if removed.insert((*x, *y)) {
                    running = true;
                }
            }
        }
        if part1.is_none() {
            part1 = Some(removed.len());
        }
        // papers.pr
    }

    println!("Part 1: {}", part1.unwrap());
    // println!("Part 1: {}", movables);
}
