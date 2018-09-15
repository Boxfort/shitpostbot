
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
        if self.values.contains_key(&word) { return None }


        let mut total : u32 = 0;
        // Entry is a HashMap<Option<String>, u32>
        let mut entry = self.values.get(&word).unwrap();
        entry.keys()
             .for_each(|x| { entry.get(x)
                                  .and_then(|y| Some(total += y) ); }
                      );

        let range = Uniform::new(0f64, 1f64);
        let mut rng = rand::thread_rng();
        let chance = range.sample(&mut rng);

        let mut next : &Option<String> = &None;

        for key in entry.keys() {
            if chance < (*entry.get(key).unwrap() as f64 / total as f64) {
                next = key;
            }
        }

        next.clone()
    }

    fn add_pair(&mut self, from : Option<String>, to : Option<String>) {
        // Get the existing hashmap or create one if none exists.
        let count = self.values.entry(from)
                               .or_insert(HashMap::new())
                               .entry(to)
                               .or_insert(0);
        *count += 1;
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
