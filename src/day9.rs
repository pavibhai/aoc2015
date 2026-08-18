use std::collections::HashMap;

pub fn part1(locations: &Locations) -> u32 {
    locations.min_dist.unwrap()
}

pub fn part2(locations: &Locations) -> u32 {
    locations.max_dist.unwrap()
}

pub fn generator(input: &str) -> Locations {
    let mut l = Locations::from(input).unwrap();
    l.shortest_longest();
    l
}

pub struct Locations {
    ids: HashMap<String, usize>,
    distances: HashMap<(usize, usize), u32>,
    min_dist: Option<u32>,
    max_dist: Option<u32>,
}

impl Locations {
    fn from(input: &str) -> Result<Locations, String> {
        let mut ids = HashMap::new();
        let distances: HashMap<(usize, usize), u32> = input
            .lines()
            .map(|l| {
                let mut splits = l.split_whitespace();
                let loc1 = splits.next().ok_or("Location not found in {}")?;
                let idx = ids.len();
                let loc1 = *ids.entry(loc1.to_string()).or_insert(idx);
                splits.next();
                let loc2 = splits.next().ok_or("Location not found in {}")?;
                let idx = ids.len();
                let loc2 = *ids.entry(loc2.to_string()).or_insert(idx);
                splits.next();
                let dist = splits
                    .next()
                    .ok_or(format!("Distance not found in {}", l))?
                    .parse::<u32>()
                    .map_err(|_| "cannot parse distance")?;
                Ok::<((usize, usize), u32), String>(((loc1.min(loc2), loc1.max(loc2)), dist))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Locations { ids, distances, min_dist: None, max_dist: None })
    }

    fn distance(&self, loc1: &usize, loc2: &usize) -> u32 {
        if loc1 == loc2 {
            0
        } else if loc1 < loc2 {
            self.distances[&(*loc1, *loc2)]
        } else {
            self.distances[&(*loc2, *loc1)]
        }
    }

    fn recurse(
        &self,
        seen: &mut Vec<usize>,
        curr_dist: u32,
        min_dist: &mut u32,
        max_dist: &mut u32,
    ) {
        if seen.len() == self.ids.len() {
            if curr_dist < *min_dist {
                *min_dist = curr_dist;
            }
            if curr_dist > *max_dist {
                *max_dist = curr_dist;
            }
        } else {
            for next in 0..self.ids.len() {
                if !seen.contains(&next) {
                    let d = seen
                        .last()
                        .map(|last| self.distance(last, &next))
                        .unwrap_or(0);
                    seen.push(next);
                    self.recurse(seen, curr_dist + d, min_dist, max_dist);
                    seen.pop();
                }
            }
        }
    }

    fn shortest_longest(&mut self) {
        let mut seen = Vec::with_capacity(self.ids.len());
        let mut min_dist = u32::MAX;
        let mut max_dist = u32::MIN;
        self.recurse(&mut seen, 0, &mut min_dist, &mut max_dist);
        self.min_dist = Some(min_dist);
        self.max_dist = Some(max_dist);
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_generator() {
        let locations = generator(INPUT);
        assert_eq!(locations.ids.len(), 3);
        assert_eq!(locations.distances.len(), 3);
        assert_eq!(locations.distances.values().sum::<u32>(), 1123);
    }

    #[test]
    fn test_part_1() {
        let l = generator(INPUT);
        assert_eq!(part1(&l), 605);
    }

    #[test]
    fn test_part_2() {
        let l = generator(INPUT);
        assert_eq!(part2(&l), 982);
    }
}
