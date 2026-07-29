use itertools::Itertools;
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice_string(s: &[char]) -> bool {
    let bad_pairs = s.windows(2).any(|pair| {
        match pair {
            ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
            _ => false,
        }
    });

    let vowel_pairs = s.windows(2).any(|pair| match pair {
        [first, second] if first == second => true,
        _ => false
    });

    !bad_pairs && vowel_pairs && s.iter().filter(|&c| VOWELS.contains(c)).count() > 2
}

pub fn part1(strings: &[Vec<char>]) -> u32 {
    strings.iter().filter(|&s| is_nice_string(s)).count() as u32
}

pub fn part2(strings: &[Vec<char>]) -> u32 {
    strings.iter()
        .filter(|&s| {
            s.windows(2).enumerate()
                .any(|(i, w)| {s[i+2..].windows(2).any(|wj| w == wj)})
            && s.windows(3).any(|triplet| triplet[0] == triplet[2])
        })
        .count() as u32
}

pub fn generator(input: &str) -> Vec<Vec<char>> {
    _generator(input).expect("Unable to parse input")
}

fn _generator(input: &str) -> Result<Vec<Vec<char>>, String> {
    Ok(input.lines().map(|l| l.chars().collect_vec()).collect_vec())
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use super::{generator, is_nice_string, part1, part2};
    const INPUT_1: &str = "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb";
    const INPUT_2: &str = "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy";

    #[test]
    fn test_generator() {
        let strings = generator(INPUT_1);
        assert_eq!(5, strings.len());
    }

    #[test]
    fn test_naughty_nice() {
        assert!(is_nice_string(&"aaa".chars().collect_vec()));
    }

    #[test]
    fn test_part_1() {
        let strings = generator(INPUT_1);
        assert_eq!(2, part1(&strings));
    }

    #[test]
    fn test_part_2() {
        let strings = generator(INPUT_2);
        assert_eq!(2, part2(&strings));
    }
}
