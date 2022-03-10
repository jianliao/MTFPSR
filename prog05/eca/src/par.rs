/*

Describe the design of your parallel implementation of `run_eca`.  Especially
comment on what synchronization primitives are used and how data is communicated
or shared between threads.

YOUR ANSWER HERE.

 */

use std::io::Write;

pub fn run_eca<W>(
    threads: usize,
    rule: [bool; 8],
    size: usize,
    steps: usize,
    indices: Vec<usize>,
    write: Option<W>,
) -> ((usize, usize), (usize, usize), usize)
where
    W: Write + Send + 'static,
{
    assert!(threads >= 1);
    assert!(size >= 1);
    assert!(indices.iter().all(|i| *i < size));

    unimplemented!()
}

#[cfg(test)]
mod tests;
