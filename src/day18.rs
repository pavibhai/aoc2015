use std::fmt::Display;
use std::mem::swap;

pub fn generator(input: &str) -> Grid {
    Grid::parse(input).unwrap()
}

pub fn part1(g: &Grid) -> u32 {
    let mut g = g.clone();
    g.animate(100, false);
    g.lit_lights()
}

pub fn part2(g: &Grid) -> u32 {
    let mut g = g.clone();
    g.set_stuck_lights();
    g.animate(100, true);
    g.lit_lights()
}

#[derive(Debug, Clone)]
pub struct Grid {
    lights: Vec<Vec<bool>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.lights.iter().for_each(|row| {
            row.iter().for_each(|b| s.push(if *b { '#' } else { '.' }));
            s.push('\n');
        });
        write!(f, "{}", s.trim())
    }
}

impl Grid {
    fn parse(input: &str) -> Result<Grid, String> {
        let lights = input
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert!(lights.iter().all(|l| l.len() == lights[0].len()));
        Ok(Grid { lights })
    }

    fn lit_neighbors(&self, x: usize, y: usize) -> u32 {
        let mut cnt: u32 = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = x as i32 + dx;
                let y = y as i32 + dy;
                if x >= 0
                    && x < self.lights[0].len() as i32
                    && y >= 0
                    && y < self.lights.len() as i32
                    && self.lights[y as usize][x as usize]
                {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn set_stuck_lights(&mut self) {
        let width = self.lights[0].len();
        let height = self.lights.len();
        self.lights[0][0] = true;
        self.lights[0][width - 1] = true;
        self.lights[height - 1][0] = true;
        self.lights[height - 1][width - 1] = true;
    }

    fn animate(&mut self, times: u32, stuck_lights: bool) {
        let mut buffer = self.lights.clone();
        for _ in 0..times {
            self.lights.iter().enumerate().for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, _)| {
                    buffer[y][x] = match self.lit_neighbors(x, y) {
                        3 => true,
                        2 if self.lights[y][x] => true,
                        _ => false,
                    }
                });
            });
            swap(&mut buffer, &mut self.lights);
            if stuck_lights {
                self.set_stuck_lights();
            }
        }
    }

    fn lit_lights(&self) -> u32 {
        self.lights
            .iter()
            .map(|r| r.iter().filter(|v| **v).count() as u32)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn test_generator() {
        let g = generator(INPUT);
        assert_eq!(g.lights.len(), 6);
        assert_eq!(g.lights[0].len(), 6);

        assert!(g.lights[0][1]);
    }

    #[test]
    fn test_part1() {
        let mut g = generator(INPUT);
        g.animate(1, false);
        assert_eq!(
            g.to_string(),
            "..##..
..##.#
...##.
......
#.....
#.##.."
        );
        g.animate(1, false);
        assert_eq!(
            g.to_string(),
            "..###.
......
..###.
......
.#....
.#...."
        );
        g.animate(1, false);
        assert_eq!(
            g.to_string(),
            "...#..
......
...#..
..##..
......
......"
        );
        g.animate(1, false);
        assert_eq!(
            g.to_string(),
            "......
......
..##..
..##..
......
......"
        );
    }

    #[test]
    fn test_part2() {
        let mut g = generator(INPUT);
        g.set_stuck_lights();
        assert_eq!(
            g.to_string(),
            "##.#.#
...##.
#....#
..#...
#.#..#
####.#"
        );

        g.animate(1, true);
        assert_eq!(
            g.to_string(),
            "#.##.#
####.#
...##.
......
#...#.
#.####"
        );
    }
}
