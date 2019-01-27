use regex::Regex;
use std::collections::HashMap;

pub struct Parser {
    re: Regex,
}

impl Parser {
    pub fn new(re: &str) -> Self {
        Self {
            re: Regex::new(re).unwrap()
        }
    }

    pub fn parse(&self, input: &str) -> Vec<HashMap<String, String>> {
        let mut result: Vec<HashMap<String, String>> = Vec::new();
        let names = self.re.capture_names()
            .filter_map(|x| x)
            .collect::<Vec<_>>();

        for caps in self.re.captures_iter(input) {
            let mut map: HashMap<String, String> = HashMap::new();
            for name in names.clone() {
                if let Some(m) = caps.name(name) {
                    map.insert(name.to_owned(), m.as_str().to_owned());
                }
            }
            if map.len() > 0 {
                result.push(map);
            }
        }
        result
    }
}

mod tests {
    use super::*;

    #[test]
    fn regex_fn() {
        let parser = Parser::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})");
        let data = parser.parse("2012-03-14, 2013-01-01 and 2014-07-05");
        assert_eq!(3, data.len());
        for map in data {
            assert_eq!(3, map.len());
        }
    }
}