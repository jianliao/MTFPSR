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

    // Your code here
    for (i, el) in letter_freq(&src).iter().filter(|i| **i != 0.0).enumerate() {
        println!("{}: {:.3}%", (b'A' + i as u8) as char, *el * 100.0);
    }
}

fn letter_count(src: &str) -> [u64; 26] {
    let mut res: [u64; 26] = [0; 26];
    for c in src.chars() {
        if c.is_ascii_alphabetic() {
            // let index: usize = c.to_ascii_lowercase() as usize - 'a' as usize;
            let index: usize = (c.to_ascii_lowercase() as u8 - b'a').into(); // Use byte literal
            res[index] += 1;
        }
    }
    res
}

fn count_to_freq(inp: [u64; 26]) -> [f64; 26] {
    let total: u64 = inp.iter().sum();
    inp.map(|i| i as f64 / total as f64)
}

fn letter_freq(src: &str) -> [f64; 26] {
    count_to_freq(letter_count(src))
}

#[cfg(test)]
mod tests {
    mod letter_count {
        use crate::letter_count;
        use std::fs;

        #[test]
        fn test01_hello_world_string() {
            assert_eq!(
                letter_count("Hello, World!\n"),
                [0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 3, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0,]
            )
        }

        #[test]
        fn test02_go_tigers() {
            assert_eq!(
                letter_count("Go Tigers!\n"),
                [0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,]
            )
        }

        #[test]
        fn test03_one_two_three_four_five_six_seven_string() {
            assert_eq!(
                letter_count("one\ntwo three\n four \nfive six seven"),
                [0, 0, 0, 0, 6, 2, 0, 1, 2, 0, 0, 0, 0, 2, 3, 0, 0, 2, 2, 2, 1, 2, 1, 1, 0, 0,]
            )
        }

        fn test_file(file: &str, res: [u64; 26]) {
            let src: String =
                fs::read_to_string(file).expect(&format!("Failed to read {}", file)[..]);
            assert_eq!(letter_count(&src), res)
        }

        #[test]
        fn test04_fruits() {
            test_file(
                "assets/fruits.txt",
                [
                    5, 1, 1, 1, 3, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 2, 0, 2, 0, 1, 0, 0, 0, 0, 1, 0,
                ],
            )
        }

        #[test]
        fn test05_gettysburg() {
            test_file(
                "assets/gettysburg.txt",
                [
                    106, 16, 32, 60, 167, 27, 30, 81, 69, 0, 3, 44, 14, 79, 94, 15, 1, 82, 47, 128,
                    22, 24, 28, 0, 11, 0,
                ],
            )
        }

        #[test]
        fn test06_sherlock_holmes() {
            test_file(
                "assets/sherlock_holmes.txt",
                [
                    36914, 6771, 11380, 19367, 55889, 9527, 8463, 30024, 31885, 555, 3746, 18082,
                    12334, 30247, 35456, 7475, 443, 26227, 28451, 41303, 13658, 4665, 11692, 592,
                    9912, 185,
                ],
            )
        }
    }

    mod count_to_freq {
        use crate::count_to_freq;

        #[test]
        fn test01_hello_world_string() {
            assert_eq!(
                count_to_freq([
                    0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 3, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0,
                ]),
                [
                    0.0, 0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0, 0.2, 0.0,
                    0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0,
                ]
            )
        }

        #[test]
        fn test02_go_tigers_string() {
            assert_eq!(
                count_to_freq([
                    0, 0, 0, 0, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
                ]),
                [
                    0.0, 0.0, 0.0, 0.0, 0.125, 0.0, 0.25, 0.0, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.125, 0.0, 0.0, 0.125, 0.125, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ]
            )
        }

        #[test]
        fn test03_one_two_three_four_five_six_seven_string() {
            assert_eq!(
                count_to_freq([
                    0, 0, 0, 0, 6, 2, 0, 1, 2, 0, 0, 0, 0, 2, 3, 0, 0, 2, 2, 2, 1, 2, 1, 1, 0, 0,
                ]),
                [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.2222222222222222,
                    0.07407407407407407,
                    0.0,
                    0.037037037037037035,
                    0.07407407407407407,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.07407407407407407,
                    0.1111111111111111,
                    0.0,
                    0.0,
                    0.07407407407407407,
                    0.07407407407407407,
                    0.07407407407407407,
                    0.037037037037037035,
                    0.07407407407407407,
                    0.037037037037037035,
                    0.037037037037037035,
                    0.0,
                    0.0,
                ]
            )
        }

        #[test]
        fn test04_fruits() {
            assert_eq!(
                count_to_freq([
                    5, 1, 1, 1, 3, 0, 0, 1, 0, 0, 0, 1, 0, 2, 0, 2, 0, 2, 0, 1, 0, 0, 0, 0, 1, 0,
                ],),
                [
                    0.23809523809523808,
                    0.047619047619047616,
                    0.047619047619047616,
                    0.047619047619047616,
                    0.14285714285714285,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                ]
            )
        }

        #[test]
        fn test05_gettysburg() {
            assert_eq!(
                count_to_freq([
                    106, 16, 32, 60, 167, 27, 30, 81, 69, 0, 3, 44, 14, 79, 94, 15, 1, 82, 47, 128,
                    22, 24, 28, 0, 11, 0,
                ]),
                [
                    0.08983050847457627,
                    0.013559322033898305,
                    0.02711864406779661,
                    0.05084745762711865,
                    0.14152542372881355,
                    0.02288135593220339,
                    0.025423728813559324,
                    0.06864406779661017,
                    0.05847457627118644,
                    0.0,
                    0.002542372881355932,
                    0.03728813559322034,
                    0.011864406779661017,
                    0.06694915254237288,
                    0.07966101694915254,
                    0.012711864406779662,
                    0.000847457627118644,
                    0.06949152542372881,
                    0.03983050847457627,
                    0.10847457627118644,
                    0.01864406779661017,
                    0.020338983050847456,
                    0.023728813559322035,
                    0.0,
                    0.009322033898305085,
                    0.0,
                ],
            )
        }

        #[test]
        fn test06_sherlock_holmes() {
            assert_eq!(
                count_to_freq([
                    36914, 6771, 11380, 19367, 55889, 9527, 8463, 30024, 31885, 555, 3746, 18082,
                    12334, 30247, 35456, 7475, 443, 26227, 28451, 41303, 13658, 4665, 11692, 592,
                    9912, 185,
                ]),
                [
                    0.08108636486447897,
                    0.014873375318236634,
                    0.024997638623767967,
                    0.042542114870519704,
                    0.12276740114620104,
                    0.020927284988456713,
                    0.018590071676006,
                    0.06595159068892877,
                    0.07003951735666446,
                    0.0012191291244456257,
                    0.00822857243274471,
                    0.03971944653734379,
                    0.02709322274038261,
                    0.06644143896776007,
                    0.07788367970512451,
                    0.01641980217158748,
                    0.0009731066705034455,
                    0.057610990174478245,
                    0.06249629318847297,
                    0.09072736977833816,
                    0.03000155960662767,
                    0.010247274532502421,
                    0.025682986888321182,
                    0.0013004043994086674,
                    0.02177298717388296,
                    0.00040637637481520856,
                ]
            )
        }
    }

    mod letter_freq {
        use crate::letter_freq;
        use std::fs;

        #[test]
        fn test01_hello_world_string() {
            assert_eq!(
                letter_freq("Hello, World!\n"),
                [
                    0.0, 0.0, 0.0, 0.1, 0.1, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0, 0.2, 0.0,
                    0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0,
                ]
            )
        }

        #[test]
        fn test02_go_tigers_string() {
            assert_eq!(
                letter_freq("Go Tigers!\n"),
                [
                    0.0, 0.0, 0.0, 0.0, 0.125, 0.0, 0.25, 0.0, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.125, 0.0, 0.0, 0.125, 0.125, 0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                ]
            )
        }

        #[test]
        fn test03_one_two_three_four_five_six_seven_string() {
            assert_eq!(
                letter_freq("one\ntwo three\n four \nfive six seven"),
                [
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.2222222222222222,
                    0.07407407407407407,
                    0.0,
                    0.037037037037037035,
                    0.07407407407407407,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.07407407407407407,
                    0.1111111111111111,
                    0.0,
                    0.0,
                    0.07407407407407407,
                    0.07407407407407407,
                    0.07407407407407407,
                    0.037037037037037035,
                    0.07407407407407407,
                    0.037037037037037035,
                    0.037037037037037035,
                    0.0,
                    0.0,
                ]
            )
        }

        fn test_file(file: &str, res: [f64; 26]) {
            let src: String =
                fs::read_to_string(file).expect(&format!("Failed to read {}", file)[..]);
            assert_eq!(letter_freq(&src), res)
        }

        #[test]
        fn test04_fruits() {
            test_file(
                "assets/fruits.txt",
                [
                    0.23809523809523808,
                    0.047619047619047616,
                    0.047619047619047616,
                    0.047619047619047616,
                    0.14285714285714285,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.09523809523809523,
                    0.0,
                    0.047619047619047616,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.047619047619047616,
                    0.0,
                ],
            )
        }

        #[test]
        fn test05_gettysburg() {
            test_file(
                "assets/gettysburg.txt",
                [
                    0.08983050847457627,
                    0.013559322033898305,
                    0.02711864406779661,
                    0.05084745762711865,
                    0.14152542372881355,
                    0.02288135593220339,
                    0.025423728813559324,
                    0.06864406779661017,
                    0.05847457627118644,
                    0.0,
                    0.002542372881355932,
                    0.03728813559322034,
                    0.011864406779661017,
                    0.06694915254237288,
                    0.07966101694915254,
                    0.012711864406779662,
                    0.000847457627118644,
                    0.06949152542372881,
                    0.03983050847457627,
                    0.10847457627118644,
                    0.01864406779661017,
                    0.020338983050847456,
                    0.023728813559322035,
                    0.0,
                    0.009322033898305085,
                    0.0,
                ],
            )
        }

        #[test]
        fn test06_sherlock_holmes() {
            test_file(
                "assets/sherlock_holmes.txt",
                [
                    0.08108636486447897,
                    0.014873375318236634,
                    0.024997638623767967,
                    0.042542114870519704,
                    0.12276740114620104,
                    0.020927284988456713,
                    0.018590071676006,
                    0.06595159068892877,
                    0.07003951735666446,
                    0.0012191291244456257,
                    0.00822857243274471,
                    0.03971944653734379,
                    0.02709322274038261,
                    0.06644143896776007,
                    0.07788367970512451,
                    0.01641980217158748,
                    0.0009731066705034455,
                    0.057610990174478245,
                    0.06249629318847297,
                    0.09072736977833816,
                    0.03000155960662767,
                    0.010247274532502421,
                    0.025682986888321182,
                    0.0013004043994086674,
                    0.02177298717388296,
                    0.00040637637481520856,
                ],
            )
        }
    }
}
