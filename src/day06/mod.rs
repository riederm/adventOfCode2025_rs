use itertools::Itertools;

pub fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut lines = input.lines().rev();

    let operators = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<char>>();

    let mut numbers = Vec::new();
    for line in lines {
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<_>().unwrap())
            .collect::<Vec<_>>();
        numbers.push(row);
    }
    let transposed = transpose(numbers);
    (transposed, operators)
}

//transpose numbers matrix
fn transpose<T>(numbers: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut transposed = Vec::new();
    for i in 0..numbers[0].len() {
        let mut row = Vec::new();
        for j in 0..numbers.len() {
            row.push(numbers[j][i].clone());
        }
        row.reverse();
        transposed.push(row);
    }
    transposed
}

pub fn parse_part2(input: &str) -> (Vec<(usize, char)>, Vec<Vec<String>>) {
    let mut lines = input.lines().rev();

    let ops_line = lines.next().unwrap();
    let mut ops = Vec::new();
    ops_line.char_indices().for_each(|(i, c)| {
        if c == '+' || c == '*' {
            ops.push((i, c));
        }
    });

    let mut numbers = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        let last = [(ops_line.len(), ' ')];
        let ops_ext = ops.iter().chain(last.iter());

        for ((start, _), (end, _)) in ops_ext.tuple_windows() {
            let segment = &line[*start..*end];
            row.push(segment.to_string());
        }
        numbers.push(row);
    }
    let transposed = transpose(numbers);
    (ops, transposed)
}

#[test]
fn part1() {
    let input = include_str!("input.txt");

    let (numbers, operators) = parse_input(input);

    let result = numbers
        .iter()
        .zip(operators)
        .map(|(col, op)| match op {
            '+' => col.iter().sum::<i64>(),
            '*' => col.iter().product::<i64>(),
            _ => panic!("Unknown operator"),
        })
        .sum::<i64>();

    assert_eq!(dbg!(result), 6169101504608);
}

#[test]
fn part2() {
    let input = include_str!("input.txt");
    let (operators, numbers) = parse_part2(input);

    let mut total = 0;
    for (column, (_, op)) in numbers.iter().zip(operators.iter()) {
        let mut result = if *op == '*' { 1 } else { 0 };
        for digit in 0..column[0].len() {
            let value = column.iter().map(|num| 
                    num.chars().nth(digit).unwrap()).collect::<String>()
                    .trim().parse::<i64>();
            if let Ok(value) = value {
                match op {
                    '+' => result += value,
                    '*' => result *= value,
                    _ => panic!("Unknown operator"),
                }
            }
        }
        total += result;
    }
    assert_eq!(dbg!(total), 10442199710797);
}
