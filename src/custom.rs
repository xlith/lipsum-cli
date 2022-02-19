use lipsum::MarkovChain;

pub fn run(input: &str, mut words: usize) -> String {
    if words == 0 {
        words = 5;
    }
    let mut chain = MarkovChain::new();
    chain.learn(input);
    chain.generate(words)
}