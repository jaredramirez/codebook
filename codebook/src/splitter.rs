use std::collections::HashSet;

// fn should_skip_word(word: &str) -> bool {
//     if word.len() <= 1 {
//         return true;
//     }

//     false
// }

// pub fn split_into_words(text: &str) -> HashSet<String> {
//     let mut words_to_check = HashSet::new();

//     // Split text into words and handle punctuation
//     for word in text.split(|c: char| !c.is_alphanumeric()) {
//         if word.is_empty() || should_skip_word(word) {
//             continue;
//         }

//         // Handle camelCase and PascalCase
//         let parts = split_camel_case(word);

//         for part in parts {
//             if !should_skip_word(&part) {
//                 words_to_check.insert(part);
//             }
//         }
//     }

//     words_to_check
// }

#[derive(PartialEq)]
enum CharType {
    Lower,
    Upper,
    Digit,
}

#[derive(Debug, PartialEq)]
pub struct SplitCamelCase {
    pub word: String,
    pub start_char: u32,
}

pub fn split_camel_case(s: &str) -> Vec<SplitCamelCase> {
    let mut result = Vec::new();
    let mut current_word = String::new();
    let mut prev_char_type = None;

    for (i, c) in s.chars().enumerate() {
        assert!(
            !c.is_whitespace(),
            "There should be no whitespace in the input."
        );
        let char_type = if c.is_ascii_uppercase() {
            CharType::Upper
        } else if c.is_ascii_digit() {
            CharType::Digit
        } else {
            CharType::Lower
        };

        // Start a new word if:
        // 1. Current char is uppercase and previous was lowercase
        // 2. Current char is uppercase and next char is lowercase (for cases like "XML")
        // 3. Current char is a digit and previous was not
        // 4. Previous char was a digit and current is not
        let should_split = match prev_char_type {
            Some(CharType::Lower) if char_type == CharType::Upper => true,
            Some(CharType::Upper)
                if char_type == CharType::Upper
                    && s.chars()
                        .nth(i + 1)
                        .map_or(false, |next| next.is_ascii_lowercase()) =>
            {
                true
            }
            Some(prev)
                if (prev != CharType::Digit && char_type == CharType::Digit)
                    || (prev == CharType::Digit && char_type != CharType::Digit) =>
            {
                true
            }
            _ => false,
        };

        if should_split && !current_word.is_empty() {
            result.push(SplitCamelCase {
                word: current_word.clone(),
                start_char: (i - current_word.chars().count()) as u32,
            });
            current_word.clear();
        }

        current_word.push(c);
        prev_char_type = Some(char_type);
    }

    if !current_word.is_empty() {
        let start = s.chars().count() - current_word.chars().count();
        result.push(SplitCamelCase {
            word: current_word,
            start_char: start as u32,
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_splitting() {
        let words: Vec<String> = split_camel_case("calculateUserAge")
            .into_iter()
            .map(|s| s.word)
            .collect();
        assert_eq!(words, vec!["calculate", "User", "Age"]);
    }

    #[test]
    fn test_complex_camel_case() {
        let words = split_camel_case("XMLHttpRequest");
        assert_eq!(
            words,
            vec![
                SplitCamelCase {
                    word: "XML".to_string(),
                    start_char: 0
                },
                SplitCamelCase {
                    word: "Http".to_string(),
                    start_char: 3
                },
                SplitCamelCase {
                    word: "Request".to_string(),
                    start_char: 7
                }
            ]
        );
    }

    #[test]
    fn test_number() {
        let words: Vec<String> = split_camel_case("userAge10")
            .into_iter()
            .map(|s| s.word)
            .collect();
        assert_eq!(words, vec!["user", "Age", "10"]);
    }

    #[test]
    fn test_uppercase() {
        let words: Vec<String> = split_camel_case("EXAMPLE")
            .into_iter()
            .map(|s| s.word)
            .collect();
        assert_eq!(words, vec!["EXAMPLE"]);
    }

    #[test]
    fn test_uppercase_first() {
        let words: Vec<String> = split_camel_case("Example")
            .into_iter()
            .map(|s| s.word)
            .collect();
        assert_eq!(words, vec!["Example"]);
    }

    #[test]
    fn test_unicode() {
        let words: Vec<String> = split_camel_case("こんにちは")
            .into_iter()
            .map(|s| s.word)
            .collect();
        assert_eq!(words, vec!["こんにちは"]);
    }
}
