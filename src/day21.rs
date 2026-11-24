use itertools::Itertools;

pub fn generator(input: &str) -> Game {
    Game::new(input).expect("invalid input")
}

pub fn part1(g: &Game) -> u32 {
    g.find_outcomes()
        .iter()
        .filter_map(|(r, c)| if *r { Some(*c) } else { None })
        .min()
        .expect("no solution")
}

pub fn part2(g: &Game) -> u32 {
    g.find_outcomes()
        .iter()
        .filter_map(|(r, c)| if *r { None } else { Some(*c) })
        .max()
        .expect("no solution")
}

struct Item {
    _id: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new_damage_item(id: String, cost: u32, damage: u32) -> Item {
        Item {
            _id: id,
            cost,
            damage,
            armor: 0,
        }
    }

    fn new_armor_item(id: String, cost: u32, armor: u32) -> Item {
        Item {
            _id: id,
            cost,
            damage: 0,
            armor,
        }
    }
}

struct Boss {
    hit_points: u32,
    armor: u32,
    damage: u32,
}

impl Boss {
    fn parse(input: &str) -> Result<Self, String> {
        let mut lines = input.lines();
        let (attr, hit_points) = lines
            .next()
            .ok_or("Invalid input")?
            .rsplit_once(' ')
            .ok_or("Invalid hit points")?;
        assert!(attr.starts_with("Hit Points"));
        let (attr, damage) = lines
            .next()
            .ok_or("Invalid input")?
            .rsplit_once(' ')
            .ok_or("Invalid damage")?;
        assert!(attr.starts_with("Damage"));
        let (attr, armor) = lines
            .next()
            .ok_or("Invalid input")?
            .rsplit_once(' ')
            .ok_or("Invalid armor")?;
        assert!(attr.starts_with("Armor"));
        Ok(Boss {
            hit_points: hit_points
                .parse()
                .map_err(|_| format!("Invalid hit_points {}", hit_points))?,
            armor: armor
                .parse()
                .map_err(|_| format!("Invalid armor {}", armor))?,
            damage: damage
                .parse()
                .map_err(|_| format!("Invalid damage {}", damage))?,
        })
    }
}

pub struct Game {
    weapons: Vec<Item>,
    armors: Vec<Item>,
    rings: Vec<Item>,
    enemy: Boss,
    player_hit_points: u32,
}

impl Game {
    fn new(enemy: &str) -> Result<Self, String> {
        let weapons = vec![
            Item::new_damage_item("Dagger".to_string(), 8, 4),
            Item::new_damage_item("Shortsword".to_string(), 10, 5),
            Item::new_damage_item("Warhammer".to_string(), 25, 6),
            Item::new_damage_item("Longsword".to_string(), 40, 7),
            Item::new_damage_item("Greataxe".to_string(), 74, 8),
        ];
        let armors = vec![
            Item::new_armor_item("None".to_string(), 0, 0),
            Item::new_armor_item("Leather".to_string(), 13, 1),
            Item::new_armor_item("Chainmail".to_string(), 31, 2),
            Item::new_armor_item("Splintmail".to_string(), 53, 3),
            Item::new_armor_item("Bandedmail".to_string(), 75, 4),
            Item::new_armor_item("Platemail".to_string(), 102, 5),
        ];
        let rings = vec![
            Item::new_damage_item("None".to_string(), 0, 0),
            Item::new_damage_item("None".to_string(), 0, 0),
            Item::new_damage_item("Damage 1".to_string(), 25, 1),
            Item::new_damage_item("Damage 2".to_string(), 50, 2),
            Item::new_damage_item("Damage 3".to_string(), 100, 3),
            Item::new_armor_item("Armor 1".to_string(), 20, 1),
            Item::new_armor_item("Armor 2".to_string(), 40, 2),
            Item::new_armor_item("Armor 3".to_string(), 80, 3),
        ];
        let enemy = Boss::parse(enemy)?;

        Ok(Game {
            weapons,
            armors,
            rings,
            enemy,
            player_hit_points: 100,
        })
    }

    fn _find_turns(hit_points: u32, damage: u32, armor: u32) -> u32 {
        hit_points.div_ceil(if armor < damage { damage - armor } else { 1 })
    }

    fn find_outcomes(&self) -> Vec<(bool, u32)> {
        self.weapons
            .iter()
            .cartesian_product(self.armors.iter())
            .cartesian_product(self.rings.iter().combinations(2))
            .map(|((w, a), r)| {
                let armor = r.iter().map(|r| r.armor).sum::<u32>() + a.armor;
                let damage = r.iter().map(|r| r.damage).sum::<u32>() + w.damage;
                let cost = r.iter().map(|r| r.cost).sum::<u32>() + w.cost + a.cost;
                (
                    Self::_find_turns(self.player_hit_points, self.enemy.damage, armor)
                        >= Self::_find_turns(self.enemy.hit_points, damage, self.enemy.armor),
                    cost,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Hit Points: 101
Damage: 7
Armor: 4
";

    #[test]
    fn test_generator() {
        let g = generator(INPUT);
        assert_eq!(g.weapons.len(), 5);
        assert_eq!(g.armors.len(), 6);
        assert_eq!(g.rings.len(), 8);
    }

    #[test]
    fn test_part1() {
        let g = generator(INPUT);
        assert_eq!(part1(&g), 111);
    }

    #[test]
    fn test_part2() {
        let g = generator(INPUT);
        assert_eq!(part2(&g), 232);
    }
}
