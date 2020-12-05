use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_KEYS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

struct ID(HashMap<String, String>);
impl From<String> for ID {
    fn from(id_str: String) -> Self {
        let tokens: Vec<&str> = id_str
            .split(' ')
            .filter(|tok| !tok.is_empty())
            .collect::<Vec<&str>>();

        ID(tokens
            .iter()
            .map(|val| {
                let key_val = val.clone().split(':').collect::<Vec<&str>>();
                (key_val[0].to_string(), key_val[1].to_string())
            })
            .collect::<HashMap<String, String>>())
    }
}

impl ID {
    fn valid_keys(&self) -> bool {
        let keys = self.0.keys().cloned().collect::<Vec<String>>();

        let additional_keys: Vec<String> = keys
            .clone()
            .into_iter()
            .filter(|val| !VALID_KEYS.contains(&val.as_str()))
            .collect::<Vec<String>>();
        if additional_keys.len() > 0 {
            return false;
        }

        let missing_required_keys: Vec<&str> = REQUIRED_KEYS
            .to_vec()
            .into_iter()
            .filter(|val| !keys.contains(&val.to_string()))
            .collect();
        if missing_required_keys.len() > 0 {
            return false;
        }

        return true;
    }

    fn val_between(&self, key: &str, min: usize, max: usize) -> bool {
        let mut val_ok = false;
        if let Ok(year) = self.0[key].parse::<usize>() {
            if year <= max && year >= min {
                val_ok = true;
            }
        }

        val_ok
    }

    fn valid_height(&self) -> bool {
        let chars: Vec<char> = self.0["hgt"].chars().collect();
        let unit: String = chars
            .clone()
            .into_iter()
            .filter(|val| val.is_alphabetic())
            .collect();
        if unit != "cm" && unit != "in" {
            return false;
        }

        let value: String = chars.into_iter().filter(|val| val.is_numeric()).collect();
        if let Ok(value_usize) = value.parse::<usize>() {
            if unit == "cm" && value_usize <= 193 && value_usize >= 150 {
                return true;
            }

            if unit == "in" && value_usize <= 76 && value_usize >= 59 {
                return true;
            }
        }

        return false;
    }

    fn valid_hcl(&self) -> bool {
        if self.0["hcl"].len() != 7 {
            return false;
        }

        let mut chars = self.0["hcl"].chars();
        if chars.nth(0) != Some('#') {
            return false;
        }

        chars
            .nth(1)
            .into_iter()
            .filter(|val| !val.is_alphanumeric())
            .collect::<Vec<char>>()
            .len()
            == 0
    }

    fn valid_ecl(&self) -> bool {
        const VALID_EYE_COLOR: [&'static str; 7] =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        VALID_EYE_COLOR.contains(&self.0["ecl"].as_str())
    }

    fn valid_pid(&self) -> bool {
        let pid = &self.0["pid"];
        pid.len() == 9
            && pid
                .chars()
                .into_iter()
                .filter(|val| !val.is_numeric())
                .collect::<Vec<char>>()
                .len()
                == 0
    }

    fn valid_values(&self) -> bool {
        let byr_ok = self.val_between("byr", 1920, 2002);
        let iyr_ok = self.val_between("iyr", 2010, 2020);
        let eyr_ok = self.val_between("eyr", 2020, 2030);
        let hgt_ok = self.valid_height();
        let hcl_ok = self.valid_hcl();
        let ecl_ok = self.valid_ecl();
        let pid_ok = self.valid_pid();

        return byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok;
    }

    fn is_valid(&self) -> bool {
        self.valid_keys() && self.valid_values()
    }
}

fn read_all_ids<R: Read>(reader: R) -> Vec<String> {
    let buf_reader = BufReader::new(reader);
    let mut result = Vec::new();
    let mut passport_str = String::new();

    for line_res in buf_reader.lines() {
        let line = line_res.unwrap();
        if line == String::new() {
            result.push(passport_str);
            passport_str = String::new();
        } else {
            passport_str.push_str(line.as_str());
            passport_str.push_str(" ");
        }
    }
    result
}

pub fn solve_puzzle<R: Read>(reader: R) {
    let passports = read_all_ids(reader);
    let mut count = 0;
    for passport_str in passports {
        let id = ID::from(passport_str);
        count += id.is_valid() as usize;
    }

    println!("Nr of valid IDs = {}", count);
}

#[cfg(test)]
mod tests {
    use crate::dec4::solve_puzzle;
    use std::fs::File;

    #[test]
    fn test() {
        solve_puzzle(File::open("src/dec4/input.txt").unwrap());
    }
}
