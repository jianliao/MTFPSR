use std::fs;

use super::{Direction, Farm, FarmParseError, IotCS};
use puzzle::test::{MoveTree, MoveTreeVerifyError};

fn moves_to_string(ms: &[Direction]) -> String {
    let mut s = String::new();
    for m in ms {
        s.push_str(&format!("{}", m))
    }
    s
}

fn load(file_stem: &str) -> Result<Farm, String> {
    let src = fs::read_to_string(format!("./assets/{file_stem}.txt")).unwrap();
    match src.parse::<Farm>() {
        Ok(puzzle) => Ok(puzzle),
        Err(_) => Err(String::from(
            "Farm::from_str of {file_stem} should not fail.",
        )),
    }
}

fn parse_test(file_stem: &str, parse_test_res: Result<(), FarmParseError>) -> Result<(), String> {
    let src = fs::read_to_string(format!("./assets/{file_stem}.txt")).unwrap();
    match (src.parse::<Farm>(), parse_test_res) {
        (Err(_), Err(_)) => Ok(()),
        (Err(_), Ok(_)) => Err(String::from(
            "`Farm::from_str` of {file_stem} should not fail.",
        )),
        (Ok(_), Err(_)) => Err(String::from("`Farm::from_str` of {file_stem} should fail.")),
        (Ok(_), Ok(())) => Ok(()),
    }
}

fn solve_test(
    file_stem: &str,
    puzzle: &Farm,
    solve_test_res: Option<(Vec<Direction>, String)>,
) -> Result<(), String> {
    match (puzzle::solve(IotCS::new(puzzle)), solve_test_res) {
    (None, None) => Ok(()),
    (Some((mvs, _)), None) => Err(format!("{file_stem} has solution ({}), but reference has no solution; likely has an invalid move and/or an incorrect `is_goal()`.", moves_to_string(&mvs))),
    (None, Some((mvs_res,_))) => Err(format!("{file_stem} has no solution, but reference has solution ({}).", moves_to_string(&mvs_res))),
    (Some((mvs,puzzle)), Some((mvs_res,puzzle_res))) => match mvs.len().cmp(&mvs_res.len()) {
      std::cmp::Ordering::Greater => Err(format!("{file_stem} solution ({}) is longer than reference solution ({}).", moves_to_string(&mvs), moves_to_string(&mvs_res))),
      std::cmp::Ordering::Equal => if puzzle.ufo_with_cattle_to_string() == puzzle_res { Ok(()) } else { Err(format!("{file_stem} solution ({}) final UFO config ({}) does not equal reference solution ({}) final UFO config ({}); likely has an invalid move and/or an incorrect `is_goal()`.", moves_to_string(&mvs), puzzle.ufo_with_cattle_to_string(), moves_to_string(&mvs_res), puzzle_res)) },
      std::cmp::Ordering::Less => Err(format!("{file_stem} solution ({}) is shorter than reference solution ({}); likely has an invalid move and/or an incorrect `is_goal()`.", moves_to_string(&mvs), moves_to_string(&mvs_res))),
    },
  }
}

fn moves_test(
    file_stem: &str,
    puzzle: &Farm,
    move_tree: &MoveTree<Direction, String>,
) -> Result<(), String> {
    match move_tree.verify::<_, _, ()>(&IotCS::new(puzzle), |q, q_res| {
        if &q.ufo_with_cattle_to_string() == q_res {
            Ok(())
        } else {
            Err(())
        }
    }) {
        Ok(()) => Ok(()),
        Err((ms, err)) => {
            let s = if ms.is_empty() {
                String::from("{file_stem}")
            } else {
                format!("{file_stem} with moves {}", moves_to_string(&ms))
            };
            match err {
                MoveTreeVerifyError::MissingMove(m) => {
                    Err(format!("{s}, `next()` is missing move {m:?}"))
                }
                MoveTreeVerifyError::ExtraMove(m) => {
                    Err(format!("{s}, `next()` has invalid move {m:?}"))
                }
                MoveTreeVerifyError::ChkState(m, _) => Err(format!(
                    "{s}, `next()` includes move {} leading to state with an incorrect UFO config",
                    moves_to_string(&[m])
                )),
            }
        }
    }
}

mod badinput00 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput00", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput01", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput02 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput02", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput03 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput03", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput04 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput04", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput05 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput05", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput06 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput06", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput07 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput07", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput08 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput08", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput09 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput09", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod badinput10 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("badinput10", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Err(FarmParseError)
    }
}
mod challenge {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("challenge", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("challenge")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("challenge", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/challenge.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("challenge", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 133351
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/challenge.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy00 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy00", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy00")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy00", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy00.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy00", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 43938
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy00.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy01 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy01", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy01")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy01", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy01.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy01", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 93004
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy01.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod easy10 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("easy10", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("easy10")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("easy10", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy10.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("easy10", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 73449
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/easy10.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod hard21 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("hard21", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("hard21")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("hard21", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard21.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("hard21", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 28730
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard21.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod hard30 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("hard30", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("hard30")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("hard30", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard30.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("hard30", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 113725
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/hard30.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod impossible {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("impossible", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("impossible")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("impossible", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("impossible", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 36104
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/impossible.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod medium11 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("medium11", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("medium11")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("medium11", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium11.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("medium11", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 212206
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium11.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod medium20 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("medium20", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("medium20")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("medium20", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium20.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("medium20", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 29962
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/medium20.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod superhard31 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("superhard31", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("superhard31")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("superhard31", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard31.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("superhard31", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 106477
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard31.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod superhard40 {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("superhard40", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("superhard40")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("superhard40", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard40.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("superhard40", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 337584
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/superhard40.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
mod trivial {
    use super::*;

    #[test]
    fn parse() -> Result<(), String> {
        parse_test("trivial", parse_res())
    }
    fn parse_res() -> Result<(), FarmParseError> {
        Ok(())
    }
    fn puzzle() -> Result<Farm, String> {
        load("trivial")
    }
    #[test]
    fn solve() -> Result<(), String> {
        solve_test("trivial", &puzzle()?, solve_res())
    }
    fn solve_res() -> Option<(Vec<Direction>, String)> {
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial.soln.btc.z").unwrap(),
        ))
        .unwrap()
    }
    #[test]
    fn moves() -> Result<(), String> {
        moves_test("trivial", &puzzle()?, &move_tree())
    }
    fn move_tree() -> MoveTree<Direction, String> {
        // move_tree.size() = 22208
        bincode::deserialize_from(flate2::read::ZlibDecoder::new(
            fs::File::open("./assets/btc/trivial.mt.btc.z").unwrap(),
        ))
        .unwrap()
    }
}
