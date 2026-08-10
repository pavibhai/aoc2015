use crate::day7::Signal::{And, LShift, Literal, Not, Or, RShift, Wire};
use std::collections::{HashMap, VecDeque};

pub fn part1(bt: &BobbyTables) -> u16 {
    let signal_values = bt.run();
    signal_values[*bt._signal_id("a").unwrap()].unwrap()
}

pub fn part2(bt: &BobbyTables) -> u16 {
    let a_signal = part1(bt);
    let b_idx = *bt._signal_id("b").unwrap();
    let mut bt = bt.clone();
    bt.signals[b_idx] = Literal(a_signal);
    part1(&bt)
}

pub fn generator(input: &str) -> BobbyTables {
    BobbyTables::from_str(input).expect("Input is invalid")
}

#[derive(Debug, Clone)]
pub struct BobbyTables {
    signals: Vec<Signal>,
    signal_ids: HashMap<String, usize>,
    depends_on: Vec<Vec<usize>>,
}

impl BobbyTables {
    fn from_str(input: &str) -> Result<Self, String> {
        let mut signal_ids = HashMap::new();
        let signals: Vec<&str> = input
            .lines()
            .enumerate()
            .map(|(idx, l)| {
                l.split_once(" -> ")
                    .map(|(s, t)| {
                        signal_ids.entry(t.to_owned()).or_insert(idx);
                        s
                    })
                    .ok_or(format!("{} is invalid", l))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let signals = signals
            .into_iter()
            .map(|s| Signal::from_str(s, &signal_ids))
            .collect::<Result<Vec<_>, _>>()?;

        let mut depends_on = vec![vec![]; signals.len()];
        signals.iter().enumerate().for_each(|(idx, s)| {
            s.depends_on()
                .iter()
                .for_each(|d_idx| depends_on[*d_idx].push(idx));
        });

        Ok(BobbyTables {
            signals,
            signal_ids,
            depends_on,
        })
    }

    fn _signal_id(&self, id: &str) -> Result<&usize, ()> {
        self.signal_ids.get(id).ok_or(())
    }

    fn run(&self) -> Vec<Option<u16>> {
        let mut signal_values = vec![None; self.signals.len()];
        let mut queue = VecDeque::new();
        self.signals
            .iter()
            .enumerate()
            .filter(|(_, s)| s.depends_on().is_empty())
            .for_each(|(idx, _)| queue.push_back(idx));

        while !queue.is_empty() {
            let q_idx = queue.pop_front().unwrap();
            if signal_values[q_idx].is_none() {
                signal_values[q_idx] = self.signals[q_idx].compute(&signal_values);
                if signal_values[q_idx].is_some() {
                    self.depends_on[q_idx]
                        .iter()
                        .for_each(|d_idx| queue.push_back(*d_idx));
                }
            }
        }

        signal_values
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Signal {
    Wire(usize),
    Literal(u16),
    And(Box<Signal>, Box<Signal>),
    Or(Box<Signal>, Box<Signal>),
    LShift(Box<Signal>, Box<Signal>),
    RShift(Box<Signal>, Box<Signal>),
    Not(Box<Signal>),
}

impl Signal {
    fn from_str(input: &str, signal_ids: &HashMap<String, usize>) -> Result<Signal, String> {
        if input.contains(' ') {
            let mut splits = input.split_whitespace();
            let s1 = splits.next().ok_or("Input is invalid")?;
            let s2 = splits.next().ok_or("Input is invalid")?;
            let s3 = splits.next();
            if s3.is_some() {
                let s1 = Box::from(Self::from_str(s1, signal_ids)?);
                let s3 = Box::from(Self::from_str(s3.unwrap(), signal_ids)?);
                match s2 {
                    "AND" => Ok(And(s1, s3)),
                    "OR" => Ok(Or(s1, s3)),
                    "LSHIFT" => Ok(LShift(s1, s3)),
                    "RSHIFT" => Ok(RShift(s1, s3)),
                    _ => Err(format!("Invalid input: {}", input)),
                }
            } else {
                Ok(Not(Box::new(Signal::from_str(s2, signal_ids)?)))
            }
        } else {
            match input.parse::<u16>() {
                Ok(n) => Ok(Literal(n)),
                _ => Ok(Wire(
                    signal_ids
                        .get(input)
                        .ok_or(format!("Cannot find code for {input}"))?
                        .to_owned(),
                )),
            }
        }
    }

    fn depends_on(&self) -> Vec<usize> {
        let mut depends_on = Vec::new();
        self._depends_on(&mut depends_on);
        depends_on
    }

    fn _depends_on(&self, depends_on: &mut Vec<usize>) {
        match self {
            Wire(s) => depends_on.push(s.to_owned()),
            Literal(_) => {}
            Not(s) => s._depends_on(depends_on),
            And(l, r) | Or(l, r) | LShift(l, r) | RShift(l, r) => {
                l._depends_on(depends_on);
                r._depends_on(depends_on);
            }
        }
    }

    fn compute(&self, signal_values: &[Option<u16>]) -> Option<u16> {
        match self {
            Literal(v) => Some(*v),
            Wire(idx) => signal_values[*idx],
            Not(s) => s.compute(signal_values).map(|v| !v),
            And(l, r) => l
                .compute(signal_values)
                .zip(r.compute(signal_values))
                .map(|(l, r)| l & r),
            Or(l, r) => l
                .compute(signal_values)
                .zip(r.compute(signal_values))
                .map(|(l, r)| l | r),
            LShift(l, r) => l
                .compute(signal_values)
                .zip(r.compute(signal_values))
                .map(|(l, r)| l << r),
            RShift(l, r) => l
                .compute(signal_values)
                .zip(r.compute(signal_values))
                .map(|(l, r)| l >> r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{generator, Signal};
    use crate::day7::Signal::{Literal, Not, Wire};

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_generator() {
        let bt = generator(INPUT);
        assert_eq!(bt.signals.len(), 8);
        assert_eq!(bt.signals[1], Literal(456));
        assert_eq!(bt.signal_ids.get("y").unwrap(), &1);
        assert_eq!(
            bt.signals[5],
            Signal::RShift(
                Box::from(Wire(*bt._signal_id("y").unwrap())),
                Box::from(Literal(2))
            )
        );
        assert_eq!(bt.signal_ids.get("g").unwrap(), &5);
        assert_eq!(
            bt.signals[6],
            Not(Box::from(Wire(*bt._signal_id("x").unwrap())))
        );

        // Depends On
        assert_eq!(bt.depends_on[bt.signal_ids["x"]], vec![]);
        assert_eq!(
            bt.depends_on[bt.signal_ids["e"]],
            vec![bt.signal_ids["x"], bt.signal_ids["y"]]
        );
        assert_eq!(bt.depends_on[bt.signal_ids["g"]], vec![bt.signal_ids["y"]]);
        assert_eq!(bt.depends_on[bt.signal_ids["h"]], vec![bt.signal_ids["x"]]);
    }

    #[test]
    fn test_part_1() {
        let bt = generator(INPUT);
        let result = bt.run();
        assert_eq!(
            result,
            vec![
                Some(123),
                Some(456),
                Some(72),
                Some(507),
                Some(492),
                Some(114),
                Some(65412),
                Some(65079)
            ]
        );
    }
}
