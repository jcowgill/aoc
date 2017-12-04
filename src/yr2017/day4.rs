use id;

/// Verifies a list of passwords
///  Splits the input string into lines and words, passes the words through "word_func" and returns
///  the number of lines which do not contain duplicate words
fn password_checker<'a, T>(input: &'a str, word_func: fn(&'a str) -> T) -> String where T: Ord {
    input.lines().filter(|line| {
        // A password is valid if we sort it, dedup it and the size is unchanged
        let mut words: Vec<T> = line.split_whitespace().map(word_func).collect();
        let orig_len = words.len();
        words.sort_unstable();
        words.dedup();
        orig_len == words.len()
    }).count().to_string()
}

/// Sum list of strings containing no duplicate words
pub fn star1(input: &str) -> String {
    password_checker(input, id)
}

/// Sum list of strings containing no words which are anagrams of each other
pub fn star2(input: &str) -> String {
    /// Sorts the characters in a string
    fn sort_str_chars(word: &str) -> String {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort_unstable();
        chars.iter().collect()
    }

    // To check anagrams, sort the characters of each word first
    password_checker(input, sort_str_chars)
}
