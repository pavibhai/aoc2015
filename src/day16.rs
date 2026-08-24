pub fn generator(input: &str) -> Puzzle {
    Puzzle::parse(input).unwrap()
}

pub fn part1(puzzle: &Puzzle) -> u32 {
    puzzle.find_aunt()
}

pub fn part2(puzzle: &Puzzle) -> u32 {
    puzzle.find_aunt_retro()
}

pub struct Puzzle {
    d_aunt: Aunt,
    r_aunts: Vec<Aunt>,
}

impl Puzzle {
    fn parse(input: &str) -> Result<Self, String> {
        let r_aunts = input.lines()
            .map(Aunt::parse)
            .collect::<Result<Vec<Aunt>, String>>()?;
        Ok(
            Puzzle {
                d_aunt: Puzzle::detected_aunt(),
                r_aunts,
            }
        )
    }

    fn detected_aunt() -> Aunt {
        Aunt {
            id: None,
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
        }
    }

    fn find_aunt(&self) -> u32 {
        let aunts = self.r_aunts.iter()
            .filter(|a| {
                (a.children.is_none() || a.children == self.d_aunt.children)
                && (a.cats.is_none() || a.cats == self.d_aunt.cats)
                && (a.samoyeds.is_none() || a.samoyeds == self.d_aunt.samoyeds)
                && (a.pomeranians.is_none() || a.pomeranians == self.d_aunt.pomeranians)
                && (a.akitas.is_none() || a.akitas == self.d_aunt.akitas)
                && (a.vizslas.is_none() || a.vizslas == self.d_aunt.vizslas)
                && (a.goldfish.is_none() || a.goldfish == self.d_aunt.goldfish)
                && (a.trees.is_none() || a.trees == self.d_aunt.trees)
                && (a.cars.is_none() || a.cars == self.d_aunt.cars)
                && (a.perfumes.is_none() || a.perfumes == self.d_aunt.perfumes)
            }).collect::<Vec<_>>();
        assert_eq!(aunts.len(), 1);
        aunts.first().unwrap().id.unwrap()
    }

    fn find_aunt_retro(&self) -> u32 {
        let aunts = self.r_aunts.iter()
            .filter(|a| {
                (a.children.is_none() || a.children == self.d_aunt.children)
                    && (a.cats.is_none() || a.cats > self.d_aunt.cats)
                    && (a.samoyeds.is_none() || a.samoyeds == self.d_aunt.samoyeds)
                    && (a.pomeranians.is_none() || a.pomeranians < self.d_aunt.pomeranians)
                    && (a.akitas.is_none() || a.akitas == self.d_aunt.akitas)
                    && (a.vizslas.is_none() || a.vizslas == self.d_aunt.vizslas)
                    && (a.goldfish.is_none() || a.goldfish < self.d_aunt.goldfish)
                    && (a.trees.is_none() || a.trees > self.d_aunt.trees)
                    && (a.cars.is_none() || a.cars == self.d_aunt.cars)
                    && (a.perfumes.is_none() || a.perfumes == self.d_aunt.perfumes)
            }).collect::<Vec<_>>();
        assert_eq!(aunts.len(), 1);
        aunts.first().unwrap().id.unwrap()
    }
}

struct Aunt {
    id: Option<u32>,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Aunt {
    fn parse(input: &str) -> Result<Self, String> {
        let mut splits = input.split_whitespace();
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        splits.next();
        let id = splits
            .next()
            .and_then(|s| s.strip_suffix(':').and_then(|s| s.parse::<u32>().ok()));

        loop {
            if let Some(attr) = splits.next() {
                let value = splits
                    .next()
                    .and_then(|s| {
                        s.strip_suffix(',')
                            .or(Some(s))
                            .and_then(|s| s.parse::<u32>().ok())
                    })
                    .ok_or(format!("Invalid number: {}", input))?;
                match attr {
                    "children:" => children = Some(value),
                    "cats:" => cats = Some(value),
                    "samoyeds:" => samoyeds = Some(value),
                    "pomeranians:" => pomeranians = Some(value),
                    "akitas:" => akitas = Some(value),
                    "vizslas:" => vizslas = Some(value),
                    "goldfish:" => goldfish = Some(value),
                    "trees:" => trees = Some(value),
                    "cars:" => cars = Some(value),
                    "perfumes:" => perfumes = Some(value),
                    _ => Err(format!("Invalid attribute: {}", attr))?,
                }
            } else {
                break;
            }
        }

        Ok(Aunt {
            id,
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sue 1: children: 1, cars: 8, vizslas: 7
Sue 2: akitas: 10, perfumes: 10, children: 5
Sue 213: children: 3, goldfish: 5, vizslas: 0
Sue 323: perfumes: 1, trees: 6, goldfish: 0";

    #[test]
    fn test_generator() {
        let p = generator(INPUT);
        assert_eq!(4, p.r_aunts.len());
    }

    #[test]
    fn test_part1() {
        let p = generator(INPUT);
        assert_eq!(213, part1(&p));
    }

    #[test]
    fn test_part2() {
        let p = generator(INPUT);
        assert_eq!(323, part2(&p));
    }
}
