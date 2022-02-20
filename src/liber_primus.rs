pub fn run(words: usize) -> String {
    if words == 0 {
        return lipsum::lipsum_title();
    }
    lipsum::lipsum_words(words)
}

#[test]
fn liber_primus_test_1() {
    for count in 1..100 {
        let random_text = run(count);
        let text_words = random_text.split(" ").collect::<Vec<&str>>();
        assert_eq!(count, text_words.len());
    }
}