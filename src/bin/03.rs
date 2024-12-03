use regex::{Regex};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let captures_iter = re.captures_iter(input);
    let results = captures_iter.map(|captures| {
        captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap()
    });
    Some(results.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do(?:n't)?\(\)").unwrap();
    let mut enabled = true;
    let mut result = 0;

    for captures in re.captures_iter(input) {
        if &captures[0] == "do()" {
            enabled = true;
        } else if &captures[0] == "don't()" {
            enabled = false;
        } else if enabled {
            result += captures[1].parse::<u32>().unwrap() * captures[2].parse::<u32>().unwrap();
        };
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
