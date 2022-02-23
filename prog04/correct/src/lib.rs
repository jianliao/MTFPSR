use std::collections::{hash_map::Entry as HashMapEntry, HashMap};

use triemap::TrieMap;

/// A (private) helper function to split a [`&str`] into a first character and a
/// remaining slice.
fn str_split_first(s: &str) -> Option<(char, &str)> {
    s.chars().next().map(|c| (c, &s[c.len_utf8()..]))
}

// ************************************************************************* //
// SearchState
// ************************************************************************* //

/// Helper type to remember the "best" variant seen so far.
struct SearchState {
    best: Option<(String, u64, usize)>,
}
impl SearchState {
    /// Create an initial [`SearchState`]
    fn new() -> SearchState {
        SearchState { best: None }
    }
    /// Remember `variant` if it is "better" then the current "best" variant.
    fn evaluate(&mut self, variant: &str, count: u64, edit_dist: usize) {
        match &self.best {
            None => self.best = Some((String::from(variant), count, edit_dist)),
            Some((best_variant, best_count, best_edit_dist)) => {
                if edit_dist < *best_edit_dist
                    || (edit_dist == *best_edit_dist && count > *best_count)
                    || (edit_dist == *best_edit_dist
                        && count == *best_count
                        && variant < best_variant)
                {
                    self.best = Some((String::from(variant), count, edit_dist))
                }
            }
        }
    }
    /// Return the "best" variant seen.
    fn best(self) -> Option<String> {
        self.best.map(|(best_word, _, _)| best_word)
    }
}

// ************************************************************************* //
// `by_model` method
// ************************************************************************* //

/// Computes the [Damerau–Levenshtein
/// distance](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
/// edit distance between two string slices.
///
/// The Damerau–Levenshtein distance between two words is the minimum number of
/// operations, conisting of an insertion, deletion, or substitution of a single
/// character or a transposition of two adjacent characters, required to change
/// one word into another.
///
/// Computing the Damerau–Levenshtein distance is an example of [dynamic
/// programming](https://en.wikipedia.org/wiki/Dynamic_programming).  The
/// [Distance with adjacent
/// transpositions](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Distance_with_adjacent_transpositions)
/// section of the [Damerau–Levenshtein distance Wikipedia
/// article](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
/// gives pseudocode for the algorithm.  The [`dl_edit_dist`] function may
/// assume that the input words are comprised of lowercase ascii letters, so
/// $|\Sigma|$ (the size of the alphabet) is 26.
///
/// Note: The Wikipedia pseudocode uses non-standard indexing conventions.
///
/// Hint: `s.chars().collect::<Vec<char>>()` will convert a `&str` to a
/// `Vec<char>` which will support indexing by a `usize`.
pub fn dl_edit_dist(s1: &str, s2: &str) -> usize {
    assert!(s1.chars().all(|c| c.is_ascii_lowercase()));
    assert!(s2.chars().all(|c| c.is_ascii_lowercase()));
    // Your code here
    unimplemented!()
}

/// Consider each word `word_prime` in the model, compute the
/// (Damerau–Levenshtein edit
/// distance)[https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance]
/// between `word` and `word_prime`, and choose the best word `word_prime` with
/// edit distance less than or equal to `max_edit_dist`.
pub fn by_model(model: &TrieMap<u64>, word: &str, max_edit_dist: usize) -> Option<String> {
    let mut state = SearchState::new();
    for (word_prime, count) in model.iter() {
        let edit_dist = dl_edit_dist(word, &word_prime);
        if edit_dist <= max_edit_dist {
            state.evaluate(&word_prime, *count, edit_dist)
        }
    }
    state.best()
}

// ************************************************************************* //
// `by_variants` method
// ************************************************************************* //

/// Generate each variant `variant` that is reachable from `word` by at most
/// `max_edit_dist` edit actions and choose the best variant `variant` that is
/// in the model.
pub fn by_variants(model: &TrieMap<u64>, word: &str, max_edit_dist: usize) -> Option<String> {
    by_variants1(model, word, max_edit_dist)
}

/// Enumerates all one-edit-action variants of `word`.
///
/// Because the number of one-edit-action variants of a word can be quite large
/// (e.g., over 36K for `"whale"`), we would like to avoid accumulating the
/// variants into an allocated collection (e.g., a `Vec<String>`), especially if
/// the caller would simply move each of the variants into another data
/// structure.  Thus, [`with_one_edit_variants`] takes a `with_variant` callback
/// closure that it calls with each enumerated one-edit-action variant.  This
/// callback closure may move the variant into is own accumulating data
/// structure.
///
/// Arguably, it might be more idiomatic Rust for this function to return an
/// iterator (in particular, an `impl Iterator<Item=String>`), but it is awkward
/// to define the iterator state and the code becomes complex and inverted.
///
/// Using
/// [generators](https://doc.rust-lang.org/unstable-book/language-features/generators.html)
/// would also yield (pun!) a more natural implementation.  However, that
/// feature is not yet stable in Rust.
fn with_one_edit_variants<F>(word: &str, mut with_variant: F)
where
    F: FnMut(String),
{
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
    let splits = || {
        std::iter::once((word, ""))
            .chain(word.char_indices().map(|(i, _)| (&word[..i], &word[i..])))
    };

    // deletes
    for (l, r) in splits() {
        match str_split_first(r) {
            None => (),
            Some((c0, r)) => match str_split_first(r) {
                None => with_variant(format!("{}{}", l, r)),
                Some((c1, _)) => {
                    // in "hello", only delete the second 'l'.
                    if c0 != c1 {
                        with_variant(format!("{}{}", l, r))
                    }
                }
            },
        }
    }
    // transposes
    for (l, r) in splits() {
        match str_split_first(r) {
            None => (),
            Some((c0, r)) => match str_split_first(r) {
                None => (),
                Some((c1, r)) => {
                    // in "hello", don't transpose 'l' and 'l'.
                    if c0 != c1 {
                        with_variant(format!("{}{}{}{}", l, c1, c0, r))
                    }
                }
            },
        }
    }
    // replaces
    for (l, r) in splits() {
        match str_split_first(r) {
            None => (),
            Some((c0, r)) => {
                for c in LETTERS.chars() {
                    // don't replace a letter with itself
                    if c0 != c {
                        with_variant(format!("{}{}{}", l, c, r));
                    }
                }
            }
        }
    }
    // inserts
    for (l, r) in splits() {
        for c in LETTERS.chars() {
            match str_split_first(r) {
                None => with_variant(format!("{}{}{}", l, c, r)),
                Some((c0, _)) => {
                    // in "hello", only insert an 'l' before a non-'l'
                    if c0 != c {
                        with_variant(format!("{}{}{}", l, c, r))
                    }
                }
            }
        }
    }
}

/// Enumerate all variants of `word` reachable by at most `max_edit_dist` edit
/// actions.  The variants are enumerated with (upper bounds on) edit distances.
///
/// As with [`with_one_edit_variants`], [`with_edit_variants`] takes a
/// `with_variant` callback closure that it calls with each enumerated variant.
///
/// The callback closure will be invoked with multiple kinds of redundancy.  For
/// example, the variants of `"whale"` will include `("whaza", 2)` twice,
/// because it is reachable by one edit action from both `"whala"` `"whaze"`.
/// In addition, the variants of `"whale"` will include `("whale", 2)` twice,
/// because it is reachable by one edit action from both `"whala"` `"whaze"`.
/// This redundancy doesn't affect the "best" variant to correct the word.
///
/// We accept the redundancy, because this leads to a faster correction method
/// than maintaing a unique set of variants.
fn with_edit_variants<F>(word: &str, max_edit_dist: usize, mut with_variant: F)
where
    F: FnMut(String, usize),
{
    fn doit<F>(
        word: &str,
        e: usize, // number of edits made to word thus far
        d: usize, // number of edits available to make to word
        with_variant: &mut F,
    ) where
        F: FnMut(String, usize),
    {
        with_variant(String::from(word), e);
        if d > 0 {
            with_one_edit_variants(word, |s| doit(&s, e + 1, d - 1, with_variant))
        }
    }

    doit(word, 0, max_edit_dist, &mut with_variant);
}

#[allow(dead_code)]
fn by_variants1(model: &TrieMap<u64>, word: &str, max_edit_dist: usize) -> Option<String> {
    let mut state = SearchState::new();
    with_edit_variants(word, max_edit_dist, |variant, dist| {
        match model.get(&variant) {
            None => (),
            Some(count) => state.evaluate(&variant, *count, dist),
        }
    });
    state.best()
}

/// Enumerate all variants of `word` reachable by at most `max_edit_dist` edit
/// actions.  The variants are enumerated with (upper bounds on) edit distances.
///
/// This function accumulates all variants in a vector, allowing for multiple
/// kinds of redundancy.  For example, the variants of `"whale"` will include
/// `("whaza", 2)` twice, because it is reachable by one edit action from both
/// `"whala"` `"whaze"`.  In addition, the variants of `"whale"` will include
/// `("whale", 2)` twice, because it is reachable by one edit action from both
/// `"whala"` `"whaze"`.  This redundancy doesn't affect the "best" variant to
/// correct the word.
///
/// We accept the redundancy, because this leads to a faster correction method
/// than maintaing a unique set of variants.  Try replacing `variants_vec` in
/// `by_variants2` with `unique_variants_hashmap` or
/// `unique_variants_trie` and compare the running times.
#[allow(dead_code)]
fn variants_vec(word: &str, max_edit_dist: usize) -> impl Iterator<Item = (String, usize)> {
    let mut res: Vec<(String, usize)> = vec![];
    let mut last = vec![String::from(word)];
    for edit_dist in 1..=max_edit_dist {
        let mut next = Vec::new();
        last.into_iter().for_each(|word| {
            with_one_edit_variants(&word, |variant| {
                if edit_dist < max_edit_dist {
                    next.push(variant.clone());
                };
                res.push((variant, edit_dist));
            })
        });
        last = next;
    }
    res.into_iter()
}

/// Enumerate all variants of `word` reachable by at most `max_edit_dist` edit
/// actions.  The variants are enumerated with edit distances.
///
/// This function accumulates all variants in a [`HashMap`], ensuring
/// uniqueness.
#[allow(dead_code)]
fn unique_variants_hashmap(
    word: &str,
    max_edit_dist: usize,
) -> impl Iterator<Item = (String, usize)> {
    let mut res = HashMap::new();
    res.insert(String::from(word), 0);
    let mut last = vec![String::from(word)];
    for edit_dist in 1..=max_edit_dist {
        let mut next = Vec::new();
        last.into_iter().for_each(|word| {
            with_one_edit_variants(&word, |variant| match res.entry(variant) {
                HashMapEntry::Vacant(entry) => {
                    if edit_dist < max_edit_dist {
                        next.push(entry.key().clone());
                    };
                    entry.insert(edit_dist);
                }
                HashMapEntry::Occupied(_) => (),
            })
        });
        last = next;
    }
    res.remove(word);
    res.into_iter()
}

/// Enumerate all variants of `word` reachable by at most `max_edit_dist` edit
/// actions.  The variants are enumerated with edit distances.
///
/// This function accumulates all variants in a [`TrieMap`], ensuring
/// uniqueness.
#[allow(dead_code)]
fn unique_variants_triemap(
    word: &str,
    max_edit_dist: usize,
) -> impl Iterator<Item = (String, usize)> {
    let mut res = TrieMap::new();
    res.insert(word, 0);
    let mut last = vec![String::from(word)];
    for edit_dist in 1..=max_edit_dist {
        let mut next = Vec::new();
        last.into_iter().for_each(|word| {
            with_one_edit_variants(&word, |variant| match res.get(&variant) {
                None => {
                    res.insert(&variant, edit_dist);
                    if edit_dist < max_edit_dist {
                        next.push(variant);
                    };
                }
                Some(_) => (),
            })
        });
        last = next;
    }
    res.remove(word);
    res.into_iter()
}

#[allow(dead_code)]
fn by_variants2(model: &TrieMap<u64>, word: &str, max_edit_dist: usize) -> Option<String> {
    let mut state = SearchState::new();
    for (variant, dist) in variants_vec(word, max_edit_dist) {
        match model.get(&variant) {
            None => (),
            Some(count) => state.evaluate(&variant, *count, dist),
        }
    }
    state.best()
}

// ************************************************************************* //
// `by_filter` method
// ************************************************************************* //

/// Generates _prefixes_ of variants and filters the model (represented as a
/// trie); when there are no correctly spelled words in the model with a
/// particular prefix, then it is not necessary to generate any of the variants
/// with that prefix.
pub fn by_filter(_model: &TrieMap<u64>, _word: &str, _max_edit_dist: usize) -> Option<String> {
    // Your code here
    unimplemented!()
}

// ************************************************************************* //
// load & save
// ************************************************************************* //

pub mod load;
pub mod save;

// ************************************************************************* //
// tests
// ************************************************************************* //

#[cfg(test)]
mod tests;
