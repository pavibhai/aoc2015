use crate::day8::Char::{DQuote, Other, Slash, X};

pub fn part1(list: &[Vec<Char>]) -> u32 {
    list.iter()
        .map(|line| {
            line.iter().fold((0u32, false), |(char_reduction, is_escaped), c| {
                match c {
                    Slash if is_escaped => (char_reduction + 1, false),
                    Slash => (char_reduction, true),
                    DQuote => (char_reduction + 1, false),
                    X if is_escaped => (char_reduction + 3, false),
                    _ => (char_reduction, false),
                }
            }).0
        })
        .sum()
}

#[derive(PartialEq, Clone)]
pub enum Char {
    DQuote,
    Slash,
    X,
    Other
}

pub fn part2(list: &[Vec<Char>]) -> u32 {
    list.iter()
        .map(|line| {
            line.iter().filter(|x| **x == Slash || **x == DQuote).count() + 2
        }).sum::<usize>() as u32
}

pub fn generator(input: &str) -> Vec<Vec<Char>> {
    input.lines().map(|l| {
        l.chars().map(|c| match c {
            '\\' => Slash,
            '"' => DQuote,
            'x' => X,
            _ => Other
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
";

    #[test]
    fn test_generator() {
        let input = generator(INPUT);
        assert_eq!(input.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_part_2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 19);
    }
}
