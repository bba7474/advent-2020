use std::cmp::Ordering::{Greater, Less};
use std::str::FromStr;

use regex::Regex;

pub fn is_valid_passport_1(passport_str: &str) -> bool {
    let passport: Passport = passport_str.parse().unwrap();
    return passport.is_valid_1()
}

pub fn is_valid_passport_2(passport_str: &str) -> bool {
    let passport: Passport = passport_str.parse().unwrap();
    return passport.is_valid_2()
}

struct Passport {
    passport_fields: Vec<(String, String)>
}

impl Passport {
    pub fn is_valid_1(&self) -> bool {
        // let expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

        let optional_field = "cid";

        if self.passport_fields.len() > 7 {
            return true;
        }
        if self.passport_fields.len() < 7 {
            return false;
        }

        let optional_field_position = self.passport_fields.iter()
            .position(|(field, _value)| field == optional_field);

        if optional_field_position.is_some() {
            return false;
        }

        true
    }

    pub fn is_valid_2(&self) -> bool {
        if !Passport::is_valid_1(self) {
            return false;
        }
        for (field, value) in &self.passport_fields {
            if !is_valid_field(&field, &value) {
                return false;
            }
        }
        true
    }
}

fn is_valid_field(field: &str, value: &str) -> bool {
    if field == "cid" {
        return true;
    }
    if field == "byr" {
        return "1920".cmp(value) != Greater && "2002".cmp(value) != Less
    }
    if field == "iyr" {
        return "2010".cmp(value) != Greater && "2020".cmp(value) != Less
    }
    if field == "eyr" {
        return "2020".cmp(value) != Greater && "2030".cmp(value) != Less
    }
    if field == "hgt" {
        let unit = &value[value.len() - 2..];
        let scalar = &value[0..value.len() - 2];
        if unit == "cm" {
            return "150".cmp(scalar) != Greater && "193".cmp(scalar) != Less
        }
        if unit == "in" {
            return "59".cmp(scalar) != Greater && "76".cmp(scalar) != Less
        }
        return false;
    }
    if field == "hcl" {
        return Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(value);
    }
    if field == "ecl" {
        return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value);
    }
    if field == "pid" {
        return Regex::new(r"^[0-9]{9}$").unwrap().is_match(value);
    }
    println!("Unexpected field");
    return false;
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_fields = s.split_whitespace().collect::<Vec<&str>>();

        let fields_present = split_fields.iter()
            .map(|s| s.split(":").collect::<Vec<&str>>())
            .map(|v| (v[0].to_string(), v[1].to_string()))
            .collect::<Vec<(String, String)>>();

        Ok(Passport{
            passport_fields: fields_present
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passport_1() {
        assert!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".parse::<Passport>().unwrap().is_valid_1());
        assert!(!"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929".parse::<Passport>().unwrap().is_valid_1());
        assert!("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm".parse::<Passport>().unwrap().is_valid_1());
        assert!(!"hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in".parse::<Passport>().unwrap().is_valid_1());
    }

    #[test]
    fn test_field_validation_byr() {
        assert!(is_valid_field("byr", "1991"));
        assert!(!is_valid_field("byr", "2003"));
        assert!(!is_valid_field("byr", "20O1"));
    }

    #[test]
    fn test_field_validation_iyr() {
        assert!(is_valid_field("iyr", "2011"));
        assert!(!is_valid_field("iyr", "2021"));
        assert!(!is_valid_field("iyr", "20O3"));
    }

    #[test]
    fn test_field_validation_eyr() {
        assert!(is_valid_field("eyr", "2023"));
        assert!(!is_valid_field("eyr", "2032"));
        assert!(!is_valid_field("eyr", "2O23"));
    }

    #[test]
    fn test_field_validation_hgt() {
        assert!(is_valid_field("hgt", "175cm"));
        assert!(is_valid_field("hgt", "70in"));
        assert!(!is_valid_field("hgt", "194cm"));
        assert!(!is_valid_field("hgt", "56in"));
    }

    #[test]
    fn test_field_validation_hcl() {
        assert!(is_valid_field("hcl", "#123456"));
        assert!(!is_valid_field("hcl", "123456"));
        assert!(!is_valid_field("hcl", "#12345"));
    }

    #[test]
    fn test_field_validation_ecl() {
        assert!(is_valid_field("ecl", "brn"));
        assert!(!is_valid_field("ecl", "abc"));
        assert!(!is_valid_field("ecl", "123"));
    }

    #[test]
    fn test_field_validation_pid() {
        assert!(is_valid_field("pid", "123456789"));
        assert!(!is_valid_field("pid", "1234567"));
        assert!(!is_valid_field("pid", "ab3456789"));
    }

    #[test]
    fn test_field_validation_cid() {
        assert!(is_valid_field("cid", "anything"));
    }
}