use itertools::Itertools;

const INVALID_CHARS: [char; 3] = ['i', 'o', 'l'];
const CHAR_A: char = 'a';
const CHAR_Z: char = 'z';

pub fn generator(input: &str) -> Password {
    Password::from(input).unwrap()
}

pub fn part1(password: &Password) -> String {
    let mut password = password.clone();
    password.next();
    String::from_iter(password.value)
}

pub fn part2(password: &Password) -> String {
    let mut password = password.clone();
    password.next();
    password.next();
    String::from_iter(password.value)
}

#[derive(Debug, Clone)]
pub struct Password {
    value: [char; 8],
}

impl Password {
    fn from(input: &str) -> Result<Self, ()> {
        if input.trim().len() > 8 {
            return Err(());
        }
        let mut value = ['a'; 8];
        let chars = input.trim().chars();
        chars.enumerate().for_each(|(i, c)| {
            value[i] = c;
        });

        Ok(Password { value })
    }

    fn next(&mut self) {
        loop {
            self._next_value();
            if self._is_valid() {
                break;
            }
        }
    }

    fn _has_valid_chars(&self) -> bool {
        !self.value.iter().any(|&c| INVALID_CHARS.contains(&c))
    }

    fn _has_triplet(&self) -> bool {
        self
            .value
            .windows(3)
            .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8)
    }

    fn _has_non_overlapping_pairs(&self) -> bool {
        self
            .value
            .windows(2)
            .filter_map(|e| if e[0] == e[1] { Some(e[0]) } else { None })
            .dedup()
            .count()
            > 1
    }

    fn _is_valid(&self) -> bool {
        self._has_valid_chars()
            && self._has_triplet()
            && self._has_non_overlapping_pairs()
    }

    fn _next_valid_value(&mut self) {
        match self.value.iter().find_position(|c| INVALID_CHARS.contains(c)) {
            Some((idx, _)) => {
                while INVALID_CHARS.contains(&self.value[idx]) {
                    self.value[idx] = (self.value[idx] as u8 + 1) as char;
                }
                self.value[idx+1..].iter_mut().for_each(|c| *c = CHAR_A);
            }
            None => {}
        }
    }

    fn _next_value(&mut self) {
        for idx in (0..self.value.len()).rev() {
            match self.value[idx] {
                CHAR_Z => self.value[idx] = CHAR_A,
                _ => {
                    self.value[idx] = (self.value[idx] as u8 + 1) as char;
                    break;
                }
            }
        }
        self._next_valid_value();
    }
}

#[cfg(test)]
mod tests {
    use super::generator;

    #[test]
    fn test_generator() {
        let p = generator("abcdffaa");
        assert_eq!(p.value.len(), 8);
        assert_eq!(p.value, ['a', 'b', 'c', 'd', 'f', 'f', 'a', 'a']);

        assert!(p._has_triplet());
        assert!(p._has_valid_chars());
        assert!(p._has_non_overlapping_pairs());
    }

    #[test]
    fn test_part1() {
        let mut p = generator("abcdefgh");
        p.next();
        assert_eq!(String::from_iter(p.value), "abcdffaa");

        let mut p = generator("ghijklmn");
        p.next();
        assert_eq!(String::from_iter(p.value), "ghjaabcc");
    }

    #[test]
    fn test_next_valid_value() {
        let mut p = generator("hepxxozz");
        p._next_valid_value();
        assert_eq!(String::from_iter(p.value), "hepxxpaa");
        p._next_valid_value();
        assert_eq!(String::from_iter(p.value), "hepxxpaa");
    }
}
