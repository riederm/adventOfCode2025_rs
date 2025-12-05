use std::ops::{RangeInclusive};

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges.lines().map(|l| l.split_once("-").unwrap())
        .map(|(s,e)| s.parse().unwrap()..=e.parse().unwrap())
        .collect();

    let ingredients = ingredients.lines().map(|l| l.parse().unwrap()).collect();

    (ranges, ingredients)
}

#[test]
fn part1() {
    let (ranges, ingredients) = parse_input(include_str!("input.txt"));
    let fresh = ingredients.iter()
        .filter(|it| ranges.iter().any(|r| r.contains(it)))
        .count();
    
    println!("fresh: {}", fresh);
}

#[test]
fn part2() {
    let (mut ranges, _) = parse_input(include_str!("input.txt"));

    // sort reversed
    ranges.sort_by(|a,b| b.start().cmp(a.start()));
    let mut unique = Vec::new();
    while !ranges.is_empty() {
        match (ranges.pop(), ranges.pop()) {
            (Some(r1), Some(r2)) => {
                if r1.start() <= r2.end() && r2.start() <= r1.end() {
                    let merged = *r1.start().min(r2.start())..=*r1.end().max(r2.end());
                    ranges.push(merged);
                } else {
                    unique.push(r1);
                    ranges.push(r2);
                }
            },
            (Some(r1), None) => {
                unique.push(r1);
            },
            _ => {},
        }
    }

    let total: u64 = unique.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("total unique ingredients: {}", total);
}