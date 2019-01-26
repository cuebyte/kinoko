use regex::Regex;
use std::io::Result;

///    log String
/// -> regex::Capture
/// -> hashmap
/// -> serde_json.to_string

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_fn() -> Result<()> {
        let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let before = "2012-03-14, 2013-01-01 and 2014-07-05";
        let cap = re.captures(before).unwrap();
        dbg!(&cap);
        println!("{:?}", serde_json::to_string(&cap["y"]));
        assert!(false);
    }
}