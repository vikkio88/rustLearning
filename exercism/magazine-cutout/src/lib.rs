// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut letters_needed: HashMap<char, u32> = HashMap::new();
    let mut letters_owned: HashMap<char, u32> = HashMap::new();

    for word in magazine {
        let cleaned_word = word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''], "");
        for letter in cleaned_word.chars() {
            *letters_owned.entry(letter).or_insert(0) += 1;
        }
    }

    for word in note {
        let cleaned_word = word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''], "");
        for letter in cleaned_word.chars() {
            *letters_needed.entry(letter).or_insert(0) += 1;
        }
    }

    for (letter, count) in letters_needed.iter() {
        let owned = *letters_owned.get(letter).unwrap_or(&0);
        if (owned < *count) {
            return false;
        }
    }

    true
}
