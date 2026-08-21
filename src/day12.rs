use json::JsonValue;

pub fn generator(input: &str) -> JsonValue {
    json::parse(input).unwrap()
}

pub fn part1(doc: &JsonValue) -> i32 {
    _cumulate_numbers(doc, false)
}

pub fn part2(doc: &JsonValue) -> i32 {
    _cumulate_numbers(doc, true)
}

fn _cumulate_numbers(doc: &JsonValue, ignore_red: bool) -> i32 {
    match doc {
        JsonValue::Object(obj) if ignore_red
            && obj.iter().find(|(_, v)| *v == "red").is_some() => {
            0
        }
        JsonValue::Object(obj) => obj
            .iter()
            .map(|(_, v)| _cumulate_numbers(v, ignore_red))
            .sum(),
        JsonValue::Array(vec) => vec.iter().map(|v| _cumulate_numbers(v, ignore_red)).sum(),
        JsonValue::Number(number) => number.as_fixed_point_i64(0).unwrap() as i32,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "{\"a\":{\"b\":4, \"d\": [[[3]]]},\"c\":-1}";

    #[test]
    fn test_generator() {
        let d = generator(INPUT);
        println!("{:?}", d);
    }

    #[test]
    fn test_part1() {
        let d = generator(INPUT);
        assert_eq!(part1(&d), 6);
    }

    #[test]
    fn test_part2() {
        let d = generator(r#"[1,{"c":"red","b":2},3]"#);
        assert_eq!(part2(&d), 4);

        let d = generator(r#"{"d":"red","e":[1,2,3,4],"f":5}"#);
        assert_eq!(part2(&d), 0);

        let d = generator(r#"[1,"red",5]"#);
        assert_eq!(part2(&d), 6);
    }
}
