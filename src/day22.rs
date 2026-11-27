pub fn generator(input: &str) -> Game {
    Game::new(input, 50, 500, false).expect("invalid input")
}

pub fn part1(game: &Game) -> u32 {
    game.find_min_mana_wins()
}

pub fn part2(game: &Game) -> u32 {
    let mut game = game.clone();
    game.hard_mode = true;
    game.find_min_mana_wins()
}

#[derive(Debug, Clone)]
pub struct Game {
    boss: Boss,
    wizard: Wizard,
    spells: Vec<Spell>,
    hard_mode: bool,
}

impl Game {
    fn new(boss_stats: &str, wizard_hit_points: u32, wizard_mana: u32, hard_mode: bool) -> Result<Self, String> {
        Ok(Game {
            boss: Boss::parse(boss_stats)?,
            wizard: Wizard::new(wizard_hit_points, wizard_mana),
            spells: Spell::all_spells(),
            hard_mode,
        })
    }

    fn find_min_mana_wins(&self) -> u32 {
        let mut min_mana = u32::MAX;
        let wizard = self.wizard.clone();
        let boss = self.boss.clone();
        self.wizard_turn(wizard, boss, &mut min_mana);
        min_mana
    }

    fn wizard_turn(&self, mut wizard: Wizard, mut boss: Boss, min_mana: &mut u32) {
        if self.hard_mode {
            if wizard.hit_points <= 1 {
                return;
            } else {
                wizard.hit_points -= 1;
            }
        }
        self.before_turn(&mut wizard, &mut boss);
        if boss.hit_points == 0 {
            *min_mana = wizard.spent_mana.min(*min_mana);
            return;
        }

        for spell in self.spells.iter().filter(|s| {
            s.cost <= wizard.mana
                && wizard.active_spells.iter().all(|a| s.name != a.name)
        }) {
            if wizard.spent_mana + spell.cost >= *min_mana {
                continue;
            }
            let mut wizard = wizard.clone();
            let mut boss = boss.clone();
            self.cast(&mut wizard, spell, &mut boss);
            self.boss_turn(wizard, boss, min_mana);
        }
    }

    fn boss_turn(&self, mut wizard: Wizard, mut boss: Boss, min_mana: &mut u32) {
        self.before_turn(&mut wizard, &mut boss);
        if boss.hit_points == 0 {
            *min_mana = wizard.spent_mana.min(*min_mana);
            return;
        }

        wizard.defend(&boss);
        if wizard.hit_points > 0 {
            self.wizard_turn(wizard, boss, min_mana);
        }
    }

    fn cast(&self, wizard: &mut Wizard, spell: &Spell, boss: &mut Boss) {
        if spell.turns > 0 {
            wizard.active_spells.push(spell.clone());
            boss.active_spells.push(spell.clone());
        } else {
            boss.defend(&spell);
            wizard.cast(&spell);
        }
        wizard.spent_mana += spell.cost;
        wizard.mana -= spell.cost;
    }

    fn before_turn(&self, wizard: &mut Wizard, boss: &mut Boss) {
        wizard.before_turn();
        boss.before_turn();
    }
}

#[derive(Clone, Debug)]
struct Boss {
    hit_points: u32,
    damage: u32,
    active_spells: Vec<Spell>,
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

        Ok(Boss {
            hit_points: hit_points.parse::<u32>().map_err(|e| e.to_string())?,
            damage: damage.parse().map_err(|_| "invalid damage")?,
            active_spells: Vec::new(),
        })
    }

    fn defend(&mut self, spell: &Spell) -> bool {
        if self.hit_points > spell.damage {
            self.hit_points -= spell.damage;
            true
        } else {
            self.hit_points = 0;
            false
        }
    }

    fn before_turn(&mut self) {
        for spell in self.active_spells.iter_mut() {
            if self.hit_points > spell.damage {
                self.hit_points -= spell.damage;
            } else {
                self.hit_points = 0;
            }
            spell.turns -= 1;
        }
        self.active_spells.retain(|s| s.turns > 0);
    }
}

#[derive(Clone, Debug)]
struct Wizard {
    mana: u32,
    hit_points: u32,
    armor: u32,
    active_spells: Vec<Spell>,
    spent_mana: u32,
}

impl Wizard {
    fn new(hit_points: u32, mana: u32) -> Self {
        Wizard {
            mana,
            hit_points,
            armor: 0,
            active_spells: vec![],
            spent_mana: 0,
        }
    }

    fn defend(&mut self, boss: &Boss) {
        let d = if boss.damage > self.armor {
            boss.damage - self.armor
        } else {
            1
        };
        if d < self.hit_points {
            self.hit_points -= d;
        } else {
            self.hit_points = 0;
        }
    }

    fn before_turn(&mut self) {
        self.armor = 0;
        for spell in self.active_spells.iter_mut() {
            self.armor += spell.armor;
            self.mana += spell.mana;
            self.hit_points += spell.heals;
            spell.turns -= 1;
        }
        self.active_spells.retain(|s| s.turns > 0);
    }

    fn cast(&mut self, spell: &Spell) {
        self.hit_points += spell.heals;
    }
}

#[derive(Clone, Debug)]
struct Spell {
    name: String,
    cost: u32,
    turns: u32,
    damage: u32,
    heals: u32,
    armor: u32,
    mana: u32,
}

impl Spell {
    fn all_spells() -> Vec<Spell> {
        vec![
            Spell {
                name: "Magic Missile".to_string(),
                cost: 53,
                turns: 0,
                damage: 4,
                heals: 0,
                armor: 0,
                mana: 0,
            },
            Spell {
                name: "Drain".to_string(),
                cost: 73,
                turns: 0,
                damage: 2,
                heals: 2,
                armor: 0,
                mana: 0,
            },
            Spell {
                name: "Shield".to_string(),
                cost: 113,
                turns: 6,
                damage: 0,
                heals: 0,
                armor: 7,
                mana: 0,
            },
            Spell {
                name: "Poison".to_string(),
                cost: 173,
                turns: 6,
                damage: 3,
                heals: 0,
                armor: 0,
                mana: 0,
            },
            Spell {
                name: "Recharge".to_string(),
                cost: 229,
                turns: 5,
                damage: 0,
                heals: 0,
                armor: 0,
                mana: 101,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Hit Points: 13
Damage: 8
";
    const INPUT2: &str = "Hit Points: 14
Damage: 8
";

    #[test]
    fn test_generator() {
        let g = generator(INPUT);
        assert_eq!(g.boss.hit_points, 13);
        assert_eq!(g.boss.damage, 8);
        assert_eq!(g.wizard.mana, 500);
        assert_eq!(g.wizard.hit_points, 50);

        let g = Game::new(INPUT, 10, 250, false).unwrap();

        assert_eq!(g.boss.hit_points, 13);
        assert_eq!(g.boss.damage, 8);
        assert_eq!(g.wizard.mana, 250);
        assert_eq!(g.wizard.hit_points, 10);
    }

    #[test]
    fn test_part1() {
        let g = Game::new(INPUT, 10, 250, false).unwrap();
        assert_eq!(part1(&g), 226);

        let g = Game::new(INPUT2, 10, 250, false).unwrap();
        assert_eq!(part1(&g), 641);
    }

    #[test]
    fn test_spells() {
        let g = Game::new(INPUT, 10, 250, false).unwrap();
        let mut w = g.wizard.clone();
        let mut b = g.boss.clone();
        g.cast(&mut w, &g.spells[3], &mut b);
        assert_eq!(w.mana, 77);
        assert_eq!(b.hit_points, 13);

        g.before_turn(&mut w, &mut b);
        assert_eq!(w.mana, 77);
        assert_eq!(b.hit_points, 10);

        w.defend(&b);
        assert_eq!(w.hit_points, 2);
        assert_eq!(w.mana, 77);

        g.before_turn(&mut w, &mut b);
        assert_eq!(w.mana, 77);
        assert_eq!(b.hit_points, 7);

        g.cast(&mut w, &g.spells[0], &mut b);
        assert_eq!(b.hit_points, 3);
    }
}
