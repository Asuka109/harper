use crate::{
    patterns::{EitherPattern, Pattern, SequencePattern, WhitespacePattern, WordSet},
    Token, TokenStringExt,
};

use super::{Lint, LintKind, PatternLinter, Suggestion};

pub struct ChockFull {
    pattern: Box<dyn Pattern>,
}

impl Default for ChockFull {
    fn default() -> Self {
        Self {
            pattern: Box::new(
                SequencePattern::default()
                    .then(Box::new(WordSet::all(&["chalk", "choke"])))
                    .then(Box::new(EitherPattern::new(vec![
                        Box::new(WhitespacePattern),
                        Box::new(|tok: &Token, _source: &[char]| tok.kind.is_hyphen()),
                    ])))
                    .then_exact_word("full"),
            ),
        }
    }
}

impl PatternLinter for ChockFull {
    fn pattern(&self) -> &dyn Pattern {
        self.pattern.as_ref()
    }

    fn match_to_lint(&self, matched_toks: &[Token], source: &[char]) -> Lint {
        let span = matched_toks.span().unwrap();

        Lint {
            span,
            lint_kind: LintKind::WordChoice,
            suggestions: vec![Suggestion::replace_with_match_case_str(
                "chock-full",
                span.get_content(source),
            )],
            message: format!(
                "The standard term is \"chock-full\"{}.",
                if matched_toks[1].kind.is_whitespace() {
                    ", and it should be hyphenated"
                } else {
                    ""
                }
            ),
            priority: 126,
        }
    }

    fn description(&self) -> &'static str {
        "Flags common soundalikes of \"chock-full\" and makes sure they're hyphenated."
    }
}

#[cfg(test)]
mod tests {
    use super::ChockFull;
    use crate::linting::tests::{assert_lint_count, assert_suggestion_result};

    #[test]
    fn allows_correct_form() {
        assert_lint_count(
            "'Chalk full', 'chalk-full', 'choke full', and 'choke-full' are nonstandard forms of 'chock-full'.",
            ChockFull::default(),
            4,
        );
    }

    #[test]
    fn lower_space_chalk() {
        assert_suggestion_result(
            "The codebase is chalk full of errors that we need to address.",
            ChockFull::default(),
            "The codebase is chock-full of errors that we need to address.",
        );
    }

    #[test]
    fn lower_space_choke() {
        assert_suggestion_result(
            "The project is choke full of questionable decisions that we need to revisit.",
            ChockFull::default(),
            "The project is chock-full of questionable decisions that we need to revisit.",
        );
    }

    #[test]
    fn upper_space_chalk() {
        assert_suggestion_result(
            "Chalk full of deprecated methods; we should refactor.",
            ChockFull::default(),
            "Chock-full of deprecated methods; we should refactor.",
        );
    }

    #[test]
    fn upper_space_choke() {
        assert_suggestion_result(
            "Choke full of unnecessary complexity; simplify it.",
            ChockFull::default(),
            "Chock-full of unnecessary complexity; simplify it.",
        );
    }

    #[test]
    fn lower_hyphen_chalk() {
        assert_suggestion_result(
            "The code is chalk-full of bugs; we need to debug before release.",
            ChockFull::default(),
            "The code is chock-full of bugs; we need to debug before release.",
        );
    }

    #[test]
    fn lower_hyphen_choke() {
        assert_suggestion_result(
            "The project is choke-full of warnings; we should address them.",
            ChockFull::default(),
            "The project is chock-full of warnings; we should address them.",
        );
    }

    #[test]
    fn upper_hyphen_chalk() {
        assert_suggestion_result(
            "Chalk-full of features, but we only need a few.",
            ChockFull::default(),
            "Chock-full of features, but we only need a few.",
        );
    }

    #[test]
    fn upper_hyphen_choke() {
        assert_suggestion_result(
            "Choke-full of pitfalls; let's consider alternatives.",
            ChockFull::default(),
            "Chock-full of pitfalls; let's consider alternatives.",
        );
    }
}
