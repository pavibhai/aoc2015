use crate::day6::Operation::{Off, On, Toggle};
use std::ops::RangeInclusive;

pub fn part1(puzzle: &Puzzle) -> u32 {
    puzzle.follow_instructions()
}

pub fn part2(puzzle: &Puzzle) -> u32 {
    puzzle.follow_instructions_brightness()
}

pub fn generator(input: &str) -> Puzzle {
    Puzzle::from(input).expect("Unable to parse input")
}

enum Operation {
    On,
    Off,
    Toggle,
}

impl Operation {
    fn from_str(op: &str) -> Result<Self, String> {
        match op {
            "on" => Ok(On),
            "off" => Ok(Off),
            "toggle" => Ok(Toggle),
            _ => Err(format!("Unknown operation: {}", op)),
        }
    }

    fn get_fn_on_off(&self) -> fn(&mut bool) {
        match self {
            On => |s| *s = true,
            Off => |s| *s = false,
            Toggle => |s| *s ^= true,
        }
    }

    fn get_fn_brightness(&self) -> fn(&mut u32) {
        match self {
            On => |b| *b += 1,
            Off => |b| if *b > 0 {*b -= 1},
            Toggle => |b| *b += 2,
        }
    }
}

struct Instruction {
    op: Operation,
    x: RangeInclusive<u32>,
    y: RangeInclusive<u32>,
}

impl Instruction {
    fn from_str(instruction: &str) -> Result<Self, String> {
        let (prefix, coord2) = instruction
            .split_once(" through ")
            .ok_or("Unable to parse input")?;
        let (x2, y2) = coord2.split_once(",").ok_or("Unable to parse input")?;
        let mut splits = prefix.split_whitespace().rev();
        let (x1, y1) = splits
            .next()
            .ok_or("Unable to parse input")?
            .split_once(',')
            .ok_or("Unable to parse input")?;
        let x1 = x1.parse::<u32>().map_err(|e| e.to_string())?;
        let x2 = x2.parse::<u32>().map_err(|e| e.to_string())?;
        let y1 = y1.parse::<u32>().map_err(|e| e.to_string())?;
        let y2 = y2.parse::<u32>().map_err(|e| e.to_string())?;

        Ok(Instruction {
            op: splits
                .next()
                .map(Operation::from_str)
                .ok_or("Unable to parse input")??,
            x: x1.min(x2)..=x1.max(x2),
            y: y1.min(y2)..=y1.max(y2),
        })
    }

    fn apply<T>(&self, grid: &mut Vec<Vec<T>>, action: fn(&Operation) -> fn(&mut T)) {
        for y in self.y.clone() {
            for x in self.x.clone() {
                action(&self.op)(&mut grid[y as usize][x as usize]);
            }
        }
    }
}

pub struct Puzzle {
    instructions: Vec<Instruction>,
}

impl Puzzle {
    fn from(input: &str) -> Result<Self, String> {
        let instructions = input
            .lines()
            .map(|line| Instruction::from_str(line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Puzzle { instructions })
    }

    fn follow_instructions(&self) -> u32 {
        let mut grid = vec![vec![false; 1000]; 1000];
        self.instructions.iter().for_each(|instruction| {
            instruction.apply(&mut grid, Operation::get_fn_on_off)
        });
        grid.iter().flatten().filter(|&&v| v).count() as u32
    }

    fn follow_instructions_brightness(&self) -> u32 {
        let mut grid = vec![vec![0u32; 1000]; 1000];
        self.instructions.iter().for_each(|instruction| {
            instruction.apply(&mut grid, Operation::get_fn_brightness)
        });
        grid.iter().flatten().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};
    const INPUT: &str = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";

    #[test]
    fn test_generator() {
        let p = generator(INPUT);
        assert_eq!(3, p.instructions.len());
    }

    #[test]
    fn test_part_1() {
        let p = generator(INPUT);
        assert_eq!(part1(&p), 1000000 - 1000 - 4);
    }

    #[test]
    fn test_part_2() {
        let p = generator("turn on 0,0 through 0,0
toggle 0,0 through 999,999");
        assert_eq!(part2(&p), 2000001);
    }
}
