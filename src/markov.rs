
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::fs::File;

extern crate rand;

use markov::rand::distributions::{Distribution, Uniform};

pub struct MarkovChain {
    values : HashMap<Option<String>, HashMap<Option<String>, u32>>,
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
                self.add_pair(prev, Some(word.to_string()));
                prev = Some(word.to_string());
            }
            // Add terminator
            self.add_pair(prev, None);
        }

        Ok(true)
    }

    pub fn get_next(&mut self, word : Option<String>) -> Option<String> {
        let range = Uniform::new(0, 100);
        let mut rng = rand::thread_rng();
        let a = range.sample(&mut rng);

        let total = values.get(word);

        None
    }

    fn add_pair(&mut self, from : Option<String>, to : Option<String>) {
        // Get the existing hashmap or create one if none exists.
        let mut result = self.values.entry(from).or_insert(HashMap::new());
        // Get the occurences of 'to' or set to 0 if it doesn't exist.
        let mut entry = result.entry(to).or_insert(0);
        *entry += 1;
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
        let key : Option<String> = Some("key".to_string());
        let value : Option<String> = Some("value".to_string());
        let mut markov : MarkovChain = MarkovChain::new();

        markov.add_pair(key.clone(), value.clone());
        assert_eq!(markov.values.get(&key).unwrap().get(&value), Some(&1u32));
    }
}
