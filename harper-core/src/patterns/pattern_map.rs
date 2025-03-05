use crate::Token;

use super::Pattern;

/// A map from [`Pattern`] to arbitrary data.
///
/// When used as a [`Pattern`] in of itself, it simply iterates through
/// all contained patterns, returning the first match found.
/// You should not assume this search is deterministic.
///
/// If you'd like to use this structure in a [`PatternLinter`](crate::linting::PatternLinter), you may want to provide
/// the map as the search pattern, then use a pattern look-up once more to determine
/// the corresponding key.
pub struct PatternMap<T>
where
    T: Send + Sync,
{
    rows: Vec<Row<T>>,
}

struct Row<T>
where
    T: Send + Sync,
{
    pub key: Box<dyn Pattern>,
    pub element: T,
}

impl<T> Default for PatternMap<T>
where
    T: Send + Sync,
{
    fn default() -> Self {
        Self {
            rows: Default::default(),
        }
    }
}

impl<T> PatternMap<T>
where
    T: Send + Sync,
{
    pub fn insert(&mut self, pattern: impl Pattern + 'static, value: T) {
        self.rows.push(Row {
            key: Box::new(pattern),
            element: value,
        });
    }

    /// Lookup the corresponding value for the given map.
    pub fn lookup(&self, tokens: &[Token], source: &[char]) -> Option<&T> {
        for row in &self.rows {
            let len = row.key.matches(tokens, source);

            if len != 0 {
                return Some(&row.element);
            }
        }

        None
    }
}

impl<T> Pattern for PatternMap<T>
where
    T: Send + Sync,
{
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        for row in &self.rows {
            let len = row.key.matches(tokens, source);

            if len != 0 {
                return len;
            }
        }

        0
    }
}
