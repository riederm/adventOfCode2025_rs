pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|it| {
            it.chars()
                .map(|c| c.to_string().parse::<_>().unwrap())
                .collect::<_>()
        })
        .collect::<_>()
}

fn find_first_max(input: &[u64], start: usize, end: usize) -> (usize, u64) {
    input[start..end]
        .into_iter()
        .enumerate()
        .fold((0, 0), |(max_idx, max_v), (i, v)| 
            if *v > max_v {
                (i, *v)
            } else {
                (max_idx, max_v)
            }
        )
}

fn comput_joltages(input: &str, n: usize) -> u64 {
    let mut sum = 0u64;
    let parsed = parse_input(input);
    for p in parsed {
        let mut val = 0u64;
        let mut start = 0;
        for i in (0..n).rev() {
            let (idx, max) = find_first_max(&p, start, p.len() - i);
            val += max * 10_u64.pow(i as _);
            start += idx + 1;
        }
        sum += val;
    }
    sum
}

#[test]
fn part1() {
    let input = include_str!("input.txt");
    assert_eq!(17427, comput_joltages(input, 2))
}

#[test]
fn part2() {
    let input = include_str!("input.txt");
    assert_eq!(173161749617495, comput_joltages(input, 12))
}
