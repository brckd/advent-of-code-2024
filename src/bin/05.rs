use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let valid_updates = updates.iter().filter(|update| {
        let mut update = &update[..];
        while update.len() > 0 {
            for rule in rules.iter().filter(|r| r.1 == update[0]) {
                if update.contains(&rule.0) {
                    return false;
                }
            }
            update = &update[1..];
        }
        true
    });

    Some(valid_updates.map(|u| u[u.len() / 2]).sum())
}

pub fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = Vec::<(u32, u32)>::new();
    let mut updates = Vec::<Vec<u32>>::new();

    for line in input.lines() {
        if line.contains("|") {
            let rule = line
                .split("|")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            rules.push(rule);
        } else if line.contains(",") {
            let update = line.split(",").map(|n| n.parse().unwrap()).collect();
            updates.push(update);
        }
    }
    (rules, updates)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut updates) = parse_input(input);
    let corrected_updates = updates.iter_mut().filter_map(|update| {
        let mut update_slice = &mut update[..];
        let mut is_valid = true;
        'updates: while update_slice.len() > 0 {
            for rule in rules.iter().filter(|r| r.1 == update_slice[0]) {
                let swapped = &mut update_slice.iter().position(|&v| v == rule.0);
                match swapped {
                    Some(index) => {
                        is_valid = false;
                        update_slice.swap(0, *index);
                        continue 'updates;
                    }
                    None => {}
                }
            }
            update_slice = &mut update_slice[1..];
        }
        if is_valid {
            None
        } else {
            Some(update)
        }
    });

    Some(corrected_updates.map(|u| u[u.len() / 2]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
