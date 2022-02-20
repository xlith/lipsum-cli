use lipsum::MarkovChain;

pub fn run(input: &str, words: usize) -> String {
    let mut chain = MarkovChain::new();
    chain.learn(input);
    chain.generate(words)
}

#[test]
fn custom_test_1() {
    let text = "It is a period of civil war. \
    Rebel spaceships, striking from a hidden base, have won their first victory against the evil \
    Galactic Empire. During the battle, Rebel spies managed to steal secret plans to the \
    Empire’s ultimate weapon, the DEATH STAR, an armoured space station with enough power to \
    destroy an entire planet.\
    Pursued by the Empire’s sinister agents, Princess Leia races home aboard her starship, \
    custodian of the stolen plans that can save her people and restore freedom to the galaxy….";

    for count in 1..100 {
        let random_text = run(text, count);
        let text_words = random_text.split(" ").collect::<Vec<&str>>();
        assert_eq!(count, text_words.len());
    }
}