use std::str::FromStr;

pub fn is_valid_passport_1(passport_str: &str) -> bool {
    let passport: Passport = passport_str.parse().unwrap();
    return passport.is_valid_1()
}

struct Passport {
    present_fields: Vec<String>
}

impl Passport {
    pub fn is_valid_1(&self) -> bool {
        // let expected_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

        let optional_field = "cid";


        if self.present_fields.len() > 7 {
            return true;
        }
        if self.present_fields.len() < 7 {
            return false;
        }

        let optional_field_position = self.present_fields.iter()
            .position(|s| s == optional_field);

        if optional_field_position.is_some() {
            return false;
        }

        true
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_fields = s.split_whitespace().collect::<Vec<&str>>();

        let fields_present = split_fields.iter()
            .map(|s| s.split(":").collect::<Vec<&str>>()[0].to_string())
            .collect::<Vec<String>>();

        Ok(Passport{
            present_fields: fields_present
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passport() {
        assert!("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm".parse::<Passport>().unwrap().is_valid_1());
        assert!(!"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929".parse::<Passport>().unwrap().is_valid_1());
        assert!("hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm".parse::<Passport>().unwrap().is_valid_1());
        assert!(!"hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in".parse::<Passport>().unwrap().is_valid_1());
    }
}