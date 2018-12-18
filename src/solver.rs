use crate::input_record::InputRecord;
use std::collections::{HashMap, HashSet};

pub struct Solver {
    input: Vec<InputRecord>,
}

impl Solver {
    pub fn new(input: Vec<InputRecord>) -> Self {
        Self { input }
    }

    pub fn result(&self) -> String {
        let mut letters = HashSet::new();
        for i in self.input.iter() {
            letters.insert(i.first);
            letters.insert(i.next);
        }

        let mut all = HashMap::new();

        for i in letters.iter() {
            let deps = HashSet::new();
            all.insert(i, deps);
        }

        for i in self.input.iter() {
            all.get_mut(&i.next).unwrap().insert(i.first);
        }

        let mut result: Vec<char> = vec![];

        loop {
            if all.is_empty() {
                break;
            }

            let mut variants: Vec<char> = all
                .iter()
                .filter(|(_, deps)| deps.is_empty())
                .map(|(k, _)| **k)
                .collect();

            if variants.len() == 0 {
                break;
            }

            variants.sort();

            let first = variants[0];

            result.push(first);
            all.remove(&first);

            for set in all.values_mut() {
                set.remove(&first);
            }
        }

        result.iter().collect()
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_basic_solver() {
        let i1 = InputRecord::new('B', 'A');

        let input = vec![i1];
        let solver = Solver::new(input);
        let result = solver.result();

        assert_eq!(result, "BA");
    }
}
