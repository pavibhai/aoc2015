use itertools::Itertools;

pub fn part1(presents: &u32) -> u32 {
    let mut delivery = Delivery::new(10, |_, _| true);
    find_minimum_house(presents, &mut delivery)
}

pub fn part2(presents: &u32) -> u32 {
    let mut delivery = Delivery::new(11, |n, f| n / f < 51);
    find_minimum_house(presents, &mut delivery)
}

fn find_minimum_house(presents: &u32, delivery: &mut Delivery) -> u32 {
    let mut house_num = 0;
    loop {
        house_num += 1;
        if delivery.presents(house_num) >= *presents {
            return house_num;
        }
    }
}

pub fn generator(input: &str) -> u32 {
    _generator(input).expect("Unable to parse input")
}

fn _generator(input: &str) -> Result<u32, String> {
    let presents: u32 = input
        .trim()
        .parse()
        .map_err(|_| "Failed to parse input".to_string())?;

    Ok(presents)
}

struct Delivery {
    present_multiplier: u32,
    // Excludes 1 and the number
    factors: Vec<Vec<u32>>,
    limiter: fn(u32, u32) -> bool,
}

impl Delivery {
    fn new(present_multiplier: u32, limiter: fn(u32, u32) -> bool) -> Delivery {
        Delivery {
            present_multiplier,
            factors: vec![Vec::new()],
            limiter,
        }
    }

    fn presents(&mut self, house_num: u32) -> u32 {
        for n in self.factors.len() as u32..=house_num {
            self._find_factors(n);
        }
        self.present_multiplier
            * self.factors[house_num as usize]
                .iter()
                .filter(|&x| (self.limiter)(house_num, *x))
                .sum::<u32>()
    }

    fn _find_factors(&mut self, num: u32) {
        assert_eq!(num, self.factors.len() as u32);
        let mut curr = 2u32;
        let mut factors = vec![1u32, num];

        while curr * curr <= num {
            if num % curr == 0 {
                for (i, j) in self.factors[curr as usize]
                    .iter()
                    .cartesian_product(self.factors[(num / curr) as usize].iter())
                {
                    factors.push(*i * *j);
                }
                break;
            }
            curr += 1;
        }
        factors.sort();
        factors.dedup();
        self.factors.push(factors);
    }
}

#[cfg(test)]
mod tests {
    use super::{find_minimum_house, generator, part1, Delivery};

    const INPUT: &str = "130";

    #[test]
    fn test_generator() {
        let presents = generator(INPUT);
        assert_eq!(130, presents);
    }

    #[test]
    fn test_part_1() {
        let presents = generator(INPUT);
        assert_eq!(8, part1(&presents));
        assert_eq!(6, part1(&119));
        assert_eq!(4, part1(&70));
        assert_eq!(30, part1(&720));
    }

    #[test]
    fn test_part_2() {
        let presents = generator(INPUT);
        assert_eq!(
            8,
            find_minimum_house(&presents, &mut Delivery::new(11, |n, f| n / f < 3))
        );
    }
}
