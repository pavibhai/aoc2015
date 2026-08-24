pub fn generator(input: &str) -> Refrigerator {
    Refrigerator::parse(input).unwrap()
}

pub fn part1(r: &Refrigerator) -> u32 {
    r.fill(150).iter().sum()
}

pub fn part2(r: &Refrigerator) -> u32 {
    *r.fill(150).iter().filter(|&v| *v != 0).next().unwrap()
}

pub struct Refrigerator {
    containers: Vec<u32>,
}

impl Refrigerator {
    fn parse(input: &str) -> Result<Self, String> {
        let containers = input
            .lines()
            .map(|l| l.parse::<u32>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<u32>, String>>()?;
        Ok(Self { containers })
    }

    fn fill(&self, eggnog: u32) -> Vec<u32> {
        let mut counts = vec![0; self.containers.len()];
        let mut combination = Vec::new();
        self._fill(eggnog, &mut counts, &mut combination);
        counts
    }

    fn _fill(&self, eggnog: u32, counts: &mut Vec<u32>, combination: &mut Vec<usize>) {
        let start_idx = combination.last().map_or(0, |i| *i + 1);
        for (idx, s) in self.containers[start_idx..]
            .iter()
            .enumerate()
            .filter(|(_, s)| **s <= eggnog)
        {
            combination.push(start_idx + idx);
            if *s == eggnog {
                counts[combination.len() - 1] += 1;
            } else {
                self._fill(eggnog - *s, counts, combination);
            }
            combination.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "20
15
10
5
5";

    #[test]
    fn test_generator() {
        let r = generator(INPUT);
        assert_eq!(5, r.containers.len());
    }

    #[test]
    fn test_part1() {
        let r = generator(INPUT);
        assert_eq!(4, r.fill(25).iter().sum::<u32>());
    }

    #[test]
    fn test_part2() {
        let r = generator(INPUT);
        assert_eq!(3, *r.fill(25).iter().filter(|&v| *v != 0).next().unwrap());
    }
}
