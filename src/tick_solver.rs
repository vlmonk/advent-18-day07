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
        self.deps.len() == 0 && !self.active
    }

    pub fn ready_stop(&self) -> bool {
        self.tick <= 0
    }
}

pub struct TickSolver {
    parts: HashMap<char, Part>,
    max_workers: i32,
    current_workers: i32,
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

        Self {
            parts,
            max_workers: workers,
            current_workers: 0,
        }
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
            let to_remove: Vec<char> = self
                .parts
                .values()
                .filter(|part| part.ready_stop())
                .map(|part| part.letter)
                .collect();

            for letter in to_remove {
                self.remove(letter);
                self.current_workers -= 1;
            }

            if self.parts.len() == 0 {
                break;
            }

            let max_jobs_add = (self.max_workers - self.current_workers) as usize;

            let ready_to_start: Vec<char> = self
                .parts
                .values()
                .filter(|part| part.ready_start())
                .map(|part| part.letter)
                .collect();

            for part in self
                .parts
                .values_mut()
                .filter(|part| part.ready_start())
                .take(max_jobs_add)
            {
                part.start();
                self.current_workers += 1;
            }

            for part in self.parts.values_mut().filter(|part| part.active()) {
                part.tick();
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
    fn very_simple() {
        let input = vec![InputRecord::parse("A->B")];
        let mut solver = TickSolver::new(input, 11, 0);
        let result = solver.result();

        assert_eq!(result, 3);
    }
    #[test]
    fn simple() {
        let input = vec![InputRecord::parse("B->A"), InputRecord::parse("C->A")];
        let mut solver = TickSolver::new(input, 10, 0);
        let result = solver.result();

        assert_eq!(result, 4);
    }
    #[test]
    fn simple_with_add() {
        let input = vec![InputRecord::parse("B->A"), InputRecord::parse("C->A")];
        let mut solver = TickSolver::new(input, 10, 10);
        let result = solver.result();

        assert_eq!(result, 24);
    }
    #[test]
    fn simple_with_worker_limit() {
        let input = vec![InputRecord::parse("B->A"), InputRecord::parse("C->A")];
        let mut solver = TickSolver::new(input, 1, 0);
        let result = solver.result();

        assert_eq!(result, 6);
    }

    #[test]
    fn char_index_test() {
        assert_eq!(char_index('A'), 1);
        assert_eq!(char_index('B'), 2);
        assert_eq!(char_index('z'), 26);
    }
}
