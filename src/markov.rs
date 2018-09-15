use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::fs::File;

pub struct MarkovChain {
    values : HashMap<String, (String, u32)>,
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            values: HashMap::new(),
        }
    }
    
    pub fn generate(&mut self, filepath : String) -> Result<bool, io::Error> {
        let file = File::open(filepath)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let clean = clean_sentence(line?.to_string());
            let mut prev : Option<String> = None;

            for word in clean.split(" ") {
                if prev.is_some() { self.add_pair(prev.unwrap(), word.to_string()) };
                prev = Some(word.to_string());
            }
        }

        Ok(true)
    }

    fn add_pair(&mut self, from : String, to : String) {
        let result = self.values.entry(from).or_insert((to, 0));
        result.1 += 1;
    }
}

fn clean_sentence(s : String) -> String {
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_pair() {
        let key : String = "key".to_string();
        let value : String = "value".to_string();
        let mut markov : MarkovChain = MarkovChain::new();

        markov.add_pair(key.clone(), value.clone());

        assert_eq!(markov.values.get(&key).unwrap(), &(value, 1 as u32));
    }
}
