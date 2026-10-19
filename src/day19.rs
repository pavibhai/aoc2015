use itertools::Itertools;
use std::collections::HashMap;
use std::mem::swap;
use std::str::Chars;

pub fn generator(input: &str) -> Chemistry {
    Chemistry::parse(input)
}

pub fn part1(c: &Chemistry) -> u32 {
    c.single_replacements().len() as u32
}

pub fn part2(c: &Chemistry) -> u32 {
    let mut c = c.clone();
    c.minimum_steps()
}

type Molecule = Vec<usize>;

#[derive(Debug, Clone)]
pub struct Chemistry {
    ids: HashMap<String, usize>,
    names: Vec<String>,
    replacements: Vec<Vec<Molecule>>,
    molecule: Molecule,
    matrix: Vec<Vec<Vec<Entry>>>,
}

impl Chemistry {
    fn split_elements(chars: Chars, ids: &mut HashMap<String, usize>) -> Molecule {
        let mut molecule = Molecule::new();
        let last = chars.fold(String::new(), |mut a, c| {
            if c.is_uppercase() && !a.is_empty() {
                let len = ids.len();
                molecule.push(*ids.entry(a.clone()).or_insert(len));
                a.clear();
            }
            a.push(c);
            a
        });
        let len = ids.len();
        molecule.push(*ids.entry(last).or_insert(len));
        molecule
    }

    fn parse(input: &str) -> Self {
        let mut ids: HashMap<String, usize> = HashMap::new();

        let (replacements_str, start) = input.split_once("\n\n").unwrap();
        let mut replacements = Vec::new();

        replacements_str.lines().enumerate().for_each(|(_, line)| {
            let (from, to) = line.split_once(" => ").unwrap();
            let len = ids.len();
            let from_id = *ids.entry(from.to_string()).or_insert(len);
            while replacements.len() < ids.len() {
                replacements.push(Vec::new());
            }
            replacements[from_id].push(Chemistry::split_elements(to.chars(), &mut ids));
        });
        let molecule = Self::split_elements(start.trim().chars(), &mut ids);
        while replacements.len() < ids.len() {
            replacements.push(Vec::new());
        }
        let names = ids
            .iter()
            .sorted_by_key(|(_, i)| *i)
            .map(|(n, _)| n.to_owned())
            .collect();

        let matrix = vec![vec![Vec::new(); molecule.len()]; ids.len()];

        Chemistry {
            replacements,
            ids,
            molecule,
            matrix,
            names,
        }
    }

    fn single_replacements(&self) -> Vec<Molecule> {
        let mut result: Vec<Molecule> = Vec::new();

        for (idx, e) in self.molecule.iter().enumerate() {
            for t in self.replacements[*e].iter() {
                let mut entry = self.molecule[..idx].to_vec();
                t.iter().for_each(|te| entry.push(te.clone()));
                self.molecule[idx + 1..]
                    .iter()
                    .for_each(|e| entry.push(e.clone()));
                result.push(entry);
            }
        }

        result.sort();
        result.dedup();
        result
    }

    fn seed_matrix(&mut self) {
        // Seed the matrix with identity transformations
        for (idx, e) in self.molecule.iter().enumerate() {
            self.matrix[*e][idx].push(Entry {
                steps: 0,
                end_pos: idx + 1,
            });
        }
    }

    fn minimum_steps(&mut self) -> u32 {
        self.seed_matrix();
        let elemental_transitions = self.extract_elemental_transitions();

        for len in 1..=self.molecule.len() {
            self.find(len);
            self.apply_elemental_transitions(&elemental_transitions, len);
        }

        self.matrix[self.ids["e"]][0]
            .iter()
            .find(|e| e.end_pos == self.molecule.len())
            .unwrap()
            .steps
    }

    fn find(&mut self, len: usize) {
        for e_idx in 0..self.names.len() {
            self.find_for_element(e_idx, len);
        }
    }

    fn find_for_element(&mut self, e_idx: usize, len: usize) {
        for r_idx in 0..self.replacements[e_idx].len() {
            for start_pos in 0..=self.molecule.len() - len {
                match self.find_for_element_position(e_idx, start_pos, start_pos + len, r_idx) {
                    Some(steps) => {
                        self.update_matrix(e_idx, start_pos, start_pos + len, steps + 1);
                    }
                    None => (),
                }
            }
        }
    }

    fn find_for_element_position(
        &mut self,
        e_idx: usize,
        start_pos: usize,
        end_pos: usize,
        r_idx: usize,
    ) -> Option<u32> {
        let mut curr = Vec::new();
        let mut next = vec![(start_pos, 0u32)];

        for e in self.replacements[e_idx][r_idx].iter() {
            swap(&mut curr, &mut next);
            next.clear();
            for (start_pos, steps) in curr.iter() {
                if *start_pos < self.molecule.len() {
                    for e in self.matrix[*e][*start_pos]
                        .iter()
                        .filter(|e| e.end_pos <= end_pos)
                    {
                        next.push((e.end_pos, steps + e.steps));
                    }
                }
            }
        }

        next.iter()
            .filter(|(start_pos, _)| *start_pos == end_pos)
            .map(|(_, steps)| steps)
            .min()
            .map(|min| *min)
    }

    fn extract_elemental_transitions(&self) -> Vec<(usize, usize)> {
        // Find all a -> b where b is single element and != a
        let mut result = Vec::new();
        for (idx, transitions) in self.replacements.iter().enumerate() {
            for t in transitions.iter() {
                if t.len() == 1 && t[0] != idx {
                    result.push((idx, t[0]));
                }
            }
        }
        result
    }

    fn apply_elemental_transitions(&mut self, element_transitions: &[(usize, usize)], len: usize) {
        // If we have single element transitions a -> b then apply that costs,
        // do this across many levels that is a -> b -> c should be handled
        let mut repeat = true;
        while repeat {
            repeat = false;
            for start_pos in 0..=self.molecule.len() - len {
                for (lhs, rhs) in element_transitions {
                    let found = self.matrix[*rhs][start_pos]
                        .iter()
                        .find(|e| e.end_pos == start_pos + len)
                        .map(|e| Entry {
                            end_pos: e.end_pos,
                            steps: e.steps + 1,
                        });
                    match found {
                        Some(e) => {
                            if self.update_matrix(*lhs, start_pos, e.end_pos, e.steps) {
                                repeat = true;
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    }

    fn update_matrix(
        &mut self,
        e_idx: usize,
        start_pos: usize,
        end_pos: usize,
        steps: u32,
    ) -> bool {
        match self.matrix[e_idx][start_pos]
            .iter_mut()
            .find(|e| e.end_pos == end_pos)
        {
            Some(e) if e.steps > steps => {
                e.steps = steps;
                true
            }
            None => {
                self.matrix[e_idx][start_pos].push(Entry { end_pos, steps });
                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    steps: u32,
    end_pos: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "H => HO
H => OH
O => HH

HOH";
    const INPUT_2: &str = "H => HO
H => OH
O => HH

HOHOHO";

    #[test]
    fn test_generator() {
        let c = generator(INPUT);
        assert_eq!(c.ids.len(), 2);
        assert_eq!(c.ids["H"], 0);
        assert_eq!(c.ids["O"], 1);
        assert_eq!(c.replacements.len(), 2);
        assert_eq!(
            c.replacements,
            vec![vec![vec![0, 1], vec![1, 0]], vec![vec![0, 0]]]
        );
    }

    #[test]
    fn test_part1() {
        let c = generator(INPUT);
        assert_eq!(part1(&c), 4);

        let c = generator(INPUT_2);
        assert_eq!(part1(&c), 7);
    }

    #[test]
    fn test_part2() {
        let c = generator(
            "e => H
e => O
H => HO
H => OH
O => HH

HOH",
        );
        assert_eq!(part2(&c), 3);

        let c = generator(
            "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO",
        );
        assert_eq!(part2(&c), 6);
    }

    #[test]
    fn test_grammar() {
        let mut c = generator(
            "e => H
e => O
H => HO
H => OH
O => HH

HOH",
        );

        c.seed_matrix();
        assert_eq!(
            c.matrix[c.ids["e"]].iter().filter(|v| v.is_empty()).count(),
            3
        );

        assert_eq!(
            c.matrix[c.ids["H"]].iter().filter(|v| v.is_empty()).count(),
            1
        );
        assert_eq!(
            c.matrix[c.ids["H"]][0],
            vec![Entry {
                steps: 0,
                end_pos: 1
            }]
        );
        assert_eq!(
            c.matrix[c.ids["H"]][2],
            vec![Entry {
                steps: 0,
                end_pos: 3
            }]
        );

        assert_eq!(
            c.matrix[c.ids["O"]].iter().filter(|v| v.is_empty()).count(),
            2
        );
        assert_eq!(
            c.matrix[c.ids["O"]][1],
            vec![Entry {
                steps: 0,
                end_pos: 2
            }]
        );

        c.find(1);
        assert_eq!(
            c.matrix[c.ids["e"]].iter().filter(|v| v.is_empty()).count(),
            0
        );
        assert_eq!(
            c.matrix[c.ids["e"]][0],
            vec![Entry {
                steps: 1,
                end_pos: 1
            }]
        );
        assert_eq!(
            c.matrix[c.ids["e"]][1],
            vec![Entry {
                steps: 1,
                end_pos: 2
            }]
        );
        assert_eq!(
            c.matrix[c.ids["e"]][2],
            vec![Entry {
                steps: 1,
                end_pos: 3
            }]
        );

        assert_eq!(
            c.matrix[c.ids["H"]].iter().filter(|v| v.is_empty()).count(),
            1
        );
        assert_eq!(
            c.matrix[c.ids["H"]][0],
            vec![Entry {
                steps: 0,
                end_pos: 1
            }]
        );
        assert_eq!(
            c.matrix[c.ids["H"]][2],
            vec![Entry {
                steps: 0,
                end_pos: 3
            }]
        );

        assert_eq!(
            c.matrix[c.ids["O"]].iter().filter(|v| v.is_empty()).count(),
            2
        );
        assert_eq!(
            c.matrix[c.ids["O"]][1],
            vec![Entry {
                steps: 0,
                end_pos: 2
            }]
        );

        c.find(2);
        c.find(3);
        println!("{:?}", c);
    }
}
