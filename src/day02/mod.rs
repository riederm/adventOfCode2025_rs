fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|s| s.trim().split_once("-").unwrap())
        .map(|(l,r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect()
}

fn is_palindrome(n: i64) -> bool {
    let s = format!("{n}");
    let len = s.len();
    if len % 2 == 0 {
        let first = s.chars().into_iter().take(len / 2);
        let second = s.chars().into_iter().skip(len / 2);
        first.eq(second)
    } else {
        false
    }
}

fn is_repeatodrome(n: i64) -> bool {
    let s = format!("{n}");
    let len = s.len();
    for i in 1..=(len / 2) {
        if len % i == 0 {
            let candidate = s.chars().take(i).cycle().take(len);
            if s.chars().eq(candidate) {
                return true;
            }
        }
    }
    return false;
}

#[test]
fn part1() {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for (start, end) in parse_input(include_str!("input.txt")) {

        for i in start..=end {
            if is_palindrome(i) { // part 1
                sum1 += i;
            }

            if is_repeatodrome(i) { // part 2
                sum2 += i;
            }
        }
    }

    dbg!(sum1);
    dbg!(sum2);
    panic!() // force print to console
}
