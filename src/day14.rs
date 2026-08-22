pub fn generator(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(Reindeer::parse)
        .collect::<Result<Vec<Reindeer>, String>>()
        .expect("failed to parse input")
}

pub fn part1(reindeers: &[Reindeer]) -> u32 {
    max_distance(reindeers, 2503)
}

pub fn part2(reindeers: &[Reindeer]) -> u32 {
    max_score(reindeers, 2503)
}

fn max_score(reindeers: &[Reindeer], secs: u32) -> u32 {
    let distances = reindeers
        .iter()
        .map(|r| r.distances_travelled(secs))
        .collect::<Vec<_>>();

    (1..secs)
        .map(|s| (s, (0..reindeers.len()).map(|r| distances[r][s as usize]).max().unwrap()))
        .fold(vec![0u32; reindeers.len()], |mut acc, (s, max_distance)| {
            (0..reindeers.len()).for_each(|r| {
                if distances[r][s as usize] == max_distance {
                    acc[r] += 1;
                }
            });
            acc
        })
        .iter().max().unwrap().clone()
}

fn max_distance(reindeers: &[Reindeer], secs: u32) -> u32 {
    reindeers
        .iter()
        .map(|r| r.distance_travelled(secs))
        .max()
        .unwrap()
}

pub struct Reindeer {
    _name: String,
    speed_in_sec: u32,
    fly_seconds: u32,
    rest_seconds: u32,
}

impl Reindeer {
    fn parse(input: &str) -> Result<Reindeer, String> {
        let (split1, rest_seconds) = input
            .split_once(" seconds, but then must rest for ")
            .ok_or(format!("Cannot parse {}", input))?;
        let mut split1 = split1.split_whitespace();
        let name = split1.next().unwrap().to_string();
        split1.next();
        split1.next();
        let speed_in_sec = split1.next().unwrap().parse().unwrap();
        split1.next();
        split1.next();
        let fly_seconds = split1.next().unwrap().parse().unwrap();
        let (rest_seconds, _) = rest_seconds
            .split_once(' ')
            .ok_or(format!("Cannot parse {}", input))?;

        Ok(Reindeer {
            _name: name,
            speed_in_sec,
            fly_seconds,
            rest_seconds: rest_seconds
                .parse()
                .map_err(|_| format!("Cannot parse {}", input))?,
        })
    }

    fn distance_travelled(&self, secs: u32) -> u32 {
        secs / (self.fly_seconds + self.rest_seconds) * self.fly_seconds * self.speed_in_sec
            + (secs % (self.fly_seconds + self.rest_seconds)).min(self.fly_seconds)
                * self.speed_in_sec
    }

    fn distances_travelled(&self, secs: u32) -> Vec<u32> {
        (0..secs).map(|s| self.distance_travelled(s)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_generator() {
        let deers = generator(INPUT);
        assert_eq!(deers.len(), 2);
        assert_eq!(deers[0].speed_in_sec, 14);
        assert_eq!(deers[0].fly_seconds, 10);
        assert_eq!(deers[0].rest_seconds, 127);
        assert_eq!(deers[1].speed_in_sec, 16);
        assert_eq!(deers[1].fly_seconds, 11);
        assert_eq!(deers[1].rest_seconds, 162);
    }

    #[test]
    fn test_part1() {
        let deers = generator(INPUT);
        assert_eq!(max_distance(&deers, 1000), 1120);
    }

    #[test]
    fn test_part2() {
        let deers = generator(INPUT);
        assert_eq!(max_score(&deers, 1000), 689);
    }
}
