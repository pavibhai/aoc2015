use itertools::Itertools;
use std::mem::swap;
const DIGIT_0: char = '0';

pub fn generator(input: &str) -> LookSay {
    LookSay::from(input)
}

pub fn part1(look_say: &LookSay) -> u64 {
    let mut look_say = look_say.clone();
    look_say.next(40);
    look_say.seed.len() as u64
}

pub fn part2(look_say: &LookSay) -> u64 {
    let mut look_say = look_say.clone();
    look_say.next(50);
    look_say.seed.len() as u64
}

#[derive(Debug, Clone)]
pub struct LookSay {
    seed: Vec<char>,
}

impl LookSay {
    fn from(input: &str) -> LookSay {
        let seed = input.trim().chars().collect_vec();
        LookSay { seed }
    }

    fn next(&mut self, times: usize) {
        let mut buffer = Vec::new();
        let mut digits: Vec<char> = Vec::new();
        for _ in 0..times {
            buffer.clear();
            for (mut cnt, digit) in self.seed.iter().dedup_with_count() {
                if cnt > 9 {
                    while cnt > 0 {
                        digits.push((DIGIT_0 as u8 + (cnt % 10) as u8) as char);
                        cnt /= 10;
                    }
                    for _ in 0..digits.len() {
                        buffer.push(digits.pop().unwrap());
                    }
                } else {
                    buffer.push((DIGIT_0 as u8 + cnt as u8) as char);
                }
                buffer.push(*digit);
            }
            swap(&mut buffer, &mut self.seed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::generator;

    #[test]
    fn test_generator() {
        let ls = generator("211");
        assert_eq!(ls.seed.len(), 3);
    }

    #[test]
    fn test_part1() {
        let mut ls = generator("1");
        ls.next(1);
        assert_eq!(ls.seed, vec!['1', '1']);
        ls.next(1);
        assert_eq!(ls.seed, vec!['2', '1']);
        ls.next(1);
        assert_eq!(ls.seed, vec!['1', '2', '1', '1']);
        ls.next(1);
        assert_eq!(ls.seed, vec!['1', '1', '1', '2', '2', '1']);
        ls.next(1);
        assert_eq!(ls.seed, vec!['3', '1', '2', '2', '1', '1']);
    }
}
