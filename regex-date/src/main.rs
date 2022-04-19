use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d\d\d\d").unwrap();
    assert!(re.is_match("2014-01-01"));
    println!("Hello, world!");
}
