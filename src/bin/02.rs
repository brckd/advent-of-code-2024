advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports_amount = input.lines().filter(|line| {
        let report: Vec<u32> = line.split(" ").map(|l| l.parse::<u32>().unwrap()).collect();
        report_is_valid(&report)
    }).count() as u32;

    Some(safe_reports_amount)
}

pub fn difference_is_valid((first_level, second_level): (&u32, &u32)) -> bool {
    let diff = first_level.abs_diff(*second_level);
    diff >= 1 && diff <= 3
}

pub fn ordering_is_valid((first_level, second_level, third_level): (&u32, &u32, &u32)) -> bool {
    let first_ord = first_level.cmp(second_level);
    let second_ord = second_level.cmp(third_level);
    first_ord == second_ord
}

pub fn report_is_valid(report: &Vec<u32>) -> bool {
    let mut level_pairs = report.iter().zip(report.iter().skip(1));
    if !level_pairs.all(difference_is_valid) {
        return false
    }

    if !(report.iter().is_sorted() || report.iter().rev().is_sorted()) {
        return false
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports_amount = input.lines().filter(|line| {
        let report: Vec<u32> = line.split(" ").map(|l| l.parse::<u32>().unwrap()).collect();
        report_is_valid(&report) || (0..report.len()).any(|i| {
            let mut report = report.clone();
            report.remove(i);
            report_is_valid(&report)
        })
    }).count() as u32;

    Some(safe_reports_amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
