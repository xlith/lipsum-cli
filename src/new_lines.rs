/// NOTE: New lines before the insertion are removed
pub fn ensure_new_lines_by(input: String, words_per_line: usize) -> String {
    if input.len() < words_per_line {
        input
    } else {
        const NEW_LINE: char = '\n';

        let mut output = String::with_capacity(input.len());
        let mut counter: usize = 0;

        for next_word in input.replace(NEW_LINE, "").split_whitespace() {
            output.push_str(next_word);
            counter += 1;
            if counter == words_per_line {
                counter = 0;
                output.push('\n');
            } else {
                output.push(' ');
            }
        }

        // Makes sure the no new line or space comes after last word
        let take_by = output.len() - 1;
        output.chars().take(take_by).collect()
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn should_have_new_lines() {
        assert_case(
            "Consuetudinum instituendarum voluntates fieri propter voluptatem; quod vero securi percussit filium, privavisse se etiam videtur multis voluptatibus, cum ipsi naturae.",
            "Consuetudinum instituendarum voluntates fieri propter\nvoluptatem; quod vero securi percussit\nfilium, privavisse se etiam videtur\nmultis voluptatibus, cum ipsi naturae.",
            5
        );
        assert_case("Not to be changed", "Not to be changed", 0);
        assert_case("Hello world, out there.", "Hello\nworld,\nout\nthere.", 1);
        assert_case("Natura postulet non intellegunt, errore maximo, si Epicurum audire voluerint, liberabuntur: istae enim vestrae", 
            "Natura postulet non\nintellegunt, errore maximo,\nsi Epicurum audire\nvoluerint, liberabuntur: istae\nenim vestrae", 3);
        // New lines from input should be ignored
        assert_case("Natura postulet\n non intellegunt\n, errore maximo, si Epicurum audire\n voluerint, liberabuntur\n: istae enim vestrae", 
            "Natura postulet non\nintellegunt, errore maximo,\nsi Epicurum audire\nvoluerint, liberabuntur: istae\nenim vestrae", 3);
        fn assert_case(input: &str, expected: &str, lines: usize) {
            let actual = ensure_new_lines_by(input.to_string(), lines);
            assert_eq!(expected, actual);
        }
    }
}
