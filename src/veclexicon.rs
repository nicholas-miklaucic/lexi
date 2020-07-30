//! Implements `Lexicon` in the most basic way, using a `Vec` to store strings.
//! This provides O(n) performance for all of the operations in the `Lexicon`
//! trait, where n is the size of the lexicon and the size of words is
//! negligible compared to the size of the lexicon.
//!
//! This lexicon is case-insensitive, and converts everything to lowercase internally.

use crate::lexicon::Lexicon;

/// A simple list of words.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VecLexicon {
    /// The words in the list, all lowercase.
    words: Vec<String>
}

impl VecLexicon {
    /// Creates a new lexicon with the given words, in lowercase.
    pub fn new(words: Vec<String>) -> VecLexicon {
        VecLexicon{words}
    }
}

impl From<Vec<String>> for VecLexicon {
    fn from(words: Vec<String>) -> Self {
        VecLexicon::new(words)
    }
}

impl IntoIterator for VecLexicon {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

impl Lexicon for VecLexicon {
    /// Returns `true` if the word list contains the given word and `false`
    /// otherwise.
    fn contains(&self, word: &str) -> bool {
        self.words.contains(&String::from(word))
    }

    /// Keeps only the words in the list with the given letter.
    fn with_letter(&mut self, letter: char) {
        self.words.retain(|word| word.contains(letter));
    }

    /// Keeps only the words in the list without the given letter.
    fn without_letter(&mut self, letter: char) {
        self.words.retain(|word| !word.contains(letter));
    }

    /// Keeps only the words that only contain the given letters. Words that
    /// don't use all of the given letters are kept, unlike `with_letters.`
    fn only_using_letters<T: IntoIterator<Item = char>>(&mut self, letters: T) {
        let string: String = letters.into_iter().collect();
        self.words.retain(|word| word.chars().all(|l| string.contains(l)));
    }

    fn with_exact_length(&mut self, length: usize) {
        self.words.retain(|word| word.len() == length);
    }

    fn with_more_length(&mut self, length: usize) {
        self.words.retain(|word| word.len() > length);
    }

    fn with_less_length(&mut self, length: usize) {
        self.words.retain(|word| word.len() < length);
    }
}
