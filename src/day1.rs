use itertools::Itertools;

pub fn part1(instructions: &[i32]) -> i32 {
    instructions.last().unwrap().to_owned()
}

pub fn part2(instructions: &[i32]) -> i32 {
    1 + instructions.iter().find_position(|&&i| i == -1).unwrap().0 as i32
}

pub fn generator(input: &str) -> Vec<i32> {
    let mut instructions: Vec<i32> = input.lines().next().unwrap().chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!()
        })
        .collect();
    for i in 1..instructions.len() {
        instructions[i] += instructions[i-1];
    }
    instructions
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn test_generator() {
        let instructions = generator(")())())");
        assert_eq!(instructions.len(), 7);
    }

    #[test]
    fn test_part_1() {
        let instructions = generator(")())())");
        assert_eq!(part1(&instructions), -3);
    }

    #[test]
    fn test_part_2() {
        let instructions = generator("()())");
        assert_eq!(part2(&instructions), 5);
    }
}
