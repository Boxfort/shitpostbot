use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::fs::File;

extern crate rand;

use markov::rand::distributions::{Distribution, Uniform};


pub struct MarkovChain {
    values : HashMap<Option<String>, Vec<(Option<String>, u32, u32)>>,
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

        // Generate the markov chain
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
        // Sort the chain

        // Add cumulative weights

        Ok(true)
    }

    pub fn add_line(&mut self, line : String) {
        let clean = clean_sentence(line);

        let mut prev : Option<String> = None;
        for word in clean.split(" ") {
            self.add_pair(prev, Some(word.to_string()));
            prev = Some(word.to_string());
        }
    }

    pub fn add_vec(&mut self, lines : Vec<String>) {
        for line in lines {
            let clean = clean_sentence(line);
            let mut prev : Option<String> = None;

            for word in clean.split(" ") {
                self.add_pair(prev, Some(word.to_string()));
                prev = Some(word.to_string());
            }
            // Add terminator
            self.add_pair(prev, None);
        }
    }

    /// Gets the next word in the chain.
    ///
    /// Returns None if the word does not exist in the chain.
    pub fn get_next(&mut self, word : Option<String>) -> Option<String> {
        if self.values.get(&word).is_none() { return None }

        // In range of 0 to max cumulative weight
        let range = Uniform::new(0, self.values[&word].iter().last().unwrap().2);
        let mut rng = rand::thread_rng();
        let chance = range.sample(&mut rng);

        let result = self.values[&word].binary_search_by_key(&chance, |x| x.2);

        // Return closest index found
        match result {
            Ok(x) =>  {
                //println!("CHANCE - {}, IDX - {}", chance, x);
                self.values[&word][x].0.clone()
            },
            Err(x) => {
                //println!("CHANCE - {}, IDX - {}", chance, x);
                self.values[&word][x].0.clone()
            },
        }
    }

    /// Adds a word pair to the hashmap.
    ///
    /// If the word pair already exists then the weight of the pair
    /// is increased, otherwise it creates a new word pair.
    /// The list of word pairs is then sorted by weight, and
    /// the cumulative weight is calculated for each entry.
    fn add_pair(&mut self, from : Option<String>, to : Option<String>) {

        {
            // Get the existing vec or create one if none exists.
            let entries = self.values.entry(from.clone())
                                   .or_insert(vec!((to.clone(), 0, 0)));


            // Find the entry in the vec corresponding to 'to' or insert it.
            if !entries.iter().find(|x| x.0 == to ).is_some() {
                entries.insert(0, (to.clone(), 0, 0))
            }

            // Increase the weight by one
            for x in entries.iter_mut() {
                if x.0 == to {
                    x.1 += 1;
                    break;
                }
            }
        }

        // Sort the vec by weight.
        self.values.get_mut(&from).unwrap().sort_by_key(|k| k.1);

        // Calculate cumulative weights
        let mut last = 0;
        for entry in self.values.get_mut(&from).unwrap().iter_mut() {
            entry.2 = entry.1 + last;
            last = entry.2;
        }
    }
}

fn clean_sentence(s : String) -> String {
    s
}

// TODO: Tests
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
