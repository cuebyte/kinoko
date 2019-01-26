#![feature(async_await, await_macro, futures_api)]

mod notify;
mod duplicate;
mod paser;

use regex::Regex;


fn main() {
    let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    let before = "2012-03-14, 2013-01-01 and 2014-07-05";
    let cap = re.captures(before).unwrap();
    dbg!(&cap["y"]);
}
