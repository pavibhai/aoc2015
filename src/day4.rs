use md5::{Md5, Digest};

const CHAR_0: u8 = '0' as u8;

fn mine_coin<F>(secret: &str, validate: F) -> u32
where F: Fn(&[u8]) -> bool {
    let mut hasher = Md5::new();
    let mut curr = 1u32;
    let mut bytes: Vec<u8> = secret.as_bytes().to_vec();
    let secret_size = bytes.len();
    for _ in 0..100 {bytes.push(0);}
    loop {
        let digits = (curr.ilog10() + 1) as usize;
        let mut v = curr;
        for idx in (0..digits).rev() {
            bytes[secret_size + idx] = (v % 10) as u8 + CHAR_0;
            v /= 10;
        }
        hasher.update(&bytes[..secret_size+digits]);
        match hasher.finalize_reset() {
            b if validate(&b) => {
                return curr
            },
            _ => {}
        }
        curr += 1;
    }
}

fn validate_5_leading_zeros(b: &[u8]) -> bool {
    b[0] == 0 && b[1] == 0 && b[2] < 16
}

fn validate_6_leading_zeros(b: &[u8]) -> bool {
    b[0] == 0 && b[1] == 0 && b[2] == 0
}

pub fn part1(secret: &str) -> u32 {
    mine_coin(secret, validate_5_leading_zeros)
}

pub fn part2(secret: &str) -> u32 {
    mine_coin(secret, validate_6_leading_zeros)
}

pub fn generator(input: &str) -> String {
    _generator(input).expect("Unable to parse input")
}

fn _generator(input: &str) -> Result<String, String> {
    input
        .lines()
        .next()
        .map(|l| l.to_string())
        .ok_or("Unable to parse input".to_string())
}

#[cfg(test)]
mod tests {
    use super::{generator, part1};

    #[test]
    fn test_generator() {
        let secret = generator("abcdef");
        assert_eq!(secret, "abcdef");
    }

    #[test]
    fn test_part_1() {
        assert_eq!(609043, part1("abcdef"));
    }
}
