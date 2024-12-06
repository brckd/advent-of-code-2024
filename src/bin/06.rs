use itertools::iproduct;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lab: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    walk(&mut lab);
    Some(lab.iter().flatten().filter(|&&c| "^>v<".contains(c)).count() as u32)
}

pub fn walk(lab: &mut Vec<Vec<char>>) -> bool {
    let mut position = lab.iter().enumerate().find_map(|(y, row)| {
        let x = row.iter().position(|&char| char == '^');
        match x {
            Some(x) => Some(Position{x, y}),
            None => None
        }
    }).unwrap();


    let mut direction = Direction{x: 0, y: -1};
    'walk: loop {
        lab[position.y][position.x] = direction_indicator(&direction);
        'turn: loop {
            let next_position = Position{x: position.x.wrapping_add_signed(direction.x), y: position.y.wrapping_add_signed(direction.y)};
            if next_position.x >= lab.len() || next_position.y >= lab[0].len() {
                break 'walk;
            }
            if lab[next_position.y][next_position.x] == '#' {
                direction = Direction{x: -direction.y, y: direction.x};
            } else {
                position = next_position;
                break 'turn
            }
        }
        let indicator = direction_indicator(&direction);
        if indicator == lab[position.y][position.x] {
            return true;
        }
    };
    false
}

pub struct Position {
    x: usize,
    y: usize,
}
pub struct Direction {
    x: isize,
    y: isize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let lab: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut loops = 0u32;
    for obstacle in iproduct!(0..lab.len(), 0..lab[0].len()).map(|(y, x)| Position{x, y}) {
        if lab[obstacle.y][obstacle.x] != '.' {continue;};
        let mut obstructed_lab = lab.clone();
        obstructed_lab[obstacle.y][obstacle.x] = '#';
        if walk(&mut obstructed_lab) {loops += 1};
    }
    Some(loops)
}

pub fn direction_indicator(direction: &Direction) -> char {
    match direction {
        Direction{x: 0, y: -1} => '^',
        Direction{x: 0, y: 1} => 'v',
        Direction{x: -1, y: 0} => '<',
        Direction{x: 1, y: 0} => '>',
        _ => 'X',
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
