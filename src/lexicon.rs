//! `Lexicon` is the basic trait that powers the library, describing a set of words that can be
//! filtered in various ways.

/// Describes a set of strings that is queryable for specific criteria. Depending on the exact
/// implementation, different operations will be faster or slower.
pub trait Lexicon {
    /// Returns `true` if the lexicon contains the given value and `false` otherwise.
    fn contains(&self, word: &str) -> bool;

    /// Keeps only the words in the `Lexicon` that have the given letter.
    fn with_letter(&mut self, letter: char);

    /// Removes any words from the `Lexicon` that have the given letter.
    fn without_letter(&mut self, letter: char);

    /// Keeps only the words in the `Lexicon` that are formed solely from the letters passed in.
    /// Different from `with_letters` in that, here, words need not have all of the letters from the
    /// input list: they just can't have any letters from outside of the list.
    fn only_using_letters<T: IntoIterator<Item = char>>(&mut self, letters: T);

    /// Keeps only the words in the `Lexicon` that have all of the given letters.
    /// Implemented via chained `with_letter()` calls by default. Different from `only_using_letters`
    /// in that, in this method, the returned words must contain all of the letters given.
    fn with_letters<T: IntoIterator<Item = char>>(&mut self, letters: T) {
        for letter in letters {
            self.with_letter(letter);
        }
    }

    /// Removes all words in the `Lexicon` that have any of the given letters.
    /// Implemented via chained `without_letter()` calls by default.
    fn without_letters<T: IntoIterator<Item = char>>(&mut self, letters: T) {
        for letter in letters {
            self.without_letter(letter);
        }
    }

    /// Keeps only the words in the `Lexicon` that have exactly the given
    /// length. Note that the exact interpretation of this can vary for some
    /// Unicode strings.
    fn with_exact_length(&mut self, length: usize);

    /// Keeps only the words in the `Lexicon` that are longer than the given
    /// length. Note that the exact interpretation of this can vary for some
    /// Unicode strings.
    fn with_more_length(&mut self, length: usize);

    /// Keeps only the words in the `Lexicon` that have less than the given
    /// length. Note that the exact interpretation of this can vary for some
    /// Unicode strings.
    fn with_less_length(&mut self, length: usize);
}
