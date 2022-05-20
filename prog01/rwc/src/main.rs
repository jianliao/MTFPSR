use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = match args.len() {
        1 => "-",
        2 => &args[1],
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
    let src: String = if file != "-" {
        fs::read_to_string(file).expect(&format!("Failed to read {}", file)[..])
    } else {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Failed to read stdin");
        buf
    };
    let (newlines, words, chars): (u64, u64, u64) = newlines_words_chars(&src[..]);
    println!("{} {} {} {}", newlines, words, chars, file)
}

fn newlines_words_chars(src: &str) -> (u64, u64, u64) {
    const NEWLINE: char = '\u{000A}';

    let mut newline_counter: u64 = 0;
    let mut word_counter: u64 = 0;
    let mut char_counter: u64 = 0;
    let mut start_word: bool = false;

    for c in src.chars() {
        char_counter += 1;
        if NEWLINE == c {
            newline_counter += 1;
        }

        if start_word && (NEWLINE == c || c.is_ascii_whitespace()) {
            word_counter += 1;
            start_word = false;
        } else if !start_word  && NEWLINE != c && !c.is_ascii_whitespace()  {
            start_word = true;
        }
    }

    if start_word {
        word_counter += 1;
    };

    (newline_counter, word_counter, char_counter)
}

#[cfg(test)]
mod tests {
    use crate::newlines_words_chars;
    use std::fs;

    #[test]
    fn test01_hello_world_string() {
        assert_eq!(newlines_words_chars("Hello, World!\n"), (1, 2, 14))
    }

    #[test]
    fn test02_go_tigers_string() {
        assert_eq!(newlines_words_chars("Go Tigers!\n"), (1, 2, 11))
    }

    #[test]
    fn test03_one_two_three_four_five_six_seven_string() {
        assert_eq!(
            newlines_words_chars("one\ntwo three\n four \nfive six seven"),
            (3, 7, 35)
        )
    }

    fn test_file(file: &str, res: (u64, u64, u64)) {
        let src: String = fs::read_to_string(file).expect(&format!("Failed to read {}", file));
        assert_eq!(newlines_words_chars(&src), res)
    }

    #[test]
    fn test04_fruits() {
        test_file("assets/fruits.txt", (4, 8, 41))
    }

    #[test]
    fn test05_gettysburg() {
        test_file("assets/gettysburg.txt", (36, 279, 1505))
    }

    #[test]
    fn test06_oneline() {
        test_file("assets/oneline.txt", (0, 1, 3))
    }

    #[test]
    fn test07_blanks() {
        test_file("assets/blanks.txt", (4, 0, 17))
    }

    #[test]
    fn test08_lines() {
        test_file("assets/lines.txt", (23, 6, 59))
    }

    #[test]
    fn test09_sherlock_holmes() {
        test_file("assets/sherlock_holmes.txt", (12759, 109463, 595402))
    }
}
