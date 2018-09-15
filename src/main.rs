use std::collections::HashMap;

struct MarkovChain {
    values : HashMap<String, (String, u32)>,
}

impl MarkovChain {
    fn new() -> MarkovChain {
        MarkovChain {
            values: HashMap::new(),
        }
    }
    
    fn generate(&self, filepath : String) {
        
    }

    fn add_pair(&mut self, from : String, to : String) {
        let mut result = self.values.entry(from).or_insert((to, 0));
        result.1 += 1;
    }
}

fn main() {
    println!("Hello, world!");
}
