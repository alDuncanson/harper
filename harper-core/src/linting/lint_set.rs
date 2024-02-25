use paste::paste;

use super::long_sentences::LongSentences;
use super::matcher::Matcher;
use super::repeated_words::RepeatedWords;
use super::sentence_capitalization::SentenceCapitalization;
use super::spaces::Spaces;
use super::spell_check::SpellCheck;
use super::unclosed_quotes::UnclosedQuotes;
use super::wrong_quotes::WrongQuotes;
use super::Linter;
use crate::{Dictionary, Document, Lint};

pub struct LintSet {
    pub(super) linters: Vec<Box<dyn Linter>>
}

impl Linter for LintSet {
    fn lint(&mut self, document: &Document) -> Vec<Lint> {
        let mut lints = Vec::new();

        for linter in &mut self.linters {
            lints.append(&mut linter.lint(document));
        }

        lints.sort_by_key(|lint| lint.span.start);

        lints
    }
}

impl LintSet {
    pub fn new() -> Self {
        Self {
            linters: Vec::new()
        }
    }

    pub fn add_standard(&mut self, dictionary: impl Dictionary + 'static) -> &mut Self {
        self.add_repeated_words()
            .add_long_sentences()
            .add_unclosed_quotes()
            .add_sentence_capitalization()
            .add_matcher()
            .add_spell_check(dictionary)
            .add_spaces();
        self
    }

    pub fn with_standard(mut self, dictionary: impl Dictionary + 'static) -> Self {
        self.add_standard(dictionary);
        self
    }

    pub fn add_spell_check(&mut self, dictionary: impl Dictionary + 'static) -> &mut Self {
        self.linters.push(Box::new(SpellCheck::new(dictionary)));
        self
    }

    pub fn with_spell_check(mut self, dictionary: impl Dictionary + 'static) -> Self {
        self.add_spell_check(dictionary);
        self
    }
}

impl Default for LintSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Create builder methods for the linters that do not take any arguments.
macro_rules! create_simple_builder_methods {
    ($($linter:ident),*) => {
        impl LintSet {
            paste! {
                $(
                    #[doc = "Modifies self, adding the `" $linter "` linter to the set."]
                    pub fn [<add_$linter:snake>](&mut self) -> &mut Self{
                        self.linters.push(Box::<$linter>::default());
                        self
                    }
                )*
            }

            paste! {
                $(
                    #[doc = "Consumes self, adding the `" $linter "` linter to the set."]
                    pub fn [<with_$linter:snake>](mut self) -> Self{
                        self.[<add_$linter:snake>]();
                        self
                    }
                )*
            }
        }
    };
}

create_simple_builder_methods!(
    SentenceCapitalization,
    UnclosedQuotes,
    WrongQuotes,
    LongSentences,
    RepeatedWords,
    Spaces,
    Matcher
);
