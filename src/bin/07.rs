use itertools::{iproduct, Itertools};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);
    let operands: Vec<fn(&u64, &u64) -> u64> = vec![|a, b| a + b, |a, b| a * b];

    let total_calibration_result = sum_calibration_results(equations, operands);
    Some(total_calibration_result)
}

pub fn parse(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + use<'_> {
    input
        .lines()
        .filter_map(|line| match line.split(": ").collect_tuple() {
            Some((result, values)) => {
                let result = result.parse::<u64>().unwrap();
                let values: Vec<u64> = values.split(" ").map(|v| v.parse().unwrap()).collect();
                Some((result, values))
            }
            None => None,
        })
}

pub fn sum_calibration_results(
    equations: impl Iterator<Item = (u64, Vec<u64>)>,
    operands: Vec<fn(&u64, &u64) -> u64>,
) -> u64 {
    let mut total_calibration_result = 0;
    for (result, values) in equations {
        let values = &values.into_iter().rev().collect_vec()[..];
        let results = evaluate(values[0], &values[1..], &operands);
        if results.contains(&result) {
            total_calibration_result += result;
        }
    }
    total_calibration_result
}

pub fn evaluate(a: u64, values: &[u64], operands: &Vec<fn(&u64, &u64) -> u64>) -> Vec<u64> {
    if values.is_empty() {
        vec![a]
    } else {
        let bs = evaluate(values[0], &values[1..], operands);
        iproduct!(bs.iter(), operands.iter())
            .map(|(b, op)| op(b, &a))
            .collect()
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let operands: Vec<fn(&u64, &u64) -> u64> = vec![|a, b| a + b, |a, b| a * b, |a, b| {
        (a.to_string() + &b.to_string()).parse().unwrap()
    }];

    let total_calibration_result = sum_calibration_results(equations, operands);
    Some(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
