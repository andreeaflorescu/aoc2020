use std::io::{self, BufRead, Read};

#[derive(PartialEq, Debug, Copy, Clone)]
struct Rule {
    letter: char,
    first: usize,
    second: usize,
}

fn parse_line(line: String) -> (Rule, String) {
    let tokens: Vec<String> = line.split("-").map(|val| val.to_string()).collect();
    let min = tokens[0].parse::<usize>().unwrap();
    let rest: Vec<String> = tokens[1].split(" ").map(|val| val.to_string()).collect();
    let max = rest[0].parse::<usize>().unwrap();
    let letter: Vec<&str> = rest[1].split(":").collect();
    let letter = letter[0].chars().nth(0).unwrap();
    let password = rest[2].to_string();

    (
        Rule {
            letter,
            first: min,
            second: max,
        },
        password,
    )
}

fn check_simple_rule(rule: Rule, passwd: String) -> bool {
    let count = passwd.matches(&rule.letter.to_string()).count();
    count >= rule.first && count <= rule.second
}

fn check_complicated_rule(rule: Rule, passwd: String) -> bool {
    let chars = passwd.as_bytes();

    (chars[rule.first - 1] as char == rule.letter) ^ (chars[rule.second - 1] as char == rule.letter)
}

pub fn solve_puzzle<T: Read>(reader: T) {
    let reader = io::BufReader::new(reader);
    let mut complicated_count = 0;
    let mut simple_count = 0;
    for line in reader.lines() {
        let (rule, passwd) = parse_line(line.unwrap());
        if check_complicated_rule(rule, passwd.clone()) {
            complicated_count += 1;
        }
        if check_simple_rule(rule, passwd) {
            simple_count += 1;
        }
    }
    println!("number of correct passwd simple={}", simple_count);
    println!("number of correct passwd complicated={}", complicated_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "4-8 t: pctpfqtrtttmvptvfmws".to_string();
        let expected_passwd = "pctpfqtrtttmvptvfmws".to_string();
        let expected_rule = Rule {
            letter: 't',
            first: 4,
            second: 8,
        };

        let (actual_rule, actual_passwd) = parse_line(line);
        assert_eq!(expected_rule, actual_rule);
        assert_eq!(expected_passwd, actual_passwd);
    }
}
