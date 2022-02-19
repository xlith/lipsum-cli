pub fn run(words: usize) -> String {
    if words == 0 {
        return lipsum::lipsum_title();
    }
    lipsum::lipsum_words(words)
}