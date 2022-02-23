use std::fs;
use std::io::{self, BufRead};

use triemap::TrieMap;

// ************************************************************************* //
// from_corpus
// ************************************************************************* //

// Load corpus and initialize the model
pub fn from_corpus(corpus_file_name: &str) -> Result<TrieMap<u64>, std::io::Error> {
    let mut model: TrieMap<u64> = TrieMap::new();
    let corpus_file = fs::File::open(corpus_file_name)?;
    for line in io::BufReader::new(corpus_file).lines() {
        for word in line?.split_whitespace() {
            // Discard leading and trailing symbols;
            // e.g., ",respectively." ==> "respectively"
            let word = word.trim_matches(|c: char| !c.is_alphabetic());
            // Skip words that are not ASCII lowercase;
            // e.g., "Matthew" (proper names)
            if !word.is_empty() && word.chars().all(|c| c.is_ascii_lowercase()) {
                match model.get_mut(word) {
                    None => {
                        model.insert(word, 1);
                    }
                    Some(n) => *n += 1,
                }
            }
        }
    }
    Ok(model)
}

// ************************************************************************* //
// from_btcz
// ************************************************************************* //

// Load model
pub fn from_btcz(model_file_name: &str) -> bincode::Result<TrieMap<u64>> {
    bincode::deserialize_from(flate2::read::ZlibDecoder::new(fs::File::open(
        model_file_name,
    )?))
}
