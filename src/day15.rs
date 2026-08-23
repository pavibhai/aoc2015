const MAX_INGREDIENTS: u32 = 100;

pub fn generator(input: &str) -> Recipe {
    Recipe::parse(input).unwrap()
}

pub fn part1(recipe: &Recipe) -> u32 {
    recipe.find_best_score(None)
}

pub fn part2(recipe: &Recipe) -> u32 {
    recipe.find_best_score(Some(500))
}

pub struct Recipe {
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    fn parse(input: &str) -> Result<Self, String> {
        let ingredients = input
            .lines()
            .map(Ingredient::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { ingredients })
    }

    fn find_best_score(&self, calories: Option<u32>) -> u32 {
        let mut combinations = vec![0; self.ingredients.len()];
        let mut best_score = 0;
        self._mix(0, &mut combinations, &mut best_score, calories);
        best_score
    }

    fn _mix(
        &self,
        idx: usize,
        combinations: &mut Vec<u32>,
        best_score: &mut u32,
        calories: Option<u32>,
    ) {
        let remaining = MAX_INGREDIENTS - combinations[0..idx].iter().sum::<u32>();
        if idx == self.ingredients.len() - 1 {
            combinations[self.ingredients.len() - 1] = remaining;
            *best_score = self._score(combinations, calories).max(*best_score);
        } else {
            for t in 0..remaining {
                combinations[idx] = t;
                self._mix(idx + 1, combinations, best_score, calories);
            }
        }
    }

    fn _score(&self, combinations: &[u32], calories: Option<u32>) -> u32 {
        let (c, d, f, t, recipe_calories) = self.ingredients.iter().zip(combinations.iter()).fold(
            (0, 0, 0, 0, 0),
            |(c, d, f, t, calories), (i, times)| {
                (
                    c + i.capacity * *times as i32,
                    d + i.durability * *times as i32,
                    f + i.flavor * *times as i32,
                    t + i.texture * *times as i32,
                    calories + i.calories * *times,
                )
            },
        );
        if c < 0 || d < 0 || f < 0 || t < 0 || (calories.is_some() && calories.unwrap() != recipe_calories) {
            0
        } else {
            (c * d * f * t) as u32
        }
    }
}

struct Ingredient {
    _name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

impl Ingredient {
    fn parse(input: &str) -> Result<Self, String> {
        let mut splits = input.split_whitespace();

        let name = splits
            .next()
            .and_then(|s| s.strip_suffix(':'))
            .ok_or("Missing name")?
            .to_string();
        let mut next_as = |value: &str| -> Result<i32, String> {
            assert_eq!(splits.next(), Some(value));
            splits
                .next()
                .and_then(|s| s.strip_suffix(',').or(Some(s)))
                .and_then(|s| s.parse::<i32>().ok())
                .ok_or(format!("Missing {value}"))
        };
        let capacity = next_as("capacity")?;
        let durability = next_as("durability")?;
        let flavor = next_as("flavor")?;
        let texture = next_as("texture")?;
        let calories = next_as("calories")? as u32;

        Ok(Ingredient {
            _name: name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_generator() {
        let r = generator(INPUT);
        assert_eq!(r.ingredients.len(), 2);
    }

    #[test]
    fn test_part1() {
        let r = generator(INPUT);
        assert_eq!(part1(&r), 62842880);
    }

    #[test]
    fn test_part2() {
        let r = generator(INPUT);
        assert_eq!(part2(&r), 57600000);
    }
}
