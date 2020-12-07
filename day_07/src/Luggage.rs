use std::str::FromStr;

pub struct Luggage {
    color: String,
    contains: Vec<LuggageContainsRules>
}

impl Luggage {
    pub fn get_entry(&self) -> (String, Vec<LuggageContainsRules>) {
        (self.color.clone(), self.contains.clone())
    }
}

impl FromStr for Luggage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" contain ").collect::<Vec<&str>>();
        let color = &parts[0][0..parts[0].len() - 5];

        let mut contains = Vec::new();
        if parts[1] != "no other bags." {
            let contains_strings = parts[1].split(", ").collect::<Vec<&str>>();

            for s in contains_strings {
                let end_of_color = s.rfind(" ").unwrap();
                let color = &s[2..end_of_color].to_string();
                let count = &s[0..1].parse::<i32>().expect("not an integer");
                contains.push(LuggageContainsRules{
                    color: color.to_string(),
                    count: count.clone()
                });
            }
        }

        Ok(Luggage{
            color: color.to_string(),
            contains
        })
    }
}

pub struct LuggageContainsRules {
    color: String,
    count: i32
}

impl LuggageContainsRules {
    pub fn get_color(&self) -> String {
        self.color.clone()
    }
    pub fn get_count(&self) -> i32 {
        self.count.clone()
    }
}

impl Clone for LuggageContainsRules {
    fn clone(&self) -> Self {
        return LuggageContainsRules {
            color: self.color.clone(),
            count: self.count.clone()
        };
    }
}

impl PartialEq for LuggageContainsRules {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.count == other.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_multi_contains() {
        let luggage1 = "mirrored gold bags contain 2 pale blue bags, 1 dark violet bag.".parse::<Luggage>().unwrap();
        assert_eq!(luggage1.color, "mirrored gold");
        assert_eq!(luggage1.contains.len(), 2);

        assert!(luggage1.contains.contains(&LuggageContainsRules { color: "pale blue".to_string(), count: 2 }));
        assert!(luggage1.contains.contains(&LuggageContainsRules { color: "dark violet".to_string(), count: 1 }));
    }

    #[test]
    fn test_parse_string_one_contains() {
        let luggage1 = "mirrored gold bags contain 2 pale blue bags.".parse::<Luggage>().unwrap();
        assert_eq!(luggage1.color, "mirrored gold");
        assert_eq!(luggage1.contains.len(), 1);
        assert!(luggage1.contains.contains(&LuggageContainsRules { color: "pale blue".to_string(), count: 2 }));
    }

    #[test]
    fn test_parse_string_empty_contains() {
        let luggage1 = "mirrored gold bags contain no other bags.".parse::<Luggage>().unwrap();
        assert_eq!(luggage1.color, "mirrored gold");
        assert_eq!(luggage1.contains.len(), 0);
    }
}

