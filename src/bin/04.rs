use std::{
    ops::{Add, Neg, Sub},
    str::Chars,
};

use itertools::iproduct;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let directions: Vec<Direction> = iproduct!(-1..=1, -1..=1)
        .filter(|d| *d != (0, 0))
        .map(|(y, x)| Direction { y, x })
        .collect();
    let mut occourences = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            for direction in &directions {
                if peek(&matrix, &mut "XMAS".chars(), &Position { y, x }, direction) {
                    occourences += 1;
                }
            }
        }
    }

    Some(occourences)
}

pub fn peek(
    matrix: &Vec<Vec<char>>,
    search: &mut Chars,
    position: &Position,
    direction: &Direction,
) -> bool {
    let char = search.next();
    match char {
        None => true,
        Some(char) => {
            if position.y >= matrix.len() || position.x >= matrix[position.y].len() {
                return false;
            }

            if matrix[position.y][position.x] != char {
                return false;
            }

            let position = Position {
                x: position.x.wrapping_add_signed(direction.x),
                y: position.y.wrapping_add_signed(direction.y),
            };
            peek(&matrix, search, &position, direction)
        }
    }
}
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Direction {
    x: isize,
    y: isize,
}

impl Add<&Direction> for Position {
    type Output = Self;

    fn add(self, rhs: &Direction) -> Self::Output {
        Position {
            x: self.x.wrapping_add_signed(rhs.x),
            y: self.y.wrapping_add_signed(rhs.y),
        }
    }
}

impl Sub<&Direction> for Position {
    type Output = Self;

    fn sub(self, rhs: &Direction) -> Self::Output {
        Position {
            x: self.x.wrapping_add_signed(-rhs.x),
            y: self.y.wrapping_add_signed(-rhs.y),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let directions: Vec<Direction> = iproduct!(vec![-1, 1], vec![-1, 1])
        .map(|(y, x)| Direction { y, x })
        .collect();
    let mut occourences = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let matches: Vec<bool> = directions
                .iter()
                .map(|direction| {
                    peek(
                        &matrix,
                        &mut "MAS".chars(),
                        &(Position { x, y } - direction),
                        direction,
                    )
                })
                .collect();
            if (matches[0] || matches[3]) && (matches[1] || matches[2]) {
                occourences += 1;
            }
        }
    }

    Some(occourences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
