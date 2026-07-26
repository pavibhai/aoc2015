use itertools::Itertools;

pub fn part1(directions: &[Direction]) -> u32 {
    let mut visited = vec![(0, 0)];
    for direction in directions {
        visited.push(direction.apply(visited.last().unwrap()))
    }
    visited.iter().unique().count() as u32
}

pub fn part2(directions: &[Direction]) -> u32 {
    let mut visited = vec![(0, 0), (0, 0)];
    for direction in directions {
        visited.push(direction.apply(&visited[visited.len() - 2]))
    }
    visited.iter().unique().count() as u32
}

pub fn generator(input: &str) -> Vec<Direction> {
    _generator(input).expect("Unable to parse input")
}

fn _generator(input: &str) -> Result<Vec<Direction>, String> {
    input.lines()
        .next()
        .ok_or_else(|| "Empty input".to_string())?
        .chars()
        .map(|c| match c {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(format!("Unexpected character: {c}").to_string()),
        })
        .collect::<Result<Vec<Direction>, String>>()
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, loc: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::Left => (loc.0 - 1, loc.1),
            Direction::Right => (loc.0 + 1, loc.1),
            Direction::Up => (loc.0, loc.1 - 1),
            Direction::Down => (loc.0, loc.1 + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn test_generator() {
        let directions = generator("^>v<");
        assert_eq!(directions.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let directions = generator("^>v<");
        assert_eq!(part1(&directions), 4);

        let directions = generator(">");
        assert_eq!(part1(&directions), 2);
    }

    #[test]
    fn test_part_2() {
        let directions = generator("^v");
        assert_eq!(part2(&directions), 3);

        let directions = generator("^v^v^v^v^v");
        assert_eq!(part2(&directions), 11);
    }
}
