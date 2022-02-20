pub fn run(words: usize) -> String {
    lipsum::lipsum(words)
}

#[test]
fn lorem_ipsum_test_1() {
    for count in 1..100 {
        let random_text = run(count);
        let text_words = random_text.split(" ").collect::<Vec<&str>>();
        assert_eq!(count, text_words.len());
    }
}
