use std::collections::HashSet;

fn tokenise(word: &str) -> Vec<char> {
    let mut result = Vec::new();
    for c in word.to_lowercase().chars() {
        result.push(c);
    }

    result.sort();

    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let letters = tokenise(word);

    for candidate in possible_anagrams {
        if (*candidate.to_lowercase()).eq(&word.to_lowercase()) {
            continue;
        }
        let chopped = tokenise(&candidate);

        if chopped == letters {
            result.insert(candidate);
        }
    }

    result
}
