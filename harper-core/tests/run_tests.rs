use harper_core::linting::{LintGroup, LintGroupConfig, Linter};
use harper_core::{Document, FstDictionary};

/// Creates a unit test checking that the linting of a Markdown document (in
/// `tests_sources`) produces the expected number of lints.
macro_rules! create_test {
    ($filename:ident.md, $correct_expected:expr) => {
        paste::paste! {
            #[test]
            fn [<lints_ $filename _correctly>](){
                let source = include_str!(
                    concat!(
                        "./test_sources/",
                        concat!(stringify!($filename), ".md")
                    )
                );

                let dict = FstDictionary::curated();
                let document = Document::new_markdown_default(&source, &dict);

                let mut linter = LintGroup::new(
                     LintGroupConfig::default(),
                     dict
                );
                let lints = linter.lint(&document);

                dbg!(&lints);
                assert_eq!(lints.len(), $correct_expected);

                // Make sure that all generated tokens span real characters
                for token in document.tokens(){
                     assert!(token.span.try_get_content(document.get_source()).is_some());
                }
            }
        }
    };
}

create_test!(whack_bullets.md, 1);
create_test!(preexisting.md, 0);
create_test!(issue_109.md, 0);
create_test!(issue_109_ext.md, 0);
create_test!(chinese_lorem_ipsum.md, 2);
create_test!(obsidian_links.md, 2);
create_test!(issue_267.md, 0);
create_test!(proper_noun_capitalization.md, 2);
create_test!(amazon_hostname.md, 0);
create_test!(issue_159.md, 1);
create_test!(issue_358.md, 0);
create_test!(issue_195.md, 0);
create_test!(issue_118.md, 0);
create_test!(lots_of_latin.md, 0);
create_test!(pr_452.md, 2);

// Make sure it doesn't panic
create_test!(lukas_homework.md, 3);
