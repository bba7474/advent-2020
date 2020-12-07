use std::str::FromStr;

pub struct Luggage {
    color: String,
    contains: Vec<String>
}

impl Luggage {
    pub fn get_entry(&self) -> (String, Vec<String>) {
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
                contains.push(&s[2..end_of_color]);
            }
        }

        Ok(Luggage{
            color: color.to_string(),
            contains: contains.iter().map(|s| s.to_string()).collect()
        })
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
        assert!(luggage1.contains.contains(&"pale blue".to_string()));
        assert!(luggage1.contains.contains(&"dark violet".to_string()));
    }

    #[test]
    fn test_parse_string_one_contains() {
        let luggage1 = "mirrored gold bags contain 2 pale blue bags.".parse::<Luggage>().unwrap();
        assert_eq!(luggage1.color, "mirrored gold");
        assert_eq!(luggage1.contains.len(), 1);
        assert!(luggage1.contains.contains(&"pale blue".to_string()));
    }

    #[test]
    fn test_parse_string_empty_contains() {
        let luggage1 = "mirrored gold bags contain no other bags.".parse::<Luggage>().unwrap();
        assert_eq!(luggage1.color, "mirrored gold");
        assert_eq!(luggage1.contains.len(), 0);
    }
}

