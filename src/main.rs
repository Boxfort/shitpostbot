mod markov;

use markov::MarkovChain;

fn main() {
    let mut my_chain = MarkovChain::new();
    let result = my_chain.generate("/home/jack/Downloads/Datasets/sentiment labelled sentences/imdb_labelled.txt".to_string());

    let mut word : Option<String> = my_chain.get_next(None);

    while word.clone().is_some() {
        println!("{}", word.clone().unwrap());
        word = my_chain.get_next(word);
    }
}
