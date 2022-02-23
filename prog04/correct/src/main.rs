use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

use clap::{ArgEnum, Parser};
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, ArgEnum)]
#[clap(rename_all = "snake_case")]
enum CorrectMethod {
    ByModel,
    ByVariants,
    ByFilter,
}
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Load file
    #[clap(
        name = "load",
        short,
        long,
        value_name = "corpus.txt|model.btc.z",
        default_value = "./assets/big_model.btc.z"
    )]
    load_file: String,
    /// Correction method
    #[clap(arg_enum, short, long, value_name = "METHOD", default_value_t = CorrectMethod::ByFilter)]
    method: CorrectMethod,
    /// Edit distance
    #[clap(short = 'd', long, value_name = "DIST", default_value_t = 2)]
    edit_dist: usize,
    /// Report allocation and time statistics
    #[clap(short, long)]
    stats: bool,
    /// Words to correct
    #[clap(value_name = "WORD")]
    words: Vec<String>,
}

use std::io;
use std::process;
use std::time::{Duration, Instant};

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let load_file = &args.load_file;
    // Phase 1: Load the model.
    let load_start = Instant::now();
    let model = if load_file.ends_with(".btc.z") {
        match correct::load::from_btcz(load_file) {
            Ok(model) => model,
            Err(err) => {
                eprintln!("Failed to load model {} [{}]", load_file, err);
                process::exit(1);
            }
        }
    } else {
        match correct::load::from_corpus(load_file) {
            Ok(model) => model,
            Err(err) => {
                eprintln!("Failed to load corpus {} [{}]", load_file, err);
                process::exit(1);
            }
        }
    };
    let load_time = {
        let load_time = load_start.elapsed();
        load_time.as_secs_f64() + load_time.subsec_nanos() as f64 / 1.0e9
    };
    println!("@");
    let correct_method = match &args.method {
        CorrectMethod::ByModel => correct::by_model,
        CorrectMethod::ByVariants => correct::by_variants,
        CorrectMethod::ByFilter => correct::by_filter,
    };
    let edit_dist = args.edit_dist;
    // Phase 2: Use model to check individual words.
    let mut correct_duration = Duration::default();
    let mut process_word = |word: &str| {
        if word.is_empty() {
            // Ignore
        } else if !word.chars().all(|c| c.is_ascii_lowercase()) {
            // Words that are not ASCII lowercase are not considered
            println!("{} !!", word)
        } else if model.get(word).is_some() {
            println!("{} --", word)
        } else {
            let start = Instant::now();
            let corr = correct_method(&model, word, edit_dist);
            correct_duration += start.elapsed();
            match corr {
                None => println!("{} ??", word),
                Some(sugg) => {
                    println!("{} => {}", word, &sugg)
                }
            }
        }
    };
    if args.words.is_empty() {
        let mut line = String::new();
        loop {
            line.clear();
            io::stdin().read_line(&mut line)?;
            if line.is_empty() || line == "\n" {
                // Terminate on an empty line
                break;
            }
            for word in line.split_ascii_whitespace() {
                process_word(word);
            }
        }
    } else {
        for word in &args.words {
            process_word(word)
        }
    }

    let correct_time =
        correct_duration.as_secs_f64() + correct_duration.subsec_nanos() as f64 / 1.0e9;
    let stats = GLOBAL.stats();
    if args.stats {
        println!();
        println!("allocated    : {:>13} bytes", stats.bytes_allocated);
        println!("deallocated  : {:>13} bytes", stats.bytes_deallocated);
        println!("reallocated  : {:>13} bytes", stats.bytes_reallocated);
        println!("load time    : {:>13.10} seconds", load_time);
        println!("correct time : {:>13.10} seconds", correct_time);
    }
    Ok(())
}
