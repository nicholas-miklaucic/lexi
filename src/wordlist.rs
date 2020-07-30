//! This file is responsible for parsing the word list used by default in Lexi.
//! Lexi currently uses the
//! [2of12inf](http://wordlist.aspell.net/12dicts-readme/#2of12inf) word list
//! from Alan Beale, and the rights for the list are as described there. This
//! list excludes capitalizations, abbrevations, etc., but includes swears.
//! There are three flags that control what words are excluded:
//!  - Plurals of uncountable nouns. These are often almost never used, but can
//!  often be argued as correct. Consider, for instance, the noun "bread", which
//!  is generally considered to not take an 's'. Yet the sentence "They had many
//!  different breads at the store" still parses as correct. The line here is
//!  very difficult to find, so it's a flag instead. (Note that "bread" is also
//!  a verb, like breading eggplant, so "breads" is in both lists.)
//!  - Obscene words. Specifically, because the source list doesn't distinguish
//!  these, I use a separate list found
//!  [here.](https://github.com/chucknorris-io/swear-words) This is
//!  by no means complete, and definitely has a few things I'd leave in, but
//!  it's the best I could find.
//!  - Neologisms. As time moves on (I think these were added in 2016), these
//!  become more and more expected, and so it's recommended to include these for
//!  words like "anime" and "blogger" that are pretty standard by now.

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use crate::veclexicon::VecLexicon;

const NEOLOGISM_ANNOT: char = '!';
const UNCOUNTABLE_PLURAL_ANNOT: char = '%';

/// The different flags controlling excluded and included words in the list. See
/// the module-level documentation for more information.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Flag {
    /// Include plurals of nouns that typically don't have them, like
    /// "acrimoniousnesses".
    UncountablePlurals,
    /// Include profanities.
    Swears,
    /// Include newer words. Recommended for many words that have become very
    /// standard, like "barista".
    Neologisms
}

/// A list of words with the ability to filter using the flags defined in `Flag`.
pub struct WordList {
    /// Words that are included in any list.
    normal_words: Vec<String>,
    /// Plurals of uncountable nouns.
    uncountable_plurals: Vec<String>,
    /// Swears.
    swears: Vec<String>,
    /// Neologisms.
    neologisms: Vec<String>,
}

impl WordList {
    /// Returns the list of words with the given flags applied. No guarantees
    /// are made as to order.
    pub(crate) fn custom_list<T: IntoIterator<Item = Flag>>(self, flags: T) -> Vec<String> {
        let mut list = self.normal_words.clone();
        let flags_iter: Vec<Flag> = flags.into_iter().collect();
        if flags_iter.contains(&Flag::UncountablePlurals) {
            list.extend(self.uncountable_plurals.into_iter());
        }

        if flags_iter.contains(&Flag::Swears) {
            list.extend(self.swears.into_iter());
        }

        if flags_iter.contains(&Flag::Neologisms) {
            list.extend(self.neologisms.into_iter());
        }

        list
    }

    /// Returns the default list, with neologisms but without swears and
    /// uncountable plurals.
    pub(crate) fn default_list(self) -> Vec<String> {
        self.custom_list(vec![Flag::Neologisms])
    }
}

impl From<WordList> for Vec<String> {
    fn from(words: WordList) -> Self {
        words.default_list()
    }
}

impl From<WordList> for VecLexicon {
    fn from(words: WordList) -> Self {
        VecLexicon::new(words.default_list())
    }

}

/// Generates a WordList from the two input files. The first one is the main
/// word list, and marks neologisms with a trailing `!` and uncountable plurals
/// with a trailing `#`. Fails if the file cannot be found or read.
pub fn parse_list<T: AsRef<Path>, U: AsRef<Path>>(main_list: T, swears_list: U) -> Result<WordList> {
    let main_file = File::open(main_list)?;
    let swears_file = File::open(swears_list)?;

    let main_lines = BufReader::new(main_file).lines();
    let swears_lines = BufReader::new(swears_file).lines();

    // I don't think this can fail?
    let swears: Vec<String> = swears_lines.map(|l| l.unwrap()).collect();

    let mut normal_words = vec![];
    let mut uncountable_plurals = vec![];
    let mut neologisms = vec![];

    for line_result in main_lines {
        let line = line_result?;
        let (line_str, word_type) = if line.ends_with(NEOLOGISM_ANNOT) {
            let mut line_trunc: String = String::from(line);
            line_trunc.truncate(line_trunc.len() - NEOLOGISM_ANNOT.len_utf8());
            (line_trunc, Some(Flag::Neologisms))
        } else if line.ends_with(UNCOUNTABLE_PLURAL_ANNOT) {
            let mut line_trunc: String = String::from(line);
            line_trunc.truncate(line_trunc.len() -
                                UNCOUNTABLE_PLURAL_ANNOT.len_utf8());
            (line_trunc, Some(Flag::UncountablePlurals))
        } else {
            (line, None)
        };

        if !swears.contains(&line_str) {
            match word_type {
                Some(Flag::UncountablePlurals) => {
                    uncountable_plurals.push(line_str);
                },
                Some(Flag::Neologisms) => {
                    neologisms.push(line_str);
                }
                Some(Flag::Swears) | None => {
                    normal_words.push(line_str);
                }
            }
        }
    }

    Ok(WordList{
        normal_words,
        uncountable_plurals,
        swears,
        neologisms
    })
}

/// Generates a WordList from the two input strings. The first one is the main
/// word list, and marks neologisms with a trailing `!` and uncountable plurals
/// with a trailing `#`.
pub fn parse_strings(main_list: &str, swears_list: &str) -> Result<WordList> {
    let main_lines = BufReader::new(main_list.as_bytes()).lines();
    let swears_lines = BufReader::new(swears_list.as_bytes()).lines();

    // I don't think this can fail?
    let swears: Vec<String> = swears_lines.map(|l| l.unwrap()).collect();

    let mut normal_words = vec![];
    let mut uncountable_plurals = vec![];
    let mut neologisms = vec![];

    for line_result in main_lines {
        let line = line_result?;
        let (line_str, word_type) = if line.ends_with(NEOLOGISM_ANNOT) {
            let mut line_trunc: String = String::from(line);
            line_trunc.truncate(line_trunc.len() - NEOLOGISM_ANNOT.len_utf8());
            (line_trunc, Some(Flag::Neologisms))
        } else if line.ends_with(UNCOUNTABLE_PLURAL_ANNOT) {
            let mut line_trunc: String = String::from(line);
            line_trunc.truncate(line_trunc.len() -
                                UNCOUNTABLE_PLURAL_ANNOT.len_utf8());
            (line_trunc, Some(Flag::UncountablePlurals))
        } else {
            (line, None)
        };

        if !swears.contains(&line_str) {
            match word_type {
                Some(Flag::UncountablePlurals) => {
                    uncountable_plurals.push(line_str);
                },
                Some(Flag::Neologisms) => {
                    neologisms.push(line_str);
                }
                Some(Flag::Swears) | None => {
                    normal_words.push(line_str);
                }
            }
        }
    }

    Ok(WordList{
        normal_words,
        uncountable_plurals,
        swears,
        neologisms
    })
}
