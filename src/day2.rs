pub fn part1(presents: &[Vec<u32>]) -> u32 {
    presents
        .iter()
        .map(|p| {
            (p.iter().sum::<u32>().pow(2)) + (p.iter().product::<u32>() / p.iter().max().unwrap())
                - p.iter().map(|s| s * s).sum::<u32>()
        })
        .sum()
}

pub fn part2(presents: &[Vec<u32>]) -> u32 {
    presents
        .iter()
        .map(|p| {
            let max = p.iter().max().unwrap();
            2 * (p.iter().sum::<u32>() - max) + p.iter().product::<u32>()
        })
        .sum()
}

pub fn generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.split("x").map(|l| l.parse::<u32>().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    #[test]
    fn test_generator() {
        let presents = generator("2x3x4");
        assert_eq!(presents.len(), 1);
    }

    #[test]
    fn test_part_1() {
        let presents = generator("2x3x4");
        assert_eq!(part1(&presents), 58);
    }

    #[test]
    fn test_part_2() {
        let presents = generator("2x3x4");
        assert_eq!(part2(&presents), 34);
    }
}
