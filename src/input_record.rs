use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct InputRecord {
    first: char,
    next: char,
}

impl InputRecord {
    pub fn parse(src: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^Step (\w) must be finished before step (\w) can begin.$").unwrap();
        }

        let cap = RE.captures(src).unwrap();
        let first = cap[1].parse::<char>().unwrap();
        let next = cap[2].parse::<char>().unwrap();

        Self { first, next }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_condition() {
        let record = InputRecord::parse("Step C must be finished before step A can begin.");
        assert_eq!(
            record,
            InputRecord {
                first: 'C',
                next: 'A'
            }
        );
    }
}
