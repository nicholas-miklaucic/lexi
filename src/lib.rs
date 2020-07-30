#[macro_use]
extern crate lazy_static;

pub mod lexicon;
pub mod veclexicon;
pub mod wordlist;

pub use lexicon::Lexicon;
pub use veclexicon::VecLexicon;

pub const MAIN_WORDLIST_PATH: &'static str = "../lexi/2of12inf.txt";
pub const SWEARS_PATH: &'static str = "../lexi/swears.txt";

// lazy_static! {
//     /// The standard wordlist for word games, derived from the `2of12inf` list
//     /// from [`12dicts`](http://wordlist.aspell.net/12dicts-readme/#2of12inf).
//     /// Plurals of uncountable nouns (e.g., "acnes") are removed, as are swears.
//     /// Neologisms are kept.
//     pub static ref WORDLIST: veclexicon::VecLexicon =
//         wordlist::parse_list(MAIN_WORDLIST_PATH, SWEARS_PATH).unwrap().into();
// }

#[cfg(test)]
mod tests {
    use super::*;
    use super::veclexicon::VecLexicon;
    use super::wordlist::Flag;
    use super::lexicon::Lexicon;

    fn gen_lexicon(flags: Vec<Flag>) -> VecLexicon {
        let list = wordlist::parse_list(MAIN_WORDLIST_PATH, SWEARS_PATH).unwrap();
        list.custom_list(flags).into()
    }

    fn gen_default_lexicon() -> VecLexicon {
        let list = wordlist::parse_list(MAIN_WORDLIST_PATH, SWEARS_PATH).unwrap();
        list.default_list().into()
    }

    #[test]
    fn test_wordlist_gen() {
        let wl1 = gen_default_lexicon();
        assert!(wl1.contains("apple"));
        assert!(!wl1.contains("fuck"));
        assert!(wl1.contains("blogger"));
        assert!(!wl1.contains("acnes"));
        assert!(!wl1.contains("asdkflj"));

        let wl2 = gen_lexicon(vec![Flag::UncountablePlurals, Flag::Swears]);
        assert!(wl2.contains("apple"));
        assert!(wl2.contains("fuck"));
        assert!(!wl2.contains("blogger"));
        assert!(wl2.contains("acnes"));
        assert!(!wl2.contains("asdkflj"));
    }

    #[test]
    fn test_spelling_bee_like() {
        let mut lex = gen_default_lexicon();
        lex.only_using_letters("doughby".chars());
        lex.with_letter('o');
        lex.with_more_length(3);
        for word in lex {
            println!("{}", word);
        }
    }
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
