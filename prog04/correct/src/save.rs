use std::fs;

use triemap::TrieMap;

// ************************************************************************* //
// to_btcz
// ************************************************************************* //

pub fn to_btcz(model: &TrieMap<u64>, model_file_name: &str) -> bincode::Result<()> {
    bincode::serialize_into(
        flate2::write::ZlibEncoder::new(
            fs::File::create(model_file_name)?,
            flate2::Compression::best(),
        ),
        model,
    )
}
