use crate::input_record::InputRecord;
use std::collections::{HashMap, HashSet};

fn char_index(input: char) -> i32 {
    assert!(input.len_utf8() == 1);

    let value = input.to_digit(36).expect("Invalid char") - 9;
    value as i32
}

#[derive(Debug)]
struct Part {
    letter: char,
    tick: i32,
    active: bool,
    deps: HashSet<char>,
}

impl Part {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn tick(&mut self) {
        self.tick = self.tick - 1;
    }

    pub fn start(&mut self) {
        self.active = true;
    }

    pub fn ready_start(&self) -> bool {
        self.deps.len() == 0
    }

    pub fn ready_stop(&self) -> bool {
        self.tick <= 0
    }
}

pub struct TickSolver {
    parts: HashMap<char, Part>,
}

impl TickSolver {
    pub fn new(input: Vec<InputRecord>, workers: i32, add: i32) -> Self {
        let mut letters = HashSet::new();

        for i in input.iter() {
            letters.insert(i.first);
            letters.insert(i.next);
        }

        let mut parts = HashMap::new();

        for l in letters {
            let part = Part {
                letter: l,
                tick: char_index(l) + add,
                active: false,
                deps: HashSet::new(),
            };

            parts.insert(l, part);
        }

        for i in input {
            let part = parts.get_mut(&i.next).unwrap();
            part.deps.insert(i.first);
        }

        Self { parts }
    }

    fn remove(&mut self, letter: char) {
        self.parts.remove(&letter);

        for part in self.parts.values_mut() {
            part.deps.remove(&letter);
        }
    }

    pub fn result(&mut self) -> i32 {
        let mut current = 0;
        loop {
            println!("Tick {}: {:?}", current, self.parts);

            let to_remove: Vec<char> = self
                .parts
                .values()
                .filter(|part| part.ready_stop())
                .map(|part| part.letter)
                .collect();

            for letter in to_remove {
                self.remove(letter);
            }

            if self.parts.len() == 0 {
                break;
            }

            for part in self.parts.values_mut().filter(|part| part.active()) {
                part.tick();
            }

            for part in self.parts.values_mut().filter(|part| part.ready_start()) {
                part.start();
            }

            current += 1;
        }
        current
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn simple() {
        let input = vec![InputRecord::parse("B->A"), InputRecord::parse("C->A")];
        let mut solver = TickSolver::new(input, 10, 0);
        let result = solver.result();

        assert_eq!(result, 6);
    }
    #[test]
    fn simple_with_add() {
        let input = vec![InputRecord::parse("B->A"), InputRecord::parse("C->A")];
        let mut solver = TickSolver::new(input, 10, 100);
        let result = solver.result();

        assert_eq!(result, 204);
    }

    #[test]
    fn char_index_test() {
        assert_eq!(char_index('A'), 1);
        assert_eq!(char_index('B'), 2);
        assert_eq!(char_index('z'), 26);
    }
}
