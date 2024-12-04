use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_ids, mut right_ids) = parse_ids(input);

    left_ids.sort();
    right_ids.sort();

    let id_pairs = left_ids.iter().zip(right_ids.iter_mut());
    let total_distance = id_pairs.fold(0, |p, (id1, id2)| p + id1.abs_diff(*id2)) as u32;

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_ids, right_ids) = parse_ids(input);

    let mut right_id_counts = HashMap::<u32, u32>::new();

    for right_id in right_ids {
        *right_id_counts.entry(right_id).or_insert(0) += 1;
    }

    let mut similarity_score: u32 = 0;
    for left_id in left_ids {
        similarity_score += left_id * right_id_counts.get(&left_id).unwrap_or(&0);
    }
    Some(similarity_score)
}

pub fn parse_ids(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let ids: Vec<u32> = line.split("   ").map(|p| p.parse().unwrap()).collect();
            (ids[0], ids[1])
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
