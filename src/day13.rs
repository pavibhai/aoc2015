use itertools::Itertools;
use std::collections::HashMap;

pub fn generator(input: &str) -> Guests {
    Guests::parse(input).unwrap()
}

pub fn part1(guests: &Guests) -> i32 {
    guests.seat_optimally(false)
}

pub fn part2(guests: &Guests) -> i32 {
    guests.seat_optimally(true)
}

pub struct Guests {
    ids: HashMap<String, usize>,
    happiness: Vec<Vec<i32>>,
}

impl Guests {
    fn parse(input: &str) -> Result<Guests, String> {
        let mut ids = HashMap::new();
        let mut happiness = Vec::new();

        for line in input.lines() {
            let (first, second) = line
                .split_once(" happiness units by sitting next to ")
                .ok_or(format!("Unable to parse {line}"))?;
            let mut first = first.split_whitespace();
            let second_person = second.strip_suffix('.').ok_or("Missing second person")?;
            let first_person = first.next().ok_or("Missing first person")?;
            first.next();
            let sign = if first.next().ok_or("Missing sign")? == "gain" {
                1
            } else {
                -1
            };
            let number = first.next().ok_or("Missing number")?
                .parse::<u32>()
                .map_err(|_| format!("Invalid number: {}", first_person))?;
            let ids_len = ids.len();
            let first_id = *ids.entry(first_person.to_string()).or_insert(ids_len);
            let ids_len = ids.len();
            let second_id = *ids.entry(second_person.to_string()).or_insert(ids_len);
            let happiness_len = happiness.len();
            if happiness_len < ids.len() {
                happiness.iter_mut().for_each(|row: &mut Vec<i32>| {
                    for _ in 0..ids.len() - happiness_len {
                        row.push(0)
                    }
                });
                for _ in 0..ids.len() - happiness.len() {
                    happiness.push(vec![0; ids.len()]);
                }
            }
            happiness[first_id][second_id] = sign * number as i32;
        }

        Ok(Guests { ids, happiness })
    }

    fn _guest_id(&self, name: &str) -> Option<usize> {
        self.ids.get(name).copied()
    }

    fn seat_optimally(&self, include_host: bool) -> i32 {
        let start_idx: usize = if include_host { 0 } else { 1 };
        (start_idx..self.ids.len())
            .permutations(self.ids.len() - start_idx)
            .map(|p| {
                let circular_value = if include_host {
                    0
                } else {
                    self.happiness[0][*p.first().unwrap()]
                        + self.happiness[*p.first().unwrap()][0]
                        + self.happiness[0][*p.last().unwrap()]
                        + self.happiness[*p.last().unwrap()][0]
                };
                p.windows(2)
                    .map(|w| self.happiness[w[0]][w[1]] + self.happiness[w[1]][w[0]])
                    .sum::<i32>()
                    + circular_value
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_generator() {
        let g = generator(INPUT);
        assert_eq!(g.ids.len(), 4);
        assert_eq!(g.happiness.len(), 4);
        assert_eq!(g.happiness[0].len(), 4);
        assert_eq!(g.happiness[0][0], 0);
        assert_eq!(
            g.happiness[g._guest_id("Alice").unwrap()][g._guest_id("David").unwrap()],
            -2
        );
    }

    #[test]
    fn test_part1() {
        let g = generator(INPUT);
        assert_eq!(part1(&g), 330);
    }

    #[test]
    fn test_part2() {
        let g = generator(INPUT);
        assert_eq!(part2(&g), 286);
    }
}
