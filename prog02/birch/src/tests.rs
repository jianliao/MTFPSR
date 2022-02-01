use super::{Command::*, Prog, ProgExecError, ProgParseError};
use std::fs;

#[test]
fn test_parse_10_ifz_cmd_07() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_07_res())
}
fn test_parse_10_ifz_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_07())
}
fn prog_10_ifz_cmd_07() -> Prog {
    Prog(vec![Ifz, Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_07() {
    assert_eq!(
        prog_10_ifz_cmd_07().exec(false),
        test_exec_10_ifz_cmd_07_res()
    )
}
fn test_exec_10_ifz_cmd_07_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_004() {
    let src = fs::read_to_string("./assets/09_gt_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_004_res())
}
fn test_parse_09_gt_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_004())
}
fn prog_09_gt_cmd_004() -> Prog {
    Prog(vec![Gt, Num(3), Num(-2147483648)])
}
#[test]
fn test_exec_09_gt_cmd_004() {
    assert_eq!(
        prog_09_gt_cmd_004().exec(false),
        test_exec_09_gt_cmd_004_res()
    )
}
fn test_exec_09_gt_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_03_sub_cmd_000() {
    let src = fs::read_to_string("./assets/03_sub_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_000_res())
}
fn test_parse_03_sub_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_000())
}
fn prog_03_sub_cmd_000() -> Prog {
    Prog(vec![Sub, Num(2147483647), Num(-2147483646)])
}
#[test]
fn test_exec_03_sub_cmd_000() {
    assert_eq!(
        prog_03_sub_cmd_000().exec(false),
        test_exec_03_sub_cmd_000_res()
    )
}
fn test_exec_03_sub_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(4294967293)
}

#[test]
fn test_parse_20_misc_19() {
    let src = fs::read_to_string("./assets/20_misc_19.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_19_res())
}
fn test_parse_20_misc_19_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_19())
}
fn prog_20_misc_19() -> Prog {
    Prog(vec![Mul, Num(1)])
}
#[test]
fn test_exec_20_misc_19() {
    assert_eq!(prog_20_misc_19().exec(false), test_exec_20_misc_19_res())
}
fn test_exec_20_misc_19_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_21() {
    let src = fs::read_to_string("./assets/20_misc_21.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_21_res())
}
fn test_parse_20_misc_21_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_21())
}
fn prog_20_misc_21() -> Prog {
    Prog(vec![Mul, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_21() {
    assert_eq!(prog_20_misc_21().exec(false), test_exec_20_misc_21_res())
}
fn test_exec_20_misc_21_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_20_misc_35() {
    let src = fs::read_to_string("./assets/20_misc_35.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_35_res())
}
fn test_parse_20_misc_35_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_35())
}
fn prog_20_misc_35() -> Prog {
    Prog(vec![Div, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_35() {
    assert_eq!(prog_20_misc_35().exec(false), test_exec_20_misc_35_res())
}
fn test_exec_20_misc_35_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_51() {
    let src = fs::read_to_string("./assets/20_misc_51.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_51_res())
}
fn test_parse_20_misc_51_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_51())
}
fn prog_20_misc_51() -> Prog {
    Prog(vec![Eq, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_51() {
    assert_eq!(prog_20_misc_51().exec(false), test_exec_20_misc_51_res())
}
fn test_exec_20_misc_51_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_017() {
    let src = fs::read_to_string("./assets/05_div_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_017_res())
}
fn test_parse_05_div_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_017())
}
fn prog_05_div_cmd_017() -> Prog {
    Prog(vec![Div, Num(-10), Num(-2147483647)])
}
#[test]
fn test_exec_05_div_cmd_017() {
    assert_eq!(
        prog_05_div_cmd_017().exec(false),
        test_exec_05_div_cmd_017_res()
    )
}
fn test_exec_05_div_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_17() {
    let src = fs::read_to_string("./assets/20_misc_17.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_17_res())
}
fn test_parse_20_misc_17_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_17())
}
fn prog_20_misc_17() -> Prog {
    Prog(vec![Sub, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_17() {
    assert_eq!(prog_20_misc_17().exec(false), test_exec_20_misc_17_res())
}
fn test_exec_20_misc_17_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_06() {
    let src = fs::read_to_string("./assets/13_swap_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_06_res())
}
fn test_parse_13_swap_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_06())
}
fn prog_13_swap_cmd_06() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_06() {
    assert_eq!(
        prog_13_swap_cmd_06().exec(false),
        test_exec_13_swap_cmd_06_res()
    )
}
fn test_exec_13_swap_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_15_cmds_cmd_07() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_07_res())
}
fn test_parse_15_cmds_cmd_07_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_06_rem_cmd_012() {
    let src = fs::read_to_string("./assets/06_rem_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_012_res())
}
fn test_parse_06_rem_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_012())
}
fn prog_06_rem_cmd_012() -> Prog {
    Prog(vec![Rem, Num(-2), Num(10)])
}
#[test]
fn test_exec_06_rem_cmd_012() {
    assert_eq!(
        prog_06_rem_cmd_012().exec(false),
        test_exec_06_rem_cmd_012_res()
    )
}
fn test_exec_06_rem_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_04_mul_cmd_015() {
    let src = fs::read_to_string("./assets/04_mul_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_015_res())
}
fn test_parse_04_mul_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_015())
}
fn prog_04_mul_cmd_015() -> Prog {
    Prog(vec![Mul, Num(-5), Num(-3)])
}
#[test]
fn test_exec_04_mul_cmd_015() {
    assert_eq!(
        prog_04_mul_cmd_015().exec(false),
        test_exec_04_mul_cmd_015_res()
    )
}
fn test_exec_04_mul_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(15)
}

#[test]
fn test_parse_62_fib() {
    let src = fs::read_to_string("./assets/62_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_62_fib_res())
}
fn test_parse_62_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_62_fib())
}
fn prog_62_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(30),
    ])
}
#[test]
fn test_exec_62_fib() {
    assert_eq!(prog_62_fib().exec(false), test_exec_62_fib_res())
}
fn test_exec_62_fib_res() -> Result<i64, ProgExecError> {
    Ok(1346269)
}

#[test]
fn test_parse_02_add_cmd_013() {
    let src = fs::read_to_string("./assets/02_add_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_013_res())
}
fn test_parse_02_add_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_013())
}
fn prog_02_add_cmd_013() -> Prog {
    Prog(vec![Add, Num(10), Num(3)])
}
#[test]
fn test_exec_02_add_cmd_013() {
    assert_eq!(
        prog_02_add_cmd_013().exec(false),
        test_exec_02_add_cmd_013_res()
    )
}
fn test_exec_02_add_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(13)
}

#[test]
fn test_parse_07_eq_cmd_006() {
    let src = fs::read_to_string("./assets/07_eq_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_006_res())
}
fn test_parse_07_eq_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_006())
}
fn prog_07_eq_cmd_006() -> Prog {
    Prog(vec![Eq, Num(0), Num(0)])
}
#[test]
fn test_exec_07_eq_cmd_006() {
    assert_eq!(
        prog_07_eq_cmd_006().exec(false),
        test_exec_07_eq_cmd_006_res()
    )
}
fn test_exec_07_eq_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_15_cmds_cmd_02() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_02_res())
}
fn test_parse_15_cmds_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_02())
}
fn prog_15_cmds_cmd_02() -> Prog {
    Prog(vec![Num(3), Cmds(vec![Num(2), Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_02() {
    assert_eq!(
        prog_15_cmds_cmd_02().exec(false),
        test_exec_15_cmds_cmd_02_res()
    )
}
fn test_exec_15_cmds_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_16_exec_cmd_00() {
    let src = fs::read_to_string("./assets/16_exec_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_00_res())
}
fn test_parse_16_exec_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_00())
}
fn prog_16_exec_cmd_00() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_16_exec_cmd_00() {
    assert_eq!(
        prog_16_exec_cmd_00().exec(false),
        test_exec_16_exec_cmd_00_res()
    )
}
fn test_exec_16_exec_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_20_misc_46() {
    let src = fs::read_to_string("./assets/20_misc_46.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_46_res())
}
fn test_parse_20_misc_46_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_46())
}
fn prog_20_misc_46() -> Prog {
    Prog(vec![Eq, Num(1)])
}
#[test]
fn test_exec_20_misc_46() {
    assert_eq!(prog_20_misc_46().exec(false), test_exec_20_misc_46_res())
}
fn test_exec_20_misc_46_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_04() {
    let src = fs::read_to_string("./assets/14_rev_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_04_res())
}
fn test_parse_14_rev_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_04())
}
fn prog_14_rev_cmd_04() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_04() {
    assert_eq!(
        prog_14_rev_cmd_04().exec(false),
        test_exec_14_rev_cmd_04_res()
    )
}
fn test_exec_14_rev_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_014() {
    let src = fs::read_to_string("./assets/06_rem_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_014_res())
}
fn test_parse_06_rem_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_014())
}
fn prog_06_rem_cmd_014() -> Prog {
    Prog(vec![Rem, Num(-2147483648), Num(-5)])
}
#[test]
fn test_exec_06_rem_cmd_014() {
    assert_eq!(
        prog_06_rem_cmd_014().exec(false),
        test_exec_06_rem_cmd_014_res()
    )
}
fn test_exec_06_rem_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_20_misc_89() {
    let src = fs::read_to_string("./assets/20_misc_89.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_89_res())
}
fn test_parse_20_misc_89_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_89())
}
fn prog_20_misc_89() -> Prog {
    Prog(vec![Ifz, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_89() {
    assert_eq!(prog_20_misc_89().exec(false), test_exec_20_misc_89_res())
}
fn test_exec_20_misc_89_res() -> Result<i64, ProgExecError> {
    Ok(-99)
}

#[test]
fn test_parse_06_rem_cmd_018() {
    let src = fs::read_to_string("./assets/06_rem_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_018_res())
}
fn test_parse_06_rem_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_018())
}
fn prog_06_rem_cmd_018() -> Prog {
    Prog(vec![Rem, Num(0), Num(-2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_018() {
    assert_eq!(
        prog_06_rem_cmd_018().exec(false),
        test_exec_06_rem_cmd_018_res()
    )
}
fn test_exec_06_rem_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_016() {
    let src = fs::read_to_string("./assets/05_div_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_016_res())
}
fn test_parse_05_div_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_016())
}
fn prog_05_div_cmd_016() -> Prog {
    Prog(vec![Div, Num(-3), Num(-1)])
}
#[test]
fn test_exec_05_div_cmd_016() {
    assert_eq!(
        prog_05_div_cmd_016().exec(false),
        test_exec_05_div_cmd_016_res()
    )
}
fn test_exec_05_div_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_20_misc_16() {
    let src = fs::read_to_string("./assets/20_misc_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_16_res())
}
fn test_parse_20_misc_16_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_16())
}
fn prog_20_misc_16() -> Prog {
    Prog(vec![Sub, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_16() {
    assert_eq!(prog_20_misc_16().exec(false), test_exec_20_misc_16_res())
}
fn test_exec_20_misc_16_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_05_div_cmd_012() {
    let src = fs::read_to_string("./assets/05_div_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_012_res())
}
fn test_parse_05_div_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_012())
}
fn prog_05_div_cmd_012() -> Prog {
    Prog(vec![Div, Num(-2147483647), Num(2147483647)])
}
#[test]
fn test_exec_05_div_cmd_012() {
    assert_eq!(
        prog_05_div_cmd_012().exec(false),
        test_exec_05_div_cmd_012_res()
    )
}
fn test_exec_05_div_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_08_lt_cmd_017() {
    let src = fs::read_to_string("./assets/08_lt_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_017_res())
}
fn test_parse_08_lt_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_017())
}
fn prog_08_lt_cmd_017() -> Prog {
    Prog(vec![Lt, Num(-2147483648), Num(5)])
}
#[test]
fn test_exec_08_lt_cmd_017() {
    assert_eq!(
        prog_08_lt_cmd_017().exec(false),
        test_exec_08_lt_cmd_017_res()
    )
}
fn test_exec_08_lt_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_66() {
    let src = fs::read_to_string("./assets/20_misc_66.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_66_res())
}
fn test_parse_20_misc_66_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_66())
}
fn prog_20_misc_66() -> Prog {
    Prog(vec![Gt, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_66() {
    assert_eq!(prog_20_misc_66().exec(false), test_exec_20_misc_66_res())
}
fn test_exec_20_misc_66_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_04_mul_cmd_003() {
    let src = fs::read_to_string("./assets/04_mul_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_003_res())
}
fn test_parse_04_mul_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_003())
}
fn prog_04_mul_cmd_003() -> Prog {
    Prog(vec![Mul, Num(-3), Num(2147483646)])
}
#[test]
fn test_exec_04_mul_cmd_003() {
    assert_eq!(
        prog_04_mul_cmd_003().exec(false),
        test_exec_04_mul_cmd_003_res()
    )
}
fn test_exec_04_mul_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(-6442450938)
}

#[test]
fn test_parse_08_lt_cmd_000() {
    let src = fs::read_to_string("./assets/08_lt_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_000_res())
}
fn test_parse_08_lt_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_000())
}
fn prog_08_lt_cmd_000() -> Prog {
    Prog(vec![Lt, Num(2147483646), Num(3)])
}
#[test]
fn test_exec_08_lt_cmd_000() {
    assert_eq!(
        prog_08_lt_cmd_000().exec(false),
        test_exec_08_lt_cmd_000_res()
    )
}
fn test_exec_08_lt_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_03_sub_cmd_013() {
    let src = fs::read_to_string("./assets/03_sub_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_013_res())
}
fn test_parse_03_sub_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_013())
}
fn prog_03_sub_cmd_013() -> Prog {
    Prog(vec![Sub, Num(2147483646), Num(-1)])
}
#[test]
fn test_exec_03_sub_cmd_013() {
    assert_eq!(
        prog_03_sub_cmd_013().exec(false),
        test_exec_03_sub_cmd_013_res()
    )
}
fn test_exec_03_sub_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_14_rev_cmd_17() {
    let src = fs::read_to_string("./assets/14_rev_cmd_17.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_17_res())
}
fn test_parse_14_rev_cmd_17_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_17())
}
fn prog_14_rev_cmd_17() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_17() {
    assert_eq!(
        prog_14_rev_cmd_17().exec(false),
        test_exec_14_rev_cmd_17_res()
    )
}
fn test_exec_14_rev_cmd_17_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_03() {
    let src = fs::read_to_string("./assets/00_example_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_03_res())
}
fn test_parse_00_example_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_03())
}
fn prog_00_example_03() -> Prog {
    Prog(vec![Exec, Swap, Num(2), Cmds(vec![Mul, Num(3)])])
}
#[test]
fn test_exec_00_example_03() {
    assert_eq!(
        prog_00_example_03().exec(false),
        test_exec_00_example_03_res()
    )
}
fn test_exec_00_example_03_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_12_pop_cmd_01() {
    let src = fs::read_to_string("./assets/12_pop_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_01_res())
}
fn test_parse_12_pop_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_01())
}
fn prog_12_pop_cmd_01() -> Prog {
    Prog(vec![Pop, Pop, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_12_pop_cmd_01() {
    assert_eq!(
        prog_12_pop_cmd_01().exec(false),
        test_exec_12_pop_cmd_01_res()
    )
}
fn test_exec_12_pop_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_07_eq_cmd_018() {
    let src = fs::read_to_string("./assets/07_eq_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_018_res())
}
fn test_parse_07_eq_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_018())
}
fn prog_07_eq_cmd_018() -> Prog {
    Prog(vec![Eq, Num(-3), Num(3)])
}
#[test]
fn test_exec_07_eq_cmd_018() {
    assert_eq!(
        prog_07_eq_cmd_018().exec(false),
        test_exec_07_eq_cmd_018_res()
    )
}
fn test_exec_07_eq_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_03_sub_cmd_002() {
    let src = fs::read_to_string("./assets/03_sub_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_002_res())
}
fn test_parse_03_sub_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_002())
}
fn prog_03_sub_cmd_002() -> Prog {
    Prog(vec![Sub, Num(-2147483647), Num(-2147483647)])
}
#[test]
fn test_exec_03_sub_cmd_002() {
    assert_eq!(
        prog_03_sub_cmd_002().exec(false),
        test_exec_03_sub_cmd_002_res()
    )
}
fn test_exec_03_sub_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_00_example_10() {
    let src = fs::read_to_string("./assets/00_example_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_10_res())
}
fn test_parse_00_example_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_10())
}
fn prog_00_example_10() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(-7),
    ])
}
#[test]
fn test_exec_00_example_10() {
    assert_eq!(
        prog_00_example_10().exec(false),
        test_exec_00_example_10_res()
    )
}
fn test_exec_00_example_10_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_72_tailfib() {
    let src = fs::read_to_string("./assets/72_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_72_tailfib_res())
}
fn test_parse_72_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_72_tailfib())
}
fn prog_72_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(92),
    ])
}
#[test]
fn test_exec_72_tailfib() {
    assert_eq!(prog_72_tailfib().exec(false), test_exec_72_tailfib_res())
}
fn test_exec_72_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(7540113804746346429)
}

#[test]
fn test_parse_20_misc_48() {
    let src = fs::read_to_string("./assets/20_misc_48.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_48_res())
}
fn test_parse_20_misc_48_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_48())
}
fn prog_20_misc_48() -> Prog {
    Prog(vec![Eq, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_48() {
    assert_eq!(prog_20_misc_48().exec(false), test_exec_20_misc_48_res())
}
fn test_exec_20_misc_48_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_11_dup_cmd_02() {
    let src = fs::read_to_string("./assets/11_dup_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_02_res())
}
fn test_parse_11_dup_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_02())
}
fn prog_11_dup_cmd_02() -> Prog {
    Prog(vec![Dup, Num(-3), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_02() {
    assert_eq!(
        prog_11_dup_cmd_02().exec(false),
        test_exec_11_dup_cmd_02_res()
    )
}
fn test_exec_11_dup_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_20_misc_42() {
    let src = fs::read_to_string("./assets/20_misc_42.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_42_res())
}
fn test_parse_20_misc_42_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_42())
}
fn prog_20_misc_42() -> Prog {
    Prog(vec![Rem, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_42() {
    assert_eq!(prog_20_misc_42().exec(false), test_exec_20_misc_42_res())
}
fn test_exec_20_misc_42_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_019() {
    let src = fs::read_to_string("./assets/04_mul_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_019_res())
}
fn test_parse_04_mul_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_019())
}
fn prog_04_mul_cmd_019() -> Prog {
    Prog(vec![Mul, Num(5)])
}
#[test]
fn test_exec_04_mul_cmd_019() {
    assert_eq!(
        prog_04_mul_cmd_019().exec(false),
        test_exec_04_mul_cmd_019_res()
    )
}
fn test_exec_04_mul_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_03() {
    let src = fs::read_to_string("./assets/20_misc_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_03_res())
}
fn test_parse_20_misc_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_03())
}
fn prog_20_misc_03() -> Prog {
    Prog(vec![Add, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_03() {
    assert_eq!(prog_20_misc_03().exec(false), test_exec_20_misc_03_res())
}
fn test_exec_20_misc_03_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_05_div_cmd_018() {
    let src = fs::read_to_string("./assets/05_div_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_018_res())
}
fn test_parse_05_div_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_018())
}
fn prog_05_div_cmd_018() -> Prog {
    Prog(vec![Div, Num(-2), Num(10)])
}
#[test]
fn test_exec_05_div_cmd_018() {
    assert_eq!(
        prog_05_div_cmd_018().exec(false),
        test_exec_05_div_cmd_018_res()
    )
}
fn test_exec_05_div_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_02_add_cmd_009() {
    let src = fs::read_to_string("./assets/02_add_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_009_res())
}
fn test_parse_02_add_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_009())
}
fn prog_02_add_cmd_009() -> Prog {
    Prog(vec![Add, Num(1), Num(-5)])
}
#[test]
fn test_exec_02_add_cmd_009() {
    assert_eq!(
        prog_02_add_cmd_009().exec(false),
        test_exec_02_add_cmd_009_res()
    )
}
fn test_exec_02_add_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_06_rem_cmd_015() {
    let src = fs::read_to_string("./assets/06_rem_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_015_res())
}
fn test_parse_06_rem_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_015())
}
fn prog_06_rem_cmd_015() -> Prog {
    Prog(vec![Rem, Num(-2147483647), Num(5)])
}
#[test]
fn test_exec_06_rem_cmd_015() {
    assert_eq!(
        prog_06_rem_cmd_015().exec(false),
        test_exec_06_rem_cmd_015_res()
    )
}
fn test_exec_06_rem_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_16_exec_cmd_18() {
    let src = fs::read_to_string("./assets/16_exec_cmd_18.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_18_res())
}
fn test_parse_16_exec_cmd_18_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_18())
}
fn prog_16_exec_cmd_18() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_18() {
    assert_eq!(
        prog_16_exec_cmd_18().exec(false),
        test_exec_16_exec_cmd_18_res()
    )
}
fn test_exec_16_exec_cmd_18_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_20_misc_82() {
    let src = fs::read_to_string("./assets/20_misc_82.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_82_res())
}
fn test_parse_20_misc_82_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_82())
}
fn prog_20_misc_82() -> Prog {
    Prog(vec![Ifz, Num(1)])
}
#[test]
fn test_exec_20_misc_82() {
    assert_eq!(prog_20_misc_82().exec(false), test_exec_20_misc_82_res())
}
fn test_exec_20_misc_82_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_01() {
    let src = fs::read_to_string("./assets/13_swap_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_01_res())
}
fn test_parse_13_swap_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_01())
}
fn prog_13_swap_cmd_01() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_01() {
    assert_eq!(
        prog_13_swap_cmd_01().exec(false),
        test_exec_13_swap_cmd_01_res()
    )
}
fn test_exec_13_swap_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_15_cmds_cmd_10() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_10_res())
}
fn test_parse_15_cmds_cmd_10_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_20_misc_26() {
    let src = fs::read_to_string("./assets/20_misc_26.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_26_res())
}
fn test_parse_20_misc_26_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_26())
}
fn prog_20_misc_26() -> Prog {
    Prog(vec![Mul, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_26() {
    assert_eq!(prog_20_misc_26().exec(false), test_exec_20_misc_26_res())
}
fn test_exec_20_misc_26_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_03() {
    let src = fs::read_to_string("./assets/13_swap_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_03_res())
}
fn test_parse_13_swap_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_03())
}
fn prog_13_swap_cmd_03() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_03() {
    assert_eq!(
        prog_13_swap_cmd_03().exec(false),
        test_exec_13_swap_cmd_03_res()
    )
}
fn test_exec_13_swap_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_11_dup_cmd_08() {
    let src = fs::read_to_string("./assets/11_dup_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_08_res())
}
fn test_parse_11_dup_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_08())
}
fn prog_11_dup_cmd_08() -> Prog {
    Prog(vec![Dup, Num(3), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_08() {
    assert_eq!(
        prog_11_dup_cmd_08().exec(false),
        test_exec_11_dup_cmd_08_res()
    )
}
fn test_exec_11_dup_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_00_example_02() {
    let src = fs::read_to_string("./assets/00_example_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_02_res())
}
fn test_parse_00_example_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_02())
}
fn prog_00_example_02() -> Prog {
    Prog(vec![Exec, Cmds(vec![Mul, Num(3)]), Num(2)])
}
#[test]
fn test_exec_00_example_02() {
    assert_eq!(
        prog_00_example_02().exec(false),
        test_exec_00_example_02_res()
    )
}
fn test_exec_00_example_02_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_12_pop_cmd_06() {
    let src = fs::read_to_string("./assets/12_pop_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_06_res())
}
fn test_parse_12_pop_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_06())
}
fn prog_12_pop_cmd_06() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_06() {
    assert_eq!(
        prog_12_pop_cmd_06().exec(false),
        test_exec_12_pop_cmd_06_res()
    )
}
fn test_exec_12_pop_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_20_misc_63() {
    let src = fs::read_to_string("./assets/20_misc_63.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_63_res())
}
fn test_parse_20_misc_63_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_63())
}
fn prog_20_misc_63() -> Prog {
    Prog(vec![Gt])
}
#[test]
fn test_exec_20_misc_63() {
    assert_eq!(prog_20_misc_63().exec(false), test_exec_20_misc_63_res())
}
fn test_exec_20_misc_63_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_007() {
    let src = fs::read_to_string("./assets/09_gt_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_007_res())
}
fn test_parse_09_gt_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_007())
}
fn prog_09_gt_cmd_007() -> Prog {
    Prog(vec![Gt, Num(-2147483646), Num(-2147483646)])
}
#[test]
fn test_exec_09_gt_cmd_007() {
    assert_eq!(
        prog_09_gt_cmd_007().exec(false),
        test_exec_09_gt_cmd_007_res()
    )
}
fn test_exec_09_gt_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_11_dup_cmd_01() {
    let src = fs::read_to_string("./assets/11_dup_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_01_res())
}
fn test_parse_11_dup_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_01())
}
fn prog_11_dup_cmd_01() -> Prog {
    Prog(vec![Dup, Num(-4), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_01() {
    assert_eq!(
        prog_11_dup_cmd_01().exec(false),
        test_exec_11_dup_cmd_01_res()
    )
}
fn test_exec_11_dup_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_11_dup_cmd_09() {
    let src = fs::read_to_string("./assets/11_dup_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_09_res())
}
fn test_parse_11_dup_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_09())
}
fn prog_11_dup_cmd_09() -> Prog {
    Prog(vec![Dup, Num(4), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_09() {
    assert_eq!(
        prog_11_dup_cmd_09().exec(false),
        test_exec_11_dup_cmd_09_res()
    )
}
fn test_exec_11_dup_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_010() {
    let src = fs::read_to_string("./assets/07_eq_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_010_res())
}
fn test_parse_07_eq_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_010())
}
fn prog_07_eq_cmd_010() -> Prog {
    Prog(vec![Eq, Num(2147483647), Num(5)])
}
#[test]
fn test_exec_07_eq_cmd_010() {
    assert_eq!(
        prog_07_eq_cmd_010().exec(false),
        test_exec_07_eq_cmd_010_res()
    )
}
fn test_exec_07_eq_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_94() {
    let src = fs::read_to_string("./assets/20_misc_94.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_94_res())
}
fn test_parse_20_misc_94_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_94())
}
fn prog_20_misc_94() -> Prog {
    Prog(vec![Dup, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_94() {
    assert_eq!(prog_20_misc_94().exec(false), test_exec_20_misc_94_res())
}
fn test_exec_20_misc_94_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_01_num_cmd_00() {
    let src = fs::read_to_string("./assets/01_num_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_00_res())
}
fn test_parse_01_num_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_00())
}
fn prog_01_num_cmd_00() -> Prog {
    Prog(vec![Num(42)])
}
#[test]
fn test_exec_01_num_cmd_00() {
    assert_eq!(
        prog_01_num_cmd_00().exec(false),
        test_exec_01_num_cmd_00_res()
    )
}
fn test_exec_01_num_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(42)
}

#[test]
fn test_parse_02_add_cmd_019() {
    let src = fs::read_to_string("./assets/02_add_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_019_res())
}
fn test_parse_02_add_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_019())
}
fn prog_02_add_cmd_019() -> Prog {
    Prog(vec![Add, Num(0)])
}
#[test]
fn test_exec_02_add_cmd_019() {
    assert_eq!(
        prog_02_add_cmd_019().exec(false),
        test_exec_02_add_cmd_019_res()
    )
}
fn test_exec_02_add_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_004() {
    let src = fs::read_to_string("./assets/04_mul_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_004_res())
}
fn test_parse_04_mul_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_004())
}
fn prog_04_mul_cmd_004() -> Prog {
    Prog(vec![Mul, Num(-1), Num(10)])
}
#[test]
fn test_exec_04_mul_cmd_004() {
    assert_eq!(
        prog_04_mul_cmd_004().exec(false),
        test_exec_04_mul_cmd_004_res()
    )
}
fn test_exec_04_mul_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-10)
}

#[test]
fn test_parse_09_gt_cmd_003() {
    let src = fs::read_to_string("./assets/09_gt_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_003_res())
}
fn test_parse_09_gt_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_003())
}
fn prog_09_gt_cmd_003() -> Prog {
    Prog(vec![Gt, Num(-2147483648), Num(1)])
}
#[test]
fn test_exec_09_gt_cmd_003() {
    assert_eq!(
        prog_09_gt_cmd_003().exec(false),
        test_exec_09_gt_cmd_003_res()
    )
}
fn test_exec_09_gt_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_14_rev_cmd_16() {
    let src = fs::read_to_string("./assets/14_rev_cmd_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_16_res())
}
fn test_parse_14_rev_cmd_16_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_16())
}
fn prog_14_rev_cmd_16() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_16() {
    assert_eq!(
        prog_14_rev_cmd_16().exec(false),
        test_exec_14_rev_cmd_16_res()
    )
}
fn test_exec_14_rev_cmd_16_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_017() {
    let src = fs::read_to_string("./assets/06_rem_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_017_res())
}
fn test_parse_06_rem_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_017())
}
fn prog_06_rem_cmd_017() -> Prog {
    Prog(vec![Rem, Num(-2147483646), Num(-3)])
}
#[test]
fn test_exec_06_rem_cmd_017() {
    assert_eq!(
        prog_06_rem_cmd_017().exec(false),
        test_exec_06_rem_cmd_017_res()
    )
}
fn test_exec_06_rem_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_23() {
    let src = fs::read_to_string("./assets/20_misc_23.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_23_res())
}
fn test_parse_20_misc_23_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_23())
}
fn prog_20_misc_23() -> Prog {
    Prog(vec![Mul, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_23() {
    assert_eq!(prog_20_misc_23().exec(false), test_exec_20_misc_23_res())
}
fn test_exec_20_misc_23_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_019() {
    let src = fs::read_to_string("./assets/05_div_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_019_res())
}
fn test_parse_05_div_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_019())
}
fn prog_05_div_cmd_019() -> Prog {
    Prog(vec![Div, Num(10)])
}
#[test]
fn test_exec_05_div_cmd_019() {
    assert_eq!(
        prog_05_div_cmd_019().exec(false),
        test_exec_05_div_cmd_019_res()
    )
}
fn test_exec_05_div_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_001() {
    let src = fs::read_to_string("./assets/04_mul_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_001_res())
}
fn test_parse_04_mul_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_001())
}
fn prog_04_mul_cmd_001() -> Prog {
    Prog(vec![Mul, Num(-10), Num(-2)])
}
#[test]
fn test_exec_04_mul_cmd_001() {
    assert_eq!(
        prog_04_mul_cmd_001().exec(false),
        test_exec_04_mul_cmd_001_res()
    )
}
fn test_exec_04_mul_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(20)
}

#[test]
fn test_parse_09_gt_cmd_011() {
    let src = fs::read_to_string("./assets/09_gt_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_011_res())
}
fn test_parse_09_gt_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_011())
}
fn prog_09_gt_cmd_011() -> Prog {
    Prog(vec![Gt, Num(2147483648), Num(-2)])
}
#[test]
fn test_exec_09_gt_cmd_011() {
    assert_eq!(
        prog_09_gt_cmd_011().exec(false),
        test_exec_09_gt_cmd_011_res()
    )
}
fn test_exec_09_gt_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_003() {
    let src = fs::read_to_string("./assets/06_rem_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_003_res())
}
fn test_parse_06_rem_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_003())
}
fn prog_06_rem_cmd_003() -> Prog {
    Prog(vec![Rem, Num(1), Num(0)])
}
#[test]
fn test_exec_06_rem_cmd_003() {
    assert_eq!(
        prog_06_rem_cmd_003().exec(false),
        test_exec_06_rem_cmd_003_res()
    )
}
fn test_exec_06_rem_cmd_003_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_07() {
    let src = fs::read_to_string("./assets/12_pop_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_07_res())
}
fn test_parse_12_pop_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_07())
}
fn prog_12_pop_cmd_07() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_07() {
    assert_eq!(
        prog_12_pop_cmd_07().exec(false),
        test_exec_12_pop_cmd_07_res()
    )
}
fn test_exec_12_pop_cmd_07_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_06() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_06_res())
}
fn test_parse_10_ifz_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_06())
}
fn prog_10_ifz_cmd_06() -> Prog {
    Prog(vec![Ifz, Num(1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_06() {
    assert_eq!(
        prog_10_ifz_cmd_06().exec(false),
        test_exec_10_ifz_cmd_06_res()
    )
}
fn test_exec_10_ifz_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_20_misc_53() {
    let src = fs::read_to_string("./assets/20_misc_53.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_53_res())
}
fn test_parse_20_misc_53_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_53())
}
fn prog_20_misc_53() -> Prog {
    Prog(vec![Eq, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_53() {
    assert_eq!(prog_20_misc_53().exec(false), test_exec_20_misc_53_res())
}
fn test_exec_20_misc_53_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_09() {
    let src = fs::read_to_string("./assets/00_example_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_09_res())
}
fn test_parse_00_example_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_09())
}
fn prog_00_example_09() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_00_example_09() {
    assert_eq!(
        prog_00_example_09().exec(false),
        test_exec_00_example_09_res()
    )
}
fn test_exec_00_example_09_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_08_lt_cmd_009() {
    let src = fs::read_to_string("./assets/08_lt_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_009_res())
}
fn test_parse_08_lt_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_009())
}
fn prog_08_lt_cmd_009() -> Prog {
    Prog(vec![Lt, Num(-1), Num(-2147483646)])
}
#[test]
fn test_exec_08_lt_cmd_009() {
    assert_eq!(
        prog_08_lt_cmd_009().exec(false),
        test_exec_08_lt_cmd_009_res()
    )
}
fn test_exec_08_lt_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_07() {
    let src = fs::read_to_string("./assets/20_misc_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_07_res())
}
fn test_parse_20_misc_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_07())
}
fn prog_20_misc_07() -> Prog {
    Prog(vec![Add, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_07() {
    assert_eq!(prog_20_misc_07().exec(false), test_exec_20_misc_07_res())
}
fn test_exec_20_misc_07_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_11_dup_cmd_03() {
    let src = fs::read_to_string("./assets/11_dup_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_03_res())
}
fn test_parse_11_dup_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_03())
}
fn prog_11_dup_cmd_03() -> Prog {
    Prog(vec![Dup, Num(-2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_03() {
    assert_eq!(
        prog_11_dup_cmd_03().exec(false),
        test_exec_11_dup_cmd_03_res()
    )
}
fn test_exec_11_dup_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_20_misc_73() {
    let src = fs::read_to_string("./assets/20_misc_73.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_73_res())
}
fn test_parse_20_misc_73_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_73())
}
fn prog_20_misc_73() -> Prog {
    Prog(vec![Exec, Num(1)])
}
#[test]
fn test_exec_20_misc_73() {
    assert_eq!(prog_20_misc_73().exec(false), test_exec_20_misc_73_res())
}
fn test_exec_20_misc_73_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_003() {
    let src = fs::read_to_string("./assets/03_sub_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_003_res())
}
fn test_parse_03_sub_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_003())
}
fn prog_03_sub_cmd_003() -> Prog {
    Prog(vec![Sub, Num(-2147483646), Num(-10)])
}
#[test]
fn test_exec_03_sub_cmd_003() {
    assert_eq!(
        prog_03_sub_cmd_003().exec(false),
        test_exec_03_sub_cmd_003_res()
    )
}
fn test_exec_03_sub_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(-2147483636)
}

#[test]
fn test_parse_10_ifz_cmd_02() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_02_res())
}
fn test_parse_10_ifz_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_02())
}
fn prog_10_ifz_cmd_02() -> Prog {
    Prog(vec![Ifz, Num(-1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_02() {
    assert_eq!(
        prog_10_ifz_cmd_02().exec(false),
        test_exec_10_ifz_cmd_02_res()
    )
}
fn test_exec_10_ifz_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_00_example_12() {
    let src = fs::read_to_string("./assets/00_example_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_12_res())
}
fn test_parse_00_example_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_12())
}
fn prog_00_example_12() -> Prog {
    Prog(vec![Exec, Swap, Num(3), Cmds(vec![Mul, Num(4)])])
}
#[test]
fn test_exec_00_example_12() {
    assert_eq!(
        prog_00_example_12().exec(false),
        test_exec_00_example_12_res()
    )
}
fn test_exec_00_example_12_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_11_dup_cmd_04() {
    let src = fs::read_to_string("./assets/11_dup_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_04_res())
}
fn test_parse_11_dup_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_04())
}
fn prog_11_dup_cmd_04() -> Prog {
    Prog(vec![Dup, Num(-1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_04() {
    assert_eq!(
        prog_11_dup_cmd_04().exec(false),
        test_exec_11_dup_cmd_04_res()
    )
}
fn test_exec_11_dup_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_008() {
    let src = fs::read_to_string("./assets/06_rem_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_008_res())
}
fn test_parse_06_rem_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_008())
}
fn prog_06_rem_cmd_008() -> Prog {
    Prog(vec![Rem, Num(2), Num(3)])
}
#[test]
fn test_exec_06_rem_cmd_008() {
    assert_eq!(
        prog_06_rem_cmd_008().exec(false),
        test_exec_06_rem_cmd_008_res()
    )
}
fn test_exec_06_rem_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_20_misc_95() {
    let src = fs::read_to_string("./assets/20_misc_95.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_95_res())
}
fn test_parse_20_misc_95_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_95())
}
fn prog_20_misc_95() -> Prog {
    Prog(vec![Dup, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_95() {
    assert_eq!(prog_20_misc_95().exec(false), test_exec_20_misc_95_res())
}
fn test_exec_20_misc_95_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_13() {
    let src = fs::read_to_string("./assets/00_example_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_13_res())
}
fn test_parse_00_example_13_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_16_exec_cmd_11() {
    let src = fs::read_to_string("./assets/16_exec_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_11_res())
}
fn test_parse_16_exec_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_11())
}
fn prog_16_exec_cmd_11() -> Prog {
    Prog(vec![Exec, Cmds(vec![Exec, Cmds(vec![])]), Num(99)])
}
#[test]
fn test_exec_16_exec_cmd_11() {
    assert_eq!(
        prog_16_exec_cmd_11().exec(false),
        test_exec_16_exec_cmd_11_res()
    )
}
fn test_exec_16_exec_cmd_11_res() -> Result<i64, ProgExecError> {
    Ok(99)
}

#[test]
fn test_parse_16_exec_cmd_05() {
    let src = fs::read_to_string("./assets/16_exec_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_05_res())
}
fn test_parse_16_exec_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_05())
}
fn prog_16_exec_cmd_05() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![Mul]),
        Swap,
        Cmds(vec![Add]),
        Lt,
        Swap,
        Num(3),
        Num(4),
        Num(5),
        Num(6),
    ])
}
#[test]
fn test_exec_16_exec_cmd_05() {
    assert_eq!(
        prog_16_exec_cmd_05().exec(false),
        test_exec_16_exec_cmd_05_res()
    )
}
fn test_exec_16_exec_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(30)
}

#[test]
fn test_parse_08_lt_cmd_002() {
    let src = fs::read_to_string("./assets/08_lt_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_002_res())
}
fn test_parse_08_lt_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_002())
}
fn prog_08_lt_cmd_002() -> Prog {
    Prog(vec![Lt, Num(10), Num(0)])
}
#[test]
fn test_exec_08_lt_cmd_002() {
    assert_eq!(
        prog_08_lt_cmd_002().exec(false),
        test_exec_08_lt_cmd_002_res()
    )
}
fn test_exec_08_lt_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_13_swap_cmd_00() {
    let src = fs::read_to_string("./assets/13_swap_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_00_res())
}
fn test_parse_13_swap_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_00())
}
fn prog_13_swap_cmd_00() -> Prog {
    Prog(vec![
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_00() {
    assert_eq!(
        prog_13_swap_cmd_00().exec(false),
        test_exec_13_swap_cmd_00_res()
    )
}
fn test_exec_13_swap_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(-8)
}

#[test]
fn test_parse_20_misc_71() {
    let src = fs::read_to_string("./assets/20_misc_71.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_71_res())
}
fn test_parse_20_misc_71_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_71())
}
fn prog_20_misc_71() -> Prog {
    Prog(vec![Gt, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_71() {
    assert_eq!(prog_20_misc_71().exec(false), test_exec_20_misc_71_res())
}
fn test_exec_20_misc_71_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_79() {
    let src = fs::read_to_string("./assets/20_misc_79.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_79_res())
}
fn test_parse_20_misc_79_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_79())
}
fn prog_20_misc_79() -> Prog {
    Prog(vec![Exec, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_79() {
    assert_eq!(prog_20_misc_79().exec(false), test_exec_20_misc_79_res())
}
fn test_exec_20_misc_79_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_52() {
    let src = fs::read_to_string("./assets/20_misc_52.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_52_res())
}
fn test_parse_20_misc_52_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_52())
}
fn prog_20_misc_52() -> Prog {
    Prog(vec![Eq, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_52() {
    assert_eq!(prog_20_misc_52().exec(false), test_exec_20_misc_52_res())
}
fn test_exec_20_misc_52_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_12_pop_cmd_02() {
    let src = fs::read_to_string("./assets/12_pop_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_02_res())
}
fn test_parse_12_pop_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_02())
}
fn prog_12_pop_cmd_02() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_02() {
    assert_eq!(
        prog_12_pop_cmd_02().exec(false),
        test_exec_12_pop_cmd_02_res()
    )
}
fn test_exec_12_pop_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_06_rem_cmd_020() {
    let src = fs::read_to_string("./assets/06_rem_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_020_res())
}
fn test_parse_06_rem_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_020())
}
fn prog_06_rem_cmd_020() -> Prog {
    Prog(vec![Rem])
}
#[test]
fn test_exec_06_rem_cmd_020() {
    assert_eq!(
        prog_06_rem_cmd_020().exec(false),
        test_exec_06_rem_cmd_020_res()
    )
}
fn test_exec_06_rem_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_01() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_01_res())
}
fn test_parse_10_ifz_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_01())
}
fn prog_10_ifz_cmd_01() -> Prog {
    Prog(vec![Ifz, Num(-2)])
}
#[test]
fn test_exec_10_ifz_cmd_01() {
    assert_eq!(
        prog_10_ifz_cmd_01().exec(false),
        test_exec_10_ifz_cmd_01_res()
    )
}
fn test_exec_10_ifz_cmd_01_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_17() {
    let src = fs::read_to_string("./assets/16_exec_cmd_17.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_17_res())
}
fn test_parse_16_exec_cmd_17_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_17())
}
fn prog_16_exec_cmd_17() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(1),
    ])
}
#[test]
fn test_exec_16_exec_cmd_17() {
    assert_eq!(
        prog_16_exec_cmd_17().exec(false),
        test_exec_16_exec_cmd_17_res()
    )
}
fn test_exec_16_exec_cmd_17_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_20_misc_57() {
    let src = fs::read_to_string("./assets/20_misc_57.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_57_res())
}
fn test_parse_20_misc_57_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_57())
}
fn prog_20_misc_57() -> Prog {
    Prog(vec![Lt, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_57() {
    assert_eq!(prog_20_misc_57().exec(false), test_exec_20_misc_57_res())
}
fn test_exec_20_misc_57_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_07_eq_cmd_020() {
    let src = fs::read_to_string("./assets/07_eq_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_020_res())
}
fn test_parse_07_eq_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_020())
}
fn prog_07_eq_cmd_020() -> Prog {
    Prog(vec![Eq])
}
#[test]
fn test_exec_07_eq_cmd_020() {
    assert_eq!(
        prog_07_eq_cmd_020().exec(false),
        test_exec_07_eq_cmd_020_res()
    )
}
fn test_exec_07_eq_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_13() {
    let src = fs::read_to_string("./assets/14_rev_cmd_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_13_res())
}
fn test_parse_14_rev_cmd_13_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_13())
}
fn prog_14_rev_cmd_13() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_13() {
    assert_eq!(
        prog_14_rev_cmd_13().exec(false),
        test_exec_14_rev_cmd_13_res()
    )
}
fn test_exec_14_rev_cmd_13_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_14_rev_cmd_14() {
    let src = fs::read_to_string("./assets/14_rev_cmd_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_14_res())
}
fn test_parse_14_rev_cmd_14_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_14())
}
fn prog_14_rev_cmd_14() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_14() {
    assert_eq!(
        prog_14_rev_cmd_14().exec(false),
        test_exec_14_rev_cmd_14_res()
    )
}
fn test_exec_14_rev_cmd_14_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_15_cmds_cmd_05() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_05_res())
}
fn test_parse_15_cmds_cmd_05_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_16_exec_cmd_08() {
    let src = fs::read_to_string("./assets/16_exec_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_08_res())
}
fn test_parse_16_exec_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_08())
}
fn prog_16_exec_cmd_08() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(7),
    ])
}
#[test]
fn test_exec_16_exec_cmd_08() {
    assert_eq!(
        prog_16_exec_cmd_08().exec(false),
        test_exec_16_exec_cmd_08_res()
    )
}
fn test_exec_16_exec_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_03_sub_cmd_020() {
    let src = fs::read_to_string("./assets/03_sub_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_020_res())
}
fn test_parse_03_sub_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_020())
}
fn prog_03_sub_cmd_020() -> Prog {
    Prog(vec![Sub])
}
#[test]
fn test_exec_03_sub_cmd_020() {
    assert_eq!(
        prog_03_sub_cmd_020().exec(false),
        test_exec_03_sub_cmd_020_res()
    )
}
fn test_exec_03_sub_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_001() {
    let src = fs::read_to_string("./assets/02_add_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_001_res())
}
fn test_parse_02_add_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_001())
}
fn prog_02_add_cmd_001() -> Prog {
    Prog(vec![Add, Num(-1), Num(2147483647)])
}
#[test]
fn test_exec_02_add_cmd_001() {
    assert_eq!(
        prog_02_add_cmd_001().exec(false),
        test_exec_02_add_cmd_001_res()
    )
}
fn test_exec_02_add_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(2147483646)
}

#[test]
fn test_parse_20_misc_87() {
    let src = fs::read_to_string("./assets/20_misc_87.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_87_res())
}
fn test_parse_20_misc_87_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_87())
}
fn prog_20_misc_87() -> Prog {
    Prog(vec![Ifz, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_87() {
    assert_eq!(prog_20_misc_87().exec(false), test_exec_20_misc_87_res())
}
fn test_exec_20_misc_87_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_06() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_06_res())
}
fn test_parse_15_cmds_cmd_06_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_20_misc_91() {
    let src = fs::read_to_string("./assets/20_misc_91.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_91_res())
}
fn test_parse_20_misc_91_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_91())
}
fn prog_20_misc_91() -> Prog {
    Prog(vec![Dup, Num(1)])
}
#[test]
fn test_exec_20_misc_91() {
    assert_eq!(prog_20_misc_91().exec(false), test_exec_20_misc_91_res())
}
fn test_exec_20_misc_91_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_009() {
    let src = fs::read_to_string("./assets/06_rem_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_009_res())
}
fn test_parse_06_rem_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_009())
}
fn prog_06_rem_cmd_009() -> Prog {
    Prog(vec![Rem, Num(-1), Num(1)])
}
#[test]
fn test_exec_06_rem_cmd_009() {
    assert_eq!(
        prog_06_rem_cmd_009().exec(false),
        test_exec_06_rem_cmd_009_res()
    )
}
fn test_exec_06_rem_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_64() {
    let src = fs::read_to_string("./assets/20_misc_64.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_64_res())
}
fn test_parse_20_misc_64_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_64())
}
fn prog_20_misc_64() -> Prog {
    Prog(vec![Gt, Num(1)])
}
#[test]
fn test_exec_20_misc_64() {
    assert_eq!(prog_20_misc_64().exec(false), test_exec_20_misc_64_res())
}
fn test_exec_20_misc_64_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_58() {
    let src = fs::read_to_string("./assets/20_misc_58.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_58_res())
}
fn test_parse_20_misc_58_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_58())
}
fn prog_20_misc_58() -> Prog {
    Prog(vec![Lt, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_58() {
    assert_eq!(prog_20_misc_58().exec(false), test_exec_20_misc_58_res())
}
fn test_exec_20_misc_58_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_08() {
    let src = fs::read_to_string("./assets/12_pop_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_08_res())
}
fn test_parse_12_pop_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_08())
}
fn prog_12_pop_cmd_08() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_08() {
    assert_eq!(
        prog_12_pop_cmd_08().exec(false),
        test_exec_12_pop_cmd_08_res()
    )
}
fn test_exec_12_pop_cmd_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_10() {
    let src = fs::read_to_string("./assets/11_dup_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_10_res())
}
fn test_parse_11_dup_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_10())
}
fn prog_11_dup_cmd_10() -> Prog {
    Prog(vec![Dup, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_10() {
    assert_eq!(
        prog_11_dup_cmd_10().exec(false),
        test_exec_11_dup_cmd_10_res()
    )
}
fn test_exec_11_dup_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_84() {
    let src = fs::read_to_string("./assets/20_misc_84.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_84_res())
}
fn test_parse_20_misc_84_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_84())
}
fn prog_20_misc_84() -> Prog {
    Prog(vec![Ifz, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_84() {
    assert_eq!(prog_20_misc_84().exec(false), test_exec_20_misc_84_res())
}
fn test_exec_20_misc_84_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_01() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_01_res())
}
fn test_parse_15_cmds_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_01())
}
fn prog_15_cmds_cmd_01() -> Prog {
    Prog(vec![Num(3), Num(2), Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_01() {
    assert_eq!(
        prog_15_cmds_cmd_01().exec(false),
        test_exec_15_cmds_cmd_01_res()
    )
}
fn test_exec_15_cmds_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_05_div_cmd_004() {
    let src = fs::read_to_string("./assets/05_div_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_004_res())
}
fn test_parse_05_div_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_004())
}
fn prog_05_div_cmd_004() -> Prog {
    Prog(vec![Div, Num(10), Num(-2147483648)])
}
#[test]
fn test_exec_05_div_cmd_004() {
    assert_eq!(
        prog_05_div_cmd_004().exec(false),
        test_exec_05_div_cmd_004_res()
    )
}
fn test_exec_05_div_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_51_fact() {
    let src = fs::read_to_string("./assets/51_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_51_fact_res())
}
fn test_parse_51_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_51_fact())
}
fn prog_51_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(11),
    ])
}
#[test]
fn test_exec_51_fact() {
    assert_eq!(prog_51_fact().exec(false), test_exec_51_fact_res())
}
fn test_exec_51_fact_res() -> Result<i64, ProgExecError> {
    Ok(39916800)
}

#[test]
fn test_parse_20_misc_54() {
    let src = fs::read_to_string("./assets/20_misc_54.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_54_res())
}
fn test_parse_20_misc_54_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_54())
}
fn prog_20_misc_54() -> Prog {
    Prog(vec![Lt])
}
#[test]
fn test_exec_20_misc_54() {
    assert_eq!(prog_20_misc_54().exec(false), test_exec_20_misc_54_res())
}
fn test_exec_20_misc_54_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_007() {
    let src = fs::read_to_string("./assets/02_add_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_007_res())
}
fn test_parse_02_add_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_007())
}
fn prog_02_add_cmd_007() -> Prog {
    Prog(vec![Add, Num(-3), Num(2147483646)])
}
#[test]
fn test_exec_02_add_cmd_007() {
    assert_eq!(
        prog_02_add_cmd_007().exec(false),
        test_exec_02_add_cmd_007_res()
    )
}
fn test_exec_02_add_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(2147483643)
}

#[test]
fn test_parse_07_eq_cmd_001() {
    let src = fs::read_to_string("./assets/07_eq_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_001_res())
}
fn test_parse_07_eq_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_001())
}
fn prog_07_eq_cmd_001() -> Prog {
    Prog(vec![Eq, Num(5), Num(-10)])
}
#[test]
fn test_exec_07_eq_cmd_001() {
    assert_eq!(
        prog_07_eq_cmd_001().exec(false),
        test_exec_07_eq_cmd_001_res()
    )
}
fn test_exec_07_eq_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_11_dup_cmd_07() {
    let src = fs::read_to_string("./assets/11_dup_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_07_res())
}
fn test_parse_11_dup_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_07())
}
fn prog_11_dup_cmd_07() -> Prog {
    Prog(vec![Dup, Num(2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_07() {
    assert_eq!(
        prog_11_dup_cmd_07().exec(false),
        test_exec_11_dup_cmd_07_res()
    )
}
fn test_exec_11_dup_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_20_misc_44() {
    let src = fs::read_to_string("./assets/20_misc_44.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_44_res())
}
fn test_parse_20_misc_44_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_44())
}
fn prog_20_misc_44() -> Prog {
    Prog(vec![Rem, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_44() {
    assert_eq!(prog_20_misc_44().exec(false), test_exec_20_misc_44_res())
}
fn test_exec_20_misc_44_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_32() {
    let src = fs::read_to_string("./assets/20_misc_32.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_32_res())
}
fn test_parse_20_misc_32_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_32())
}
fn prog_20_misc_32() -> Prog {
    Prog(vec![Div, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_32() {
    assert_eq!(prog_20_misc_32().exec(false), test_exec_20_misc_32_res())
}
fn test_exec_20_misc_32_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_015() {
    let src = fs::read_to_string("./assets/09_gt_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_015_res())
}
fn test_parse_09_gt_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_015())
}
fn prog_09_gt_cmd_015() -> Prog {
    Prog(vec![Gt, Num(2), Num(-5)])
}
#[test]
fn test_exec_09_gt_cmd_015() {
    assert_eq!(
        prog_09_gt_cmd_015().exec(false),
        test_exec_09_gt_cmd_015_res()
    )
}
fn test_exec_09_gt_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_07_eq_cmd_000() {
    let src = fs::read_to_string("./assets/07_eq_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_000_res())
}
fn test_parse_07_eq_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_000())
}
fn prog_07_eq_cmd_000() -> Prog {
    Prog(vec![Eq, Num(2), Num(-2147483648)])
}
#[test]
fn test_exec_07_eq_cmd_000() {
    assert_eq!(
        prog_07_eq_cmd_000().exec(false),
        test_exec_07_eq_cmd_000_res()
    )
}
fn test_exec_07_eq_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_002() {
    let src = fs::read_to_string("./assets/09_gt_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_002_res())
}
fn test_parse_09_gt_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_002())
}
fn prog_09_gt_cmd_002() -> Prog {
    Prog(vec![Gt, Num(-2147483647), Num(-10)])
}
#[test]
fn test_exec_09_gt_cmd_002() {
    assert_eq!(
        prog_09_gt_cmd_002().exec(false),
        test_exec_09_gt_cmd_002_res()
    )
}
fn test_exec_09_gt_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_03_sub_cmd_015() {
    let src = fs::read_to_string("./assets/03_sub_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_015_res())
}
fn test_parse_03_sub_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_015())
}
fn prog_03_sub_cmd_015() -> Prog {
    Prog(vec![Sub, Num(-2147483648), Num(-2)])
}
#[test]
fn test_exec_03_sub_cmd_015() {
    assert_eq!(
        prog_03_sub_cmd_015().exec(false),
        test_exec_03_sub_cmd_015_res()
    )
}
fn test_exec_03_sub_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2147483646)
}

#[test]
fn test_parse_08_lt_cmd_006() {
    let src = fs::read_to_string("./assets/08_lt_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_006_res())
}
fn test_parse_08_lt_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_006())
}
fn prog_08_lt_cmd_006() -> Prog {
    Prog(vec![Lt, Num(0), Num(-2147483648)])
}
#[test]
fn test_exec_08_lt_cmd_006() {
    assert_eq!(
        prog_08_lt_cmd_006().exec(false),
        test_exec_08_lt_cmd_006_res()
    )
}
fn test_exec_08_lt_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_08_lt_cmd_014() {
    let src = fs::read_to_string("./assets/08_lt_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_014_res())
}
fn test_parse_08_lt_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_014())
}
fn prog_08_lt_cmd_014() -> Prog {
    Prog(vec![Lt, Num(-2147483646), Num(2147483646)])
}
#[test]
fn test_exec_08_lt_cmd_014() {
    assert_eq!(
        prog_08_lt_cmd_014().exec(false),
        test_exec_08_lt_cmd_014_res()
    )
}
fn test_exec_08_lt_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_04_mul_cmd_009() {
    let src = fs::read_to_string("./assets/04_mul_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_009_res())
}
fn test_parse_04_mul_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_009())
}
fn prog_04_mul_cmd_009() -> Prog {
    Prog(vec![Mul, Num(10), Num(0)])
}
#[test]
fn test_exec_04_mul_cmd_009() {
    assert_eq!(
        prog_04_mul_cmd_009().exec(false),
        test_exec_04_mul_cmd_009_res()
    )
}
fn test_exec_04_mul_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_014() {
    let src = fs::read_to_string("./assets/07_eq_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_014_res())
}
fn test_parse_07_eq_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_014())
}
fn prog_07_eq_cmd_014() -> Prog {
    Prog(vec![Eq, Num(-1), Num(-1)])
}
#[test]
fn test_exec_07_eq_cmd_014() {
    assert_eq!(
        prog_07_eq_cmd_014().exec(false),
        test_exec_07_eq_cmd_014_res()
    )
}
fn test_exec_07_eq_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_24() {
    let src = fs::read_to_string("./assets/20_misc_24.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_24_res())
}
fn test_parse_20_misc_24_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_24())
}
fn prog_20_misc_24() -> Prog {
    Prog(vec![Mul, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_24() {
    assert_eq!(prog_20_misc_24().exec(false), test_exec_20_misc_24_res())
}
fn test_exec_20_misc_24_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_008() {
    let src = fs::read_to_string("./assets/07_eq_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_008_res())
}
fn test_parse_07_eq_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_008())
}
fn prog_07_eq_cmd_008() -> Prog {
    Prog(vec![Eq, Num(-2), Num(10)])
}
#[test]
fn test_exec_07_eq_cmd_008() {
    assert_eq!(
        prog_07_eq_cmd_008().exec(false),
        test_exec_07_eq_cmd_008_res()
    )
}
fn test_exec_07_eq_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_04_mul_cmd_020() {
    let src = fs::read_to_string("./assets/04_mul_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_020_res())
}
fn test_parse_04_mul_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_020())
}
fn prog_04_mul_cmd_020() -> Prog {
    Prog(vec![Mul])
}
#[test]
fn test_exec_04_mul_cmd_020() {
    assert_eq!(
        prog_04_mul_cmd_020().exec(false),
        test_exec_04_mul_cmd_020_res()
    )
}
fn test_exec_04_mul_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_34() {
    let src = fs::read_to_string("./assets/20_misc_34.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_34_res())
}
fn test_parse_20_misc_34_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_34())
}
fn prog_20_misc_34() -> Prog {
    Prog(vec![Div, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_34() {
    assert_eq!(prog_20_misc_34().exec(false), test_exec_20_misc_34_res())
}
fn test_exec_20_misc_34_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_30() {
    let src = fs::read_to_string("./assets/20_misc_30.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_30_res())
}
fn test_parse_20_misc_30_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_30())
}
fn prog_20_misc_30() -> Prog {
    Prog(vec![Div, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_30() {
    assert_eq!(prog_20_misc_30().exec(false), test_exec_20_misc_30_res())
}
fn test_exec_20_misc_30_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_09_gt_cmd_017() {
    let src = fs::read_to_string("./assets/09_gt_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_017_res())
}
fn test_parse_09_gt_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_017())
}
fn prog_09_gt_cmd_017() -> Prog {
    Prog(vec![Gt, Num(5), Num(-1)])
}
#[test]
fn test_exec_09_gt_cmd_017() {
    assert_eq!(
        prog_09_gt_cmd_017().exec(false),
        test_exec_09_gt_cmd_017_res()
    )
}
fn test_exec_09_gt_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_06_rem_cmd_001() {
    let src = fs::read_to_string("./assets/06_rem_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_001_res())
}
fn test_parse_06_rem_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_001())
}
fn prog_06_rem_cmd_001() -> Prog {
    Prog(vec![Rem, Num(-3), Num(-2147483647)])
}
#[test]
fn test_exec_06_rem_cmd_001() {
    assert_eq!(
        prog_06_rem_cmd_001().exec(false),
        test_exec_06_rem_cmd_001_res()
    )
}
fn test_exec_06_rem_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_12_pop_cmd_00() {
    let src = fs::read_to_string("./assets/12_pop_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_00_res())
}
fn test_parse_12_pop_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_00())
}
fn prog_12_pop_cmd_00() -> Prog {
    Prog(vec![Pop, Num(5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_12_pop_cmd_00() {
    assert_eq!(
        prog_12_pop_cmd_00().exec(false),
        test_exec_12_pop_cmd_00_res()
    )
}
fn test_exec_12_pop_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_10_ifz_cmd_09() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_09_res())
}
fn test_parse_10_ifz_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_09())
}
fn prog_10_ifz_cmd_09() -> Prog {
    Prog(vec![Ifz, Num(2)])
}
#[test]
fn test_exec_10_ifz_cmd_09() {
    assert_eq!(
        prog_10_ifz_cmd_09().exec(false),
        test_exec_10_ifz_cmd_09_res()
    )
}
fn test_exec_10_ifz_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_003() {
    let src = fs::read_to_string("./assets/08_lt_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_003_res())
}
fn test_parse_08_lt_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_003())
}
fn prog_08_lt_cmd_003() -> Prog {
    Prog(vec![Lt, Num(2), Num(-3)])
}
#[test]
fn test_exec_08_lt_cmd_003() {
    assert_eq!(
        prog_08_lt_cmd_003().exec(false),
        test_exec_08_lt_cmd_003_res()
    )
}
fn test_exec_08_lt_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_09() {
    let src = fs::read_to_string("./assets/20_misc_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_09_res())
}
fn test_parse_20_misc_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_09())
}
fn prog_20_misc_09() -> Prog {
    Prog(vec![Sub])
}
#[test]
fn test_exec_20_misc_09() {
    assert_eq!(prog_20_misc_09().exec(false), test_exec_20_misc_09_res())
}
fn test_exec_20_misc_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_71_tailfib() {
    let src = fs::read_to_string("./assets/71_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_71_tailfib_res())
}
fn test_parse_71_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_71_tailfib())
}
fn prog_71_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(50),
    ])
}
#[test]
fn test_exec_71_tailfib() {
    assert_eq!(prog_71_tailfib().exec(false), test_exec_71_tailfib_res())
}
fn test_exec_71_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(12586269025)
}

#[test]
fn test_parse_07_eq_cmd_016() {
    let src = fs::read_to_string("./assets/07_eq_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_016_res())
}
fn test_parse_07_eq_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_016())
}
fn prog_07_eq_cmd_016() -> Prog {
    Prog(vec![Eq, Num(-10), Num(2)])
}
#[test]
fn test_exec_07_eq_cmd_016() {
    assert_eq!(
        prog_07_eq_cmd_016().exec(false),
        test_exec_07_eq_cmd_016_res()
    )
}
fn test_exec_07_eq_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_06_rem_cmd_006() {
    let src = fs::read_to_string("./assets/06_rem_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_006_res())
}
fn test_parse_06_rem_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_006())
}
fn prog_06_rem_cmd_006() -> Prog {
    Prog(vec![Rem, Num(2147483647), Num(2147483646)])
}
#[test]
fn test_exec_06_rem_cmd_006() {
    assert_eq!(
        prog_06_rem_cmd_006().exec(false),
        test_exec_06_rem_cmd_006_res()
    )
}
fn test_exec_06_rem_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_00_example_16() {
    let src = fs::read_to_string("./assets/00_example_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_16_res())
}
fn test_parse_00_example_16_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_03_sub_cmd_010() {
    let src = fs::read_to_string("./assets/03_sub_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_010_res())
}
fn test_parse_03_sub_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_010())
}
fn prog_03_sub_cmd_010() -> Prog {
    Prog(vec![Sub, Num(-1), Num(2)])
}
#[test]
fn test_exec_03_sub_cmd_010() {
    assert_eq!(
        prog_03_sub_cmd_010().exec(false),
        test_exec_03_sub_cmd_010_res()
    )
}
fn test_exec_03_sub_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_20_misc_13() {
    let src = fs::read_to_string("./assets/20_misc_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_13_res())
}
fn test_parse_20_misc_13_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_13())
}
fn prog_20_misc_13() -> Prog {
    Prog(vec![Sub, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_13() {
    assert_eq!(prog_20_misc_13().exec(false), test_exec_20_misc_13_res())
}
fn test_exec_20_misc_13_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_012() {
    let src = fs::read_to_string("./assets/02_add_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_012_res())
}
fn test_parse_02_add_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_012())
}
fn prog_02_add_cmd_012() -> Prog {
    Prog(vec![Add, Num(2147483646), Num(-1)])
}
#[test]
fn test_exec_02_add_cmd_012() {
    assert_eq!(
        prog_02_add_cmd_012().exec(false),
        test_exec_02_add_cmd_012_res()
    )
}
fn test_exec_02_add_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(2147483645)
}

#[test]
fn test_parse_08_lt_cmd_001() {
    let src = fs::read_to_string("./assets/08_lt_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_001_res())
}
fn test_parse_08_lt_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_001())
}
fn prog_08_lt_cmd_001() -> Prog {
    Prog(vec![Lt, Num(3), Num(-10)])
}
#[test]
fn test_exec_08_lt_cmd_001() {
    assert_eq!(
        prog_08_lt_cmd_001().exec(false),
        test_exec_08_lt_cmd_001_res()
    )
}
fn test_exec_08_lt_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_10_ifz_cmd_10() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_10_res())
}
fn test_parse_10_ifz_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_10())
}
fn prog_10_ifz_cmd_10() -> Prog {
    Prog(vec![Ifz])
}
#[test]
fn test_exec_10_ifz_cmd_10() {
    assert_eq!(
        prog_10_ifz_cmd_10().exec(false),
        test_exec_10_ifz_cmd_10_res()
    )
}
fn test_exec_10_ifz_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_016() {
    let src = fs::read_to_string("./assets/02_add_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_016_res())
}
fn test_parse_02_add_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_016())
}
fn prog_02_add_cmd_016() -> Prog {
    Prog(vec![Add, Num(2), Num(10)])
}
#[test]
fn test_exec_02_add_cmd_016() {
    assert_eq!(
        prog_02_add_cmd_016().exec(false),
        test_exec_02_add_cmd_016_res()
    )
}
fn test_exec_02_add_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(12)
}

#[test]
fn test_parse_02_add_cmd_020() {
    let src = fs::read_to_string("./assets/02_add_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_020_res())
}
fn test_parse_02_add_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_020())
}
fn prog_02_add_cmd_020() -> Prog {
    Prog(vec![Add])
}
#[test]
fn test_exec_02_add_cmd_020() {
    assert_eq!(
        prog_02_add_cmd_020().exec(false),
        test_exec_02_add_cmd_020_res()
    )
}
fn test_exec_02_add_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_02() {
    let src = fs::read_to_string("./assets/20_misc_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_02_res())
}
fn test_parse_20_misc_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_02())
}
fn prog_20_misc_02() -> Prog {
    Prog(vec![Add, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_02() {
    assert_eq!(prog_20_misc_02().exec(false), test_exec_20_misc_02_res())
}
fn test_exec_20_misc_02_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_14() {
    let src = fs::read_to_string("./assets/00_example_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_14_res())
}
fn test_parse_00_example_14_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_16_exec_cmd_22() {
    let src = fs::read_to_string("./assets/16_exec_cmd_22.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_22_res())
}
fn test_parse_16_exec_cmd_22_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_22())
}
fn prog_16_exec_cmd_22() -> Prog {
    Prog(vec![Exec, Cmds(vec![Exec, Cmds(vec![])])])
}
#[test]
fn test_exec_16_exec_cmd_22() {
    assert_eq!(
        prog_16_exec_cmd_22().exec(false),
        test_exec_16_exec_cmd_22_res()
    )
}
fn test_exec_16_exec_cmd_22_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_001() {
    let src = fs::read_to_string("./assets/09_gt_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_001_res())
}
fn test_parse_09_gt_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_001())
}
fn prog_09_gt_cmd_001() -> Prog {
    Prog(vec![Gt, Num(-5), Num(10)])
}
#[test]
fn test_exec_09_gt_cmd_001() {
    assert_eq!(
        prog_09_gt_cmd_001().exec(false),
        test_exec_09_gt_cmd_001_res()
    )
}
fn test_exec_09_gt_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_01_num_cmd_10() {
    let src = fs::read_to_string("./assets/01_num_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_10_res())
}
fn test_parse_01_num_cmd_10_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_14_rev_cmd_12() {
    let src = fs::read_to_string("./assets/14_rev_cmd_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_12_res())
}
fn test_parse_14_rev_cmd_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_12())
}
fn prog_14_rev_cmd_12() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_12() {
    assert_eq!(
        prog_14_rev_cmd_12().exec(false),
        test_exec_14_rev_cmd_12_res()
    )
}
fn test_exec_14_rev_cmd_12_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_05_div_cmd_006() {
    let src = fs::read_to_string("./assets/05_div_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_006_res())
}
fn test_parse_05_div_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_006())
}
fn prog_05_div_cmd_006() -> Prog {
    Prog(vec![Div, Num(-2147483648), Num(3)])
}
#[test]
fn test_exec_05_div_cmd_006() {
    assert_eq!(
        prog_05_div_cmd_006().exec(false),
        test_exec_05_div_cmd_006_res()
    )
}
fn test_exec_05_div_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(-715827882)
}

#[test]
fn test_parse_02_add_cmd_015() {
    let src = fs::read_to_string("./assets/02_add_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_015_res())
}
fn test_parse_02_add_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_015())
}
fn prog_02_add_cmd_015() -> Prog {
    Prog(vec![Add, Num(3), Num(-2147483647)])
}
#[test]
fn test_exec_02_add_cmd_015() {
    assert_eq!(
        prog_02_add_cmd_015().exec(false),
        test_exec_02_add_cmd_015_res()
    )
}
fn test_exec_02_add_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(-2147483644)
}

#[test]
fn test_parse_10_ifz_cmd_08() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_08_res())
}
fn test_parse_10_ifz_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_08())
}
fn prog_10_ifz_cmd_08() -> Prog {
    Prog(vec![Ifz, Num(2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_08() {
    assert_eq!(
        prog_10_ifz_cmd_08().exec(false),
        test_exec_10_ifz_cmd_08_res()
    )
}
fn test_exec_10_ifz_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_01_num_cmd_05() {
    let src = fs::read_to_string("./assets/01_num_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_05_res())
}
fn test_parse_01_num_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_05())
}
fn prog_01_num_cmd_05() -> Prog {
    Prog(vec![Num(2147483648)])
}
#[test]
fn test_exec_01_num_cmd_05() {
    assert_eq!(
        prog_01_num_cmd_05().exec(false),
        test_exec_01_num_cmd_05_res()
    )
}
fn test_exec_01_num_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_20_misc_74() {
    let src = fs::read_to_string("./assets/20_misc_74.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_74_res())
}
fn test_parse_20_misc_74_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_74())
}
fn prog_20_misc_74() -> Prog {
    Prog(vec![Exec, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_74() {
    assert_eq!(prog_20_misc_74().exec(false), test_exec_20_misc_74_res())
}
fn test_exec_20_misc_74_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_00_example_15() {
    let src = fs::read_to_string("./assets/00_example_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_15_res())
}
fn test_parse_00_example_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_15())
}
fn prog_00_example_15() -> Prog {
    Prog(vec![Add, Num(2)])
}
#[test]
fn test_exec_00_example_15() {
    assert_eq!(
        prog_00_example_15().exec(false),
        test_exec_00_example_15_res()
    )
}
fn test_exec_00_example_15_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_38() {
    let src = fs::read_to_string("./assets/20_misc_38.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_38_res())
}
fn test_parse_20_misc_38_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_38())
}
fn prog_20_misc_38() -> Prog {
    Prog(vec![Rem, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_38() {
    assert_eq!(prog_20_misc_38().exec(false), test_exec_20_misc_38_res())
}
fn test_exec_20_misc_38_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_01_num_cmd_03() {
    let src = fs::read_to_string("./assets/01_num_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_03_res())
}
fn test_parse_01_num_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_03())
}
fn prog_01_num_cmd_03() -> Prog {
    Prog(vec![Num(-1), Num(-2), Num(-3), Num(-4), Num(-5), Num(-42)])
}
#[test]
fn test_exec_01_num_cmd_03() {
    assert_eq!(
        prog_01_num_cmd_03().exec(false),
        test_exec_01_num_cmd_03_res()
    )
}
fn test_exec_01_num_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_15_cmds_cmd_11() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_11_res())
}
fn test_parse_15_cmds_cmd_11_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_20_misc_61() {
    let src = fs::read_to_string("./assets/20_misc_61.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_61_res())
}
fn test_parse_20_misc_61_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_61())
}
fn prog_20_misc_61() -> Prog {
    Prog(vec![Lt, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_61() {
    assert_eq!(prog_20_misc_61().exec(false), test_exec_20_misc_61_res())
}
fn test_exec_20_misc_61_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_72() {
    let src = fs::read_to_string("./assets/20_misc_72.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_72_res())
}
fn test_parse_20_misc_72_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_72())
}
fn prog_20_misc_72() -> Prog {
    Prog(vec![Exec])
}
#[test]
fn test_exec_20_misc_72() {
    assert_eq!(prog_20_misc_72().exec(false), test_exec_20_misc_72_res())
}
fn test_exec_20_misc_72_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_01() {
    let src = fs::read_to_string("./assets/14_rev_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_01_res())
}
fn test_parse_14_rev_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_01())
}
fn prog_14_rev_cmd_01() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_01() {
    assert_eq!(
        prog_14_rev_cmd_01().exec(false),
        test_exec_14_rev_cmd_01_res()
    )
}
fn test_exec_14_rev_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_09_gt_cmd_009() {
    let src = fs::read_to_string("./assets/09_gt_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_009_res())
}
fn test_parse_09_gt_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_009())
}
fn prog_09_gt_cmd_009() -> Prog {
    Prog(vec![Gt, Num(2147483646), Num(2147483646)])
}
#[test]
fn test_exec_09_gt_cmd_009() {
    assert_eq!(
        prog_09_gt_cmd_009().exec(false),
        test_exec_09_gt_cmd_009_res()
    )
}
fn test_exec_09_gt_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_14_rev_cmd_18() {
    let src = fs::read_to_string("./assets/14_rev_cmd_18.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_18_res())
}
fn test_parse_14_rev_cmd_18_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_18())
}
fn prog_14_rev_cmd_18() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_18() {
    assert_eq!(
        prog_14_rev_cmd_18().exec(false),
        test_exec_14_rev_cmd_18_res()
    )
}
fn test_exec_14_rev_cmd_18_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_05() {
    let src = fs::read_to_string("./assets/12_pop_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_05_res())
}
fn test_parse_12_pop_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_05())
}
fn prog_12_pop_cmd_05() -> Prog {
    Prog(vec![
        Pop,
        Num(-8),
        Num(7),
        Num(-6),
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_05() {
    assert_eq!(
        prog_12_pop_cmd_05().exec(false),
        test_exec_12_pop_cmd_05_res()
    )
}
fn test_exec_12_pop_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_10_ifz_cmd_03() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_03_res())
}
fn test_parse_10_ifz_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_03())
}
fn prog_10_ifz_cmd_03() -> Prog {
    Prog(vec![Ifz, Num(-1)])
}
#[test]
fn test_exec_10_ifz_cmd_03() {
    assert_eq!(
        prog_10_ifz_cmd_03().exec(false),
        test_exec_10_ifz_cmd_03_res()
    )
}
fn test_exec_10_ifz_cmd_03_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_80() {
    let src = fs::read_to_string("./assets/20_misc_80.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_80_res())
}
fn test_parse_20_misc_80_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_80())
}
fn prog_20_misc_80() -> Prog {
    Prog(vec![Exec, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_80() {
    assert_eq!(prog_20_misc_80().exec(false), test_exec_20_misc_80_res())
}
fn test_exec_20_misc_80_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_75() {
    let src = fs::read_to_string("./assets/20_misc_75.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_75_res())
}
fn test_parse_20_misc_75_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_75())
}
fn prog_20_misc_75() -> Prog {
    Prog(vec![Exec, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_75() {
    assert_eq!(prog_20_misc_75().exec(false), test_exec_20_misc_75_res())
}
fn test_exec_20_misc_75_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_012() {
    let src = fs::read_to_string("./assets/08_lt_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_012_res())
}
fn test_parse_08_lt_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_012())
}
fn prog_08_lt_cmd_012() -> Prog {
    Prog(vec![Lt, Num(2147483648), Num(-2147483647)])
}
#[test]
fn test_exec_08_lt_cmd_012() {
    assert_eq!(
        prog_08_lt_cmd_012().exec(false),
        test_exec_08_lt_cmd_012_res()
    )
}
fn test_exec_08_lt_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_007() {
    let src = fs::read_to_string("./assets/07_eq_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_007_res())
}
fn test_parse_07_eq_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_007())
}
fn prog_07_eq_cmd_007() -> Prog {
    Prog(vec![Eq, Num(-2147483647), Num(-2147483647)])
}
#[test]
fn test_exec_07_eq_cmd_007() {
    assert_eq!(
        prog_07_eq_cmd_007().exec(false),
        test_exec_07_eq_cmd_007_res()
    )
}
fn test_exec_07_eq_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_07() {
    let src = fs::read_to_string("./assets/14_rev_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_07_res())
}
fn test_parse_14_rev_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_07())
}
fn prog_14_rev_cmd_07() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_07() {
    assert_eq!(
        prog_14_rev_cmd_07().exec(false),
        test_exec_14_rev_cmd_07_res()
    )
}
fn test_exec_14_rev_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_03_sub_cmd_004() {
    let src = fs::read_to_string("./assets/03_sub_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_004_res())
}
fn test_parse_03_sub_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_004())
}
fn prog_03_sub_cmd_004() -> Prog {
    Prog(vec![Sub, Num(-2), Num(-3)])
}
#[test]
fn test_exec_03_sub_cmd_004() {
    assert_eq!(
        prog_03_sub_cmd_004().exec(false),
        test_exec_03_sub_cmd_004_res()
    )
}
fn test_exec_03_sub_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_10_ifz_cmd_00() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_00_res())
}
fn test_parse_10_ifz_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_00())
}
fn prog_10_ifz_cmd_00() -> Prog {
    Prog(vec![Ifz, Num(-2), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_00() {
    assert_eq!(
        prog_10_ifz_cmd_00().exec(false),
        test_exec_10_ifz_cmd_00_res()
    )
}
fn test_exec_10_ifz_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_20_misc_05() {
    let src = fs::read_to_string("./assets/20_misc_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_05_res())
}
fn test_parse_20_misc_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_05())
}
fn prog_20_misc_05() -> Prog {
    Prog(vec![Add, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_05() {
    assert_eq!(prog_20_misc_05().exec(false), test_exec_20_misc_05_res())
}
fn test_exec_20_misc_05_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_013() {
    let src = fs::read_to_string("./assets/04_mul_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_013_res())
}
fn test_parse_04_mul_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_013())
}
fn prog_04_mul_cmd_013() -> Prog {
    Prog(vec![Mul, Num(5), Num(-5)])
}
#[test]
fn test_exec_04_mul_cmd_013() {
    assert_eq!(
        prog_04_mul_cmd_013().exec(false),
        test_exec_04_mul_cmd_013_res()
    )
}
fn test_exec_04_mul_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(-25)
}

#[test]
fn test_parse_20_misc_96() {
    let src = fs::read_to_string("./assets/20_misc_96.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_96_res())
}
fn test_parse_20_misc_96_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_96())
}
fn prog_20_misc_96() -> Prog {
    Prog(vec![Dup, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_96() {
    assert_eq!(prog_20_misc_96().exec(false), test_exec_20_misc_96_res())
}
fn test_exec_20_misc_96_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_014() {
    let src = fs::read_to_string("./assets/05_div_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_014_res())
}
fn test_parse_05_div_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_014())
}
fn prog_05_div_cmd_014() -> Prog {
    Prog(vec![Div, Num(0), Num(5)])
}
#[test]
fn test_exec_05_div_cmd_014() {
    assert_eq!(
        prog_05_div_cmd_014().exec(false),
        test_exec_05_div_cmd_014_res()
    )
}
fn test_exec_05_div_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_29() {
    let src = fs::read_to_string("./assets/20_misc_29.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_29_res())
}
fn test_parse_20_misc_29_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_29())
}
fn prog_20_misc_29() -> Prog {
    Prog(vec![Div, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_29() {
    assert_eq!(prog_20_misc_29().exec(false), test_exec_20_misc_29_res())
}
fn test_exec_20_misc_29_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_76() {
    let src = fs::read_to_string("./assets/20_misc_76.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_76_res())
}
fn test_parse_20_misc_76_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_76())
}
fn prog_20_misc_76() -> Prog {
    Prog(vec![Exec, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_76() {
    assert_eq!(prog_20_misc_76().exec(false), test_exec_20_misc_76_res())
}
fn test_exec_20_misc_76_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_02() {
    let src = fs::read_to_string("./assets/16_exec_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_02_res())
}
fn test_parse_16_exec_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_02())
}
fn prog_16_exec_cmd_02() -> Prog {
    Prog(vec![Exec, Swap, Num(7), Cmds(vec![Sub, Num(0)])])
}
#[test]
fn test_exec_16_exec_cmd_02() {
    assert_eq!(
        prog_16_exec_cmd_02().exec(false),
        test_exec_16_exec_cmd_02_res()
    )
}
fn test_exec_16_exec_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-7)
}

#[test]
fn test_parse_14_rev_cmd_00() {
    let src = fs::read_to_string("./assets/14_rev_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_00_res())
}
fn test_parse_14_rev_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_00())
}
fn prog_14_rev_cmd_00() -> Prog {
    Prog(vec![
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_00() {
    assert_eq!(
        prog_14_rev_cmd_00().exec(false),
        test_exec_14_rev_cmd_00_res()
    )
}
fn test_exec_14_rev_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_008() {
    let src = fs::read_to_string("./assets/09_gt_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_008_res())
}
fn test_parse_09_gt_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_008())
}
fn prog_09_gt_cmd_008() -> Prog {
    Prog(vec![Gt, Num(0), Num(0)])
}
#[test]
fn test_exec_09_gt_cmd_008() {
    assert_eq!(
        prog_09_gt_cmd_008().exec(false),
        test_exec_09_gt_cmd_008_res()
    )
}
fn test_exec_09_gt_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_81_tak() {
    let src = fs::read_to_string("./assets/81_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_81_tak_res())
}
fn test_parse_81_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_81_tak())
}
fn prog_81_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(12),
        Num(8),
        Num(4),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_81_tak() {
    assert_eq!(prog_81_tak().exec(false), test_exec_81_tak_res())
}
fn test_exec_81_tak_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_20_misc_28() {
    let src = fs::read_to_string("./assets/20_misc_28.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_28_res())
}
fn test_parse_20_misc_28_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_28())
}
fn prog_20_misc_28() -> Prog {
    Prog(vec![Div, Num(1)])
}
#[test]
fn test_exec_20_misc_28() {
    assert_eq!(prog_20_misc_28().exec(false), test_exec_20_misc_28_res())
}
fn test_exec_20_misc_28_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_007() {
    let src = fs::read_to_string("./assets/08_lt_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_007_res())
}
fn test_parse_08_lt_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_007())
}
fn prog_08_lt_cmd_007() -> Prog {
    Prog(vec![Lt, Num(-2147483647), Num(2)])
}
#[test]
fn test_exec_08_lt_cmd_007() {
    assert_eq!(
        prog_08_lt_cmd_007().exec(false),
        test_exec_08_lt_cmd_007_res()
    )
}
fn test_exec_08_lt_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_03() {
    let src = fs::read_to_string("./assets/14_rev_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_03_res())
}
fn test_parse_14_rev_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_03())
}
fn prog_14_rev_cmd_03() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_03() {
    assert_eq!(
        prog_14_rev_cmd_03().exec(false),
        test_exec_14_rev_cmd_03_res()
    )
}
fn test_exec_14_rev_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-8)
}

#[test]
fn test_parse_14_rev_cmd_10() {
    let src = fs::read_to_string("./assets/14_rev_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_10_res())
}
fn test_parse_14_rev_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_10())
}
fn prog_14_rev_cmd_10() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_10() {
    assert_eq!(
        prog_14_rev_cmd_10().exec(false),
        test_exec_14_rev_cmd_10_res()
    )
}
fn test_exec_14_rev_cmd_10_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_20_misc_55() {
    let src = fs::read_to_string("./assets/20_misc_55.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_55_res())
}
fn test_parse_20_misc_55_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_55())
}
fn prog_20_misc_55() -> Prog {
    Prog(vec![Lt, Num(1)])
}
#[test]
fn test_exec_20_misc_55() {
    assert_eq!(prog_20_misc_55().exec(false), test_exec_20_misc_55_res())
}
fn test_exec_20_misc_55_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_50() {
    let src = fs::read_to_string("./assets/20_misc_50.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_50_res())
}
fn test_parse_20_misc_50_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_50())
}
fn prog_20_misc_50() -> Prog {
    Prog(vec![Eq, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_50() {
    assert_eq!(prog_20_misc_50().exec(false), test_exec_20_misc_50_res())
}
fn test_exec_20_misc_50_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_60() {
    let src = fs::read_to_string("./assets/20_misc_60.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_60_res())
}
fn test_parse_20_misc_60_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_60())
}
fn prog_20_misc_60() -> Prog {
    Prog(vec![Lt, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_60() {
    assert_eq!(prog_20_misc_60().exec(false), test_exec_20_misc_60_res())
}
fn test_exec_20_misc_60_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_49() {
    let src = fs::read_to_string("./assets/20_misc_49.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_49_res())
}
fn test_parse_20_misc_49_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_49())
}
fn prog_20_misc_49() -> Prog {
    Prog(vec![Eq, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_49() {
    assert_eq!(prog_20_misc_49().exec(false), test_exec_20_misc_49_res())
}
fn test_exec_20_misc_49_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_10_ifz_cmd_04() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_04_res())
}
fn test_parse_10_ifz_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_04())
}
fn prog_10_ifz_cmd_04() -> Prog {
    Prog(vec![Ifz, Num(0), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_10_ifz_cmd_04() {
    assert_eq!(
        prog_10_ifz_cmd_04().exec(false),
        test_exec_10_ifz_cmd_04_res()
    )
}
fn test_exec_10_ifz_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_20_misc_56() {
    let src = fs::read_to_string("./assets/20_misc_56.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_56_res())
}
fn test_parse_20_misc_56_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_56())
}
fn prog_20_misc_56() -> Prog {
    Prog(vec![Lt, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_56() {
    assert_eq!(prog_20_misc_56().exec(false), test_exec_20_misc_56_res())
}
fn test_exec_20_misc_56_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_45() {
    let src = fs::read_to_string("./assets/20_misc_45.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_45_res())
}
fn test_parse_20_misc_45_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_45())
}
fn prog_20_misc_45() -> Prog {
    Prog(vec![Eq])
}
#[test]
fn test_exec_20_misc_45() {
    assert_eq!(prog_20_misc_45().exec(false), test_exec_20_misc_45_res())
}
fn test_exec_20_misc_45_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_013() {
    let src = fs::read_to_string("./assets/05_div_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_013_res())
}
fn test_parse_05_div_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_013())
}
fn prog_05_div_cmd_013() -> Prog {
    Prog(vec![Div, Num(-1), Num(-5)])
}
#[test]
fn test_exec_05_div_cmd_013() {
    assert_eq!(
        prog_05_div_cmd_013().exec(false),
        test_exec_05_div_cmd_013_res()
    )
}
fn test_exec_05_div_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_16_exec_cmd_16() {
    let src = fs::read_to_string("./assets/16_exec_cmd_16.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_16_res())
}
fn test_parse_16_exec_cmd_16_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_16())
}
fn prog_16_exec_cmd_16() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(0),
    ])
}
#[test]
fn test_exec_16_exec_cmd_16() {
    assert_eq!(
        prog_16_exec_cmd_16().exec(false),
        test_exec_16_exec_cmd_16_res()
    )
}
fn test_exec_16_exec_cmd_16_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_08_lt_cmd_005() {
    let src = fs::read_to_string("./assets/08_lt_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_005_res())
}
fn test_parse_08_lt_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_005())
}
fn prog_08_lt_cmd_005() -> Prog {
    Prog(vec![Lt, Num(-5), Num(-1)])
}
#[test]
fn test_exec_08_lt_cmd_005() {
    assert_eq!(
        prog_08_lt_cmd_005().exec(false),
        test_exec_08_lt_cmd_005_res()
    )
}
fn test_exec_08_lt_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_020() {
    let src = fs::read_to_string("./assets/08_lt_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_020_res())
}
fn test_parse_08_lt_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_020())
}
fn prog_08_lt_cmd_020() -> Prog {
    Prog(vec![Lt])
}
#[test]
fn test_exec_08_lt_cmd_020() {
    assert_eq!(
        prog_08_lt_cmd_020().exec(false),
        test_exec_08_lt_cmd_020_res()
    )
}
fn test_exec_08_lt_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_09_gt_cmd_020() {
    let src = fs::read_to_string("./assets/09_gt_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_020_res())
}
fn test_parse_09_gt_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_020())
}
fn prog_09_gt_cmd_020() -> Prog {
    Prog(vec![Gt])
}
#[test]
fn test_exec_09_gt_cmd_020() {
    assert_eq!(
        prog_09_gt_cmd_020().exec(false),
        test_exec_09_gt_cmd_020_res()
    )
}
fn test_exec_09_gt_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_007() {
    let src = fs::read_to_string("./assets/03_sub_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_007_res())
}
fn test_parse_03_sub_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_007())
}
fn prog_03_sub_cmd_007() -> Prog {
    Prog(vec![Sub, Num(2), Num(5)])
}
#[test]
fn test_exec_03_sub_cmd_007() {
    assert_eq!(
        prog_03_sub_cmd_007().exec(false),
        test_exec_03_sub_cmd_007_res()
    )
}
fn test_exec_03_sub_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(-3)
}

#[test]
fn test_parse_16_exec_cmd_13() {
    let src = fs::read_to_string("./assets/16_exec_cmd_13.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_13_res())
}
fn test_parse_16_exec_cmd_13_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_13())
}
fn prog_16_exec_cmd_13() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(0),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_13() {
    assert_eq!(
        prog_16_exec_cmd_13().exec(false),
        test_exec_16_exec_cmd_13_res()
    )
}
fn test_exec_16_exec_cmd_13_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_16_exec_cmd_12() {
    let src = fs::read_to_string("./assets/16_exec_cmd_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_12_res())
}
fn test_parse_16_exec_cmd_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_12())
}
fn prog_16_exec_cmd_12() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(0),
        Cmds(vec![Mul]),
        Cmds(vec![Add]),
        Num(3),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_12() {
    assert_eq!(
        prog_16_exec_cmd_12().exec(false),
        test_exec_16_exec_cmd_12_res()
    )
}
fn test_exec_16_exec_cmd_12_res() -> Result<i64, ProgExecError> {
    Ok(6)
}

#[test]
fn test_parse_70_tailfib() {
    let src = fs::read_to_string("./assets/70_tailfib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_70_tailfib_res())
}
fn test_parse_70_tailfib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_70_tailfib())
}
fn prog_70_tailfib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(0),
        Num(1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Dup,
            Num(4),
            Cmds(vec![Pop, Swap, Pop, Swap]),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Add,
                Dup,
                Num(4),
                Dup,
                Num(0),
                Dup,
                Num(1),
                Sub,
                Dup,
                Num(3),
                Num(1),
            ]),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_70_tailfib() {
    assert_eq!(prog_70_tailfib().exec(false), test_exec_70_tailfib_res())
}
fn test_exec_70_tailfib_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_02_add_cmd_004() {
    let src = fs::read_to_string("./assets/02_add_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_004_res())
}
fn test_parse_02_add_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_004())
}
fn prog_02_add_cmd_004() -> Prog {
    Prog(vec![Add, Num(-10), Num(-2147483648)])
}
#[test]
fn test_exec_02_add_cmd_004() {
    assert_eq!(
        prog_02_add_cmd_004().exec(false),
        test_exec_02_add_cmd_004_res()
    )
}
fn test_exec_02_add_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-2147483658)
}

#[test]
fn test_parse_20_misc_83() {
    let src = fs::read_to_string("./assets/20_misc_83.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_83_res())
}
fn test_parse_20_misc_83_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_83())
}
fn prog_20_misc_83() -> Prog {
    Prog(vec![Ifz, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_83() {
    assert_eq!(prog_20_misc_83().exec(false), test_exec_20_misc_83_res())
}
fn test_exec_20_misc_83_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_12_pop_cmd_09() {
    let src = fs::read_to_string("./assets/12_pop_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_09_res())
}
fn test_parse_12_pop_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_09())
}
fn prog_12_pop_cmd_09() -> Prog {
    Prog(vec![Pop])
}
#[test]
fn test_exec_12_pop_cmd_09() {
    assert_eq!(
        prog_12_pop_cmd_09().exec(false),
        test_exec_12_pop_cmd_09_res()
    )
}
fn test_exec_12_pop_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_015() {
    let src = fs::read_to_string("./assets/08_lt_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_015_res())
}
fn test_parse_08_lt_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_015())
}
fn prog_08_lt_cmd_015() -> Prog {
    Prog(vec![Lt, Num(2147483647), Num(-2)])
}
#[test]
fn test_exec_08_lt_cmd_015() {
    assert_eq!(
        prog_08_lt_cmd_015().exec(false),
        test_exec_08_lt_cmd_015_res()
    )
}
fn test_exec_08_lt_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_13_swap_cmd_08() {
    let src = fs::read_to_string("./assets/13_swap_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_08_res())
}
fn test_parse_13_swap_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_08())
}
fn prog_13_swap_cmd_08() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_08() {
    assert_eq!(
        prog_13_swap_cmd_08().exec(false),
        test_exec_13_swap_cmd_08_res()
    )
}
fn test_exec_13_swap_cmd_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_005() {
    let src = fs::read_to_string("./assets/02_add_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_005_res())
}
fn test_parse_02_add_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_005())
}
fn prog_02_add_cmd_005() -> Prog {
    Prog(vec![Add, Num(2147483647), Num(5)])
}
#[test]
fn test_exec_02_add_cmd_005() {
    assert_eq!(
        prog_02_add_cmd_005().exec(false),
        test_exec_02_add_cmd_005_res()
    )
}
fn test_exec_02_add_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(2147483652)
}

#[test]
fn test_parse_16_exec_cmd_01() {
    let src = fs::read_to_string("./assets/16_exec_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_01_res())
}
fn test_parse_16_exec_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_01())
}
fn prog_16_exec_cmd_01() -> Prog {
    Prog(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(7)])
}
#[test]
fn test_exec_16_exec_cmd_01() {
    assert_eq!(
        prog_16_exec_cmd_01().exec(false),
        test_exec_16_exec_cmd_01_res()
    )
}
fn test_exec_16_exec_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(14)
}

#[test]
fn test_parse_20_misc_20() {
    let src = fs::read_to_string("./assets/20_misc_20.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_20_res())
}
fn test_parse_20_misc_20_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_20())
}
fn prog_20_misc_20() -> Prog {
    Prog(vec![Mul, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_20() {
    assert_eq!(prog_20_misc_20().exec(false), test_exec_20_misc_20_res())
}
fn test_exec_20_misc_20_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_43() {
    let src = fs::read_to_string("./assets/20_misc_43.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_43_res())
}
fn test_parse_20_misc_43_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_43())
}
fn prog_20_misc_43() -> Prog {
    Prog(vec![Rem, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_43() {
    assert_eq!(prog_20_misc_43().exec(false), test_exec_20_misc_43_res())
}
fn test_exec_20_misc_43_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_97() {
    let src = fs::read_to_string("./assets/20_misc_97.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_97_res())
}
fn test_parse_20_misc_97_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_97())
}
fn prog_20_misc_97() -> Prog {
    Prog(vec![Dup, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_97() {
    assert_eq!(prog_20_misc_97().exec(false), test_exec_20_misc_97_res())
}
fn test_exec_20_misc_97_res() -> Result<i64, ProgExecError> {
    Ok(-99)
}

#[test]
fn test_parse_04_mul_cmd_002() {
    let src = fs::read_to_string("./assets/04_mul_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_002_res())
}
fn test_parse_04_mul_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_002())
}
fn prog_04_mul_cmd_002() -> Prog {
    Prog(vec![Mul, Num(1), Num(-2147483648)])
}
#[test]
fn test_exec_04_mul_cmd_002() {
    assert_eq!(
        prog_04_mul_cmd_002().exec(false),
        test_exec_04_mul_cmd_002_res()
    )
}
fn test_exec_04_mul_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(-2147483648)
}

#[test]
fn test_parse_03_sub_cmd_011() {
    let src = fs::read_to_string("./assets/03_sub_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_011_res())
}
fn test_parse_03_sub_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_011())
}
fn prog_03_sub_cmd_011() -> Prog {
    Prog(vec![Sub, Num(2147483648), Num(2147483648)])
}
#[test]
fn test_exec_03_sub_cmd_011() {
    assert_eq!(
        prog_03_sub_cmd_011().exec(false),
        test_exec_03_sub_cmd_011_res()
    )
}
fn test_exec_03_sub_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_11_dup_cmd_11() {
    let src = fs::read_to_string("./assets/11_dup_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_11_res())
}
fn test_parse_11_dup_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_11())
}
fn prog_11_dup_cmd_11() -> Prog {
    Prog(vec![Dup])
}
#[test]
fn test_exec_11_dup_cmd_11() {
    assert_eq!(
        prog_11_dup_cmd_11().exec(false),
        test_exec_11_dup_cmd_11_res()
    )
}
fn test_exec_11_dup_cmd_11_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_78() {
    let src = fs::read_to_string("./assets/20_misc_78.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_78_res())
}
fn test_parse_20_misc_78_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_78())
}
fn prog_20_misc_78() -> Prog {
    Prog(vec![Exec, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_78() {
    assert_eq!(prog_20_misc_78().exec(false), test_exec_20_misc_78_res())
}
fn test_exec_20_misc_78_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_67() {
    let src = fs::read_to_string("./assets/20_misc_67.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_67_res())
}
fn test_parse_20_misc_67_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_67())
}
fn prog_20_misc_67() -> Prog {
    Prog(vec![Gt, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_67() {
    assert_eq!(prog_20_misc_67().exec(false), test_exec_20_misc_67_res())
}
fn test_exec_20_misc_67_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_61_fib() {
    let src = fs::read_to_string("./assets/61_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_61_fib_res())
}
fn test_parse_61_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_61_fib())
}
fn prog_61_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(15),
    ])
}
#[test]
fn test_exec_61_fib() {
    assert_eq!(prog_61_fib().exec(false), test_exec_61_fib_res())
}
fn test_exec_61_fib_res() -> Result<i64, ProgExecError> {
    Ok(987)
}

#[test]
fn test_parse_01_num_cmd_02() {
    let src = fs::read_to_string("./assets/01_num_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_02_res())
}
fn test_parse_01_num_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_02())
}
fn prog_01_num_cmd_02() -> Prog {
    Prog(vec![Num(1), Num(2), Num(3), Num(4), Num(5), Num(42)])
}
#[test]
fn test_exec_01_num_cmd_02() {
    assert_eq!(
        prog_01_num_cmd_02().exec(false),
        test_exec_01_num_cmd_02_res()
    )
}
fn test_exec_01_num_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_01_num_cmd_04() {
    let src = fs::read_to_string("./assets/01_num_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_04_res())
}
fn test_parse_01_num_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_04())
}
fn prog_01_num_cmd_04() -> Prog {
    Prog(vec![Num(2147483647)])
}
#[test]
fn test_exec_01_num_cmd_04() {
    assert_eq!(
        prog_01_num_cmd_04().exec(false),
        test_exec_01_num_cmd_04_res()
    )
}
fn test_exec_01_num_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_20_misc_93() {
    let src = fs::read_to_string("./assets/20_misc_93.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_93_res())
}
fn test_parse_20_misc_93_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_93())
}
fn prog_20_misc_93() -> Prog {
    Prog(vec![Dup, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_93() {
    assert_eq!(prog_20_misc_93().exec(false), test_exec_20_misc_93_res())
}
fn test_exec_20_misc_93_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_18() {
    let src = fs::read_to_string("./assets/20_misc_18.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_18_res())
}
fn test_parse_20_misc_18_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_18())
}
fn prog_20_misc_18() -> Prog {
    Prog(vec![Mul])
}
#[test]
fn test_exec_20_misc_18() {
    assert_eq!(prog_20_misc_18().exec(false), test_exec_20_misc_18_res())
}
fn test_exec_20_misc_18_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_11_dup_cmd_05() {
    let src = fs::read_to_string("./assets/11_dup_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_05_res())
}
fn test_parse_11_dup_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_05())
}
fn prog_11_dup_cmd_05() -> Prog {
    Prog(vec![Dup, Num(0), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_05() {
    assert_eq!(
        prog_11_dup_cmd_05().exec(false),
        test_exec_11_dup_cmd_05_res()
    )
}
fn test_exec_11_dup_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_20_misc_14() {
    let src = fs::read_to_string("./assets/20_misc_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_14_res())
}
fn test_parse_20_misc_14_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_14())
}
fn prog_20_misc_14() -> Prog {
    Prog(vec![Sub, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_14() {
    assert_eq!(prog_20_misc_14().exec(false), test_exec_20_misc_14_res())
}
fn test_exec_20_misc_14_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_50_fact() {
    let src = fs::read_to_string("./assets/50_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_50_fact_res())
}
fn test_parse_50_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_50_fact())
}
fn prog_50_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_50_fact() {
    assert_eq!(prog_50_fact().exec(false), test_exec_50_fact_res())
}
fn test_exec_50_fact_res() -> Result<i64, ProgExecError> {
    Ok(120)
}

#[test]
fn test_parse_20_misc_33() {
    let src = fs::read_to_string("./assets/20_misc_33.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_33_res())
}
fn test_parse_20_misc_33_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_33())
}
fn prog_20_misc_33() -> Prog {
    Prog(vec![Div, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_33() {
    assert_eq!(prog_20_misc_33().exec(false), test_exec_20_misc_33_res())
}
fn test_exec_20_misc_33_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_005() {
    let src = fs::read_to_string("./assets/06_rem_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_005_res())
}
fn test_parse_06_rem_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_005())
}
fn prog_06_rem_cmd_005() -> Prog {
    Prog(vec![Rem, Num(2147483648), Num(2147483647)])
}
#[test]
fn test_exec_06_rem_cmd_005() {
    assert_eq!(
        prog_06_rem_cmd_005().exec(false),
        test_exec_06_rem_cmd_005_res()
    )
}
fn test_exec_06_rem_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_68() {
    let src = fs::read_to_string("./assets/20_misc_68.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_68_res())
}
fn test_parse_20_misc_68_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_68())
}
fn prog_20_misc_68() -> Prog {
    Prog(vec![Gt, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_68() {
    assert_eq!(prog_20_misc_68().exec(false), test_exec_20_misc_68_res())
}
fn test_exec_20_misc_68_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_005() {
    let src = fs::read_to_string("./assets/07_eq_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_005_res())
}
fn test_parse_07_eq_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_005())
}
fn prog_07_eq_cmd_005() -> Prog {
    Prog(vec![Eq, Num(-2147483648), Num(2147483647)])
}
#[test]
fn test_exec_07_eq_cmd_005() {
    assert_eq!(
        prog_07_eq_cmd_005().exec(false),
        test_exec_07_eq_cmd_005_res()
    )
}
fn test_exec_07_eq_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_22() {
    let src = fs::read_to_string("./assets/20_misc_22.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_22_res())
}
fn test_parse_20_misc_22_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_22())
}
fn prog_20_misc_22() -> Prog {
    Prog(vec![Mul, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_22() {
    assert_eq!(prog_20_misc_22().exec(false), test_exec_20_misc_22_res())
}
fn test_exec_20_misc_22_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_019() {
    let src = fs::read_to_string("./assets/03_sub_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_019_res())
}
fn test_parse_03_sub_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_019())
}
fn prog_03_sub_cmd_019() -> Prog {
    Prog(vec![Sub, Num(10)])
}
#[test]
fn test_exec_03_sub_cmd_019() {
    assert_eq!(
        prog_03_sub_cmd_019().exec(false),
        test_exec_03_sub_cmd_019_res()
    )
}
fn test_exec_03_sub_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_07() {
    let src = fs::read_to_string("./assets/00_example_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_07_res())
}
fn test_parse_00_example_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_07())
}
fn prog_00_example_07() -> Prog {
    Prog(vec![
        Mul,
        Add,
        Mul,
        Add,
        Dup,
        Num(-1),
        Dup,
        Num(1),
        Num(1),
        Num(2),
        Num(3),
    ])
}
#[test]
fn test_exec_00_example_07() {
    assert_eq!(
        prog_00_example_07().exec(false),
        test_exec_00_example_07_res()
    )
}
fn test_exec_00_example_07_res() -> Result<i64, ProgExecError> {
    Ok(21)
}

#[test]
fn test_parse_20_misc_98() {
    let src = fs::read_to_string("./assets/20_misc_98.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_98_res())
}
fn test_parse_20_misc_98_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_98())
}
fn prog_20_misc_98() -> Prog {
    Prog(vec![Dup, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_98() {
    assert_eq!(prog_20_misc_98().exec(false), test_exec_20_misc_98_res())
}
fn test_exec_20_misc_98_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_09() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_09_res())
}
fn test_parse_15_cmds_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_09())
}
fn prog_15_cmds_cmd_09() -> Prog {
    Prog(vec![Cmds(vec![Mul, Num(2)])])
}
#[test]
fn test_exec_15_cmds_cmd_09() {
    assert_eq!(
        prog_15_cmds_cmd_09().exec(false),
        test_exec_15_cmds_cmd_09_res()
    )
}
fn test_exec_15_cmds_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_001() {
    let src = fs::read_to_string("./assets/05_div_cmd_001.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_001_res())
}
fn test_parse_05_div_cmd_001_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_001())
}
fn prog_05_div_cmd_001() -> Prog {
    Prog(vec![Div, Num(1), Num(2147483646)])
}
#[test]
fn test_exec_05_div_cmd_001() {
    assert_eq!(
        prog_05_div_cmd_001().exec(false),
        test_exec_05_div_cmd_001_res()
    )
}
fn test_exec_05_div_cmd_001_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_01_num_cmd_07() {
    let src = fs::read_to_string("./assets/01_num_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_07_res())
}
fn test_parse_01_num_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_07())
}
fn prog_01_num_cmd_07() -> Prog {
    Prog(vec![Num(9223372036854775807)])
}
#[test]
fn test_exec_01_num_cmd_07() {
    assert_eq!(
        prog_01_num_cmd_07().exec(false),
        test_exec_01_num_cmd_07_res()
    )
}
fn test_exec_01_num_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(9223372036854775807)
}

#[test]
fn test_parse_04_mul_cmd_010() {
    let src = fs::read_to_string("./assets/04_mul_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_010_res())
}
fn test_parse_04_mul_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_010())
}
fn prog_04_mul_cmd_010() -> Prog {
    Prog(vec![Mul, Num(3), Num(3)])
}
#[test]
fn test_exec_04_mul_cmd_010() {
    assert_eq!(
        prog_04_mul_cmd_010().exec(false),
        test_exec_04_mul_cmd_010_res()
    )
}
fn test_exec_04_mul_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_52_fact() {
    let src = fs::read_to_string("./assets/52_fact.bir").unwrap();
    assert_eq!(src.parse(), test_parse_52_fact_res())
}
fn test_parse_52_fact_res() -> Result<Prog, ProgParseError> {
    Ok(prog_52_fact())
}
fn prog_52_fact() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Mul,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(20),
    ])
}
#[test]
fn test_exec_52_fact() {
    assert_eq!(prog_52_fact().exec(false), test_exec_52_fact_res())
}
fn test_exec_52_fact_res() -> Result<i64, ProgExecError> {
    Ok(2432902008176640000)
}

#[test]
fn test_parse_20_misc_47() {
    let src = fs::read_to_string("./assets/20_misc_47.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_47_res())
}
fn test_parse_20_misc_47_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_47())
}
fn prog_20_misc_47() -> Prog {
    Prog(vec![Eq, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_47() {
    assert_eq!(prog_20_misc_47().exec(false), test_exec_20_misc_47_res())
}
fn test_exec_20_misc_47_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_007() {
    let src = fs::read_to_string("./assets/06_rem_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_007_res())
}
fn test_parse_06_rem_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_007())
}
fn prog_06_rem_cmd_007() -> Prog {
    Prog(vec![Rem, Num(10), Num(2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_007() {
    assert_eq!(
        prog_06_rem_cmd_007().exec(false),
        test_exec_06_rem_cmd_007_res()
    )
}
fn test_exec_06_rem_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_05_div_cmd_011() {
    let src = fs::read_to_string("./assets/05_div_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_011_res())
}
fn test_parse_05_div_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_011())
}
fn prog_05_div_cmd_011() -> Prog {
    Prog(vec![Div, Num(2147483646), Num(-3)])
}
#[test]
fn test_exec_05_div_cmd_011() {
    assert_eq!(
        prog_05_div_cmd_011().exec(false),
        test_exec_05_div_cmd_011_res()
    )
}
fn test_exec_05_div_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(-715827882)
}

#[test]
fn test_parse_03_sub_cmd_017() {
    let src = fs::read_to_string("./assets/03_sub_cmd_017.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_017_res())
}
fn test_parse_03_sub_cmd_017_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_017())
}
fn prog_03_sub_cmd_017() -> Prog {
    Prog(vec![Sub, Num(1), Num(-2147483648)])
}
#[test]
fn test_exec_03_sub_cmd_017() {
    assert_eq!(
        prog_03_sub_cmd_017().exec(false),
        test_exec_03_sub_cmd_017_res()
    )
}
fn test_exec_03_sub_cmd_017_res() -> Result<i64, ProgExecError> {
    Ok(2147483649)
}

#[test]
fn test_parse_20_misc_15() {
    let src = fs::read_to_string("./assets/20_misc_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_15_res())
}
fn test_parse_20_misc_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_15())
}
fn prog_20_misc_15() -> Prog {
    Prog(vec![Sub, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_15() {
    assert_eq!(prog_20_misc_15().exec(false), test_exec_20_misc_15_res())
}
fn test_exec_20_misc_15_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_002() {
    let src = fs::read_to_string("./assets/02_add_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_002_res())
}
fn test_parse_02_add_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_002())
}
fn prog_02_add_cmd_002() -> Prog {
    Prog(vec![Add, Num(-2147483646), Num(1)])
}
#[test]
fn test_exec_02_add_cmd_002() {
    assert_eq!(
        prog_02_add_cmd_002().exec(false),
        test_exec_02_add_cmd_002_res()
    )
}
fn test_exec_02_add_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(-2147483645)
}

#[test]
fn test_parse_16_exec_cmd_06() {
    let src = fs::read_to_string("./assets/16_exec_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_06_res())
}
fn test_parse_16_exec_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_06())
}
fn prog_16_exec_cmd_06() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![Mul]),
        Swap,
        Cmds(vec![Add]),
        Lt,
        Swap,
        Num(4),
        Num(3),
        Num(5),
        Num(6),
    ])
}
#[test]
fn test_exec_16_exec_cmd_06() {
    assert_eq!(
        prog_16_exec_cmd_06().exec(false),
        test_exec_16_exec_cmd_06_res()
    )
}
fn test_exec_16_exec_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(11)
}

#[test]
fn test_parse_11_dup_cmd_00() {
    let src = fs::read_to_string("./assets/11_dup_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_00_res())
}
fn test_parse_11_dup_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_00())
}
fn prog_11_dup_cmd_00() -> Prog {
    Prog(vec![Dup, Num(-5), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_00() {
    assert_eq!(
        prog_11_dup_cmd_00().exec(false),
        test_exec_11_dup_cmd_00_res()
    )
}
fn test_exec_11_dup_cmd_00_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_012() {
    let src = fs::read_to_string("./assets/07_eq_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_012_res())
}
fn test_parse_07_eq_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_012())
}
fn prog_07_eq_cmd_012() -> Prog {
    Prog(vec![Eq, Num(10), Num(-2147483646)])
}
#[test]
fn test_exec_07_eq_cmd_012() {
    assert_eq!(
        prog_07_eq_cmd_012().exec(false),
        test_exec_07_eq_cmd_012_res()
    )
}
fn test_exec_07_eq_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_05_div_cmd_002() {
    let src = fs::read_to_string("./assets/05_div_cmd_002.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_002_res())
}
fn test_parse_05_div_cmd_002_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_002())
}
fn prog_05_div_cmd_002() -> Prog {
    Prog(vec![Div, Num(5), Num(2)])
}
#[test]
fn test_exec_05_div_cmd_002() {
    assert_eq!(
        prog_05_div_cmd_002().exec(false),
        test_exec_05_div_cmd_002_res()
    )
}
fn test_exec_05_div_cmd_002_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_08_lt_cmd_010() {
    let src = fs::read_to_string("./assets/08_lt_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_010_res())
}
fn test_parse_08_lt_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_010())
}
fn prog_08_lt_cmd_010() -> Prog {
    Prog(vec![Lt, Num(-2), Num(1)])
}
#[test]
fn test_exec_08_lt_cmd_010() {
    assert_eq!(
        prog_08_lt_cmd_010().exec(false),
        test_exec_08_lt_cmd_010_res()
    )
}
fn test_exec_08_lt_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_04_mul_cmd_007() {
    let src = fs::read_to_string("./assets/04_mul_cmd_007.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_007_res())
}
fn test_parse_04_mul_cmd_007_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_007())
}
fn prog_04_mul_cmd_007() -> Prog {
    Prog(vec![Mul, Num(2147483647), Num(-2147483646)])
}
#[test]
fn test_exec_04_mul_cmd_007() {
    assert_eq!(
        prog_04_mul_cmd_007().exec(false),
        test_exec_04_mul_cmd_007_res()
    )
}
fn test_exec_04_mul_cmd_007_res() -> Result<i64, ProgExecError> {
    Ok(-4611686011984936962)
}

#[test]
fn test_parse_09_gt_cmd_010() {
    let src = fs::read_to_string("./assets/09_gt_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_010_res())
}
fn test_parse_09_gt_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_010())
}
fn prog_09_gt_cmd_010() -> Prog {
    Prog(vec![Gt, Num(10), Num(2147483648)])
}
#[test]
fn test_exec_09_gt_cmd_010() {
    assert_eq!(
        prog_09_gt_cmd_010().exec(false),
        test_exec_09_gt_cmd_010_res()
    )
}
fn test_exec_09_gt_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_04() {
    let src = fs::read_to_string("./assets/20_misc_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_04_res())
}
fn test_parse_20_misc_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_04())
}
fn prog_20_misc_04() -> Prog {
    Prog(vec![Add, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_04() {
    assert_eq!(prog_20_misc_04().exec(false), test_exec_20_misc_04_res())
}
fn test_exec_20_misc_04_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_10() {
    let src = fs::read_to_string("./assets/20_misc_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_10_res())
}
fn test_parse_20_misc_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_10())
}
fn prog_20_misc_10() -> Prog {
    Prog(vec![Sub, Num(1)])
}
#[test]
fn test_exec_20_misc_10() {
    assert_eq!(prog_20_misc_10().exec(false), test_exec_20_misc_10_res())
}
fn test_exec_20_misc_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_003() {
    let src = fs::read_to_string("./assets/02_add_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_003_res())
}
fn test_parse_02_add_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_003())
}
fn prog_02_add_cmd_003() -> Prog {
    Prog(vec![Add, Num(-2), Num(2)])
}
#[test]
fn test_exec_02_add_cmd_003() {
    assert_eq!(
        prog_02_add_cmd_003().exec(false),
        test_exec_02_add_cmd_003_res()
    )
}
fn test_exec_02_add_cmd_003_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_37() {
    let src = fs::read_to_string("./assets/20_misc_37.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_37_res())
}
fn test_parse_20_misc_37_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_37())
}
fn prog_20_misc_37() -> Prog {
    Prog(vec![Rem, Num(1)])
}
#[test]
fn test_exec_20_misc_37() {
    assert_eq!(prog_20_misc_37().exec(false), test_exec_20_misc_37_res())
}
fn test_exec_20_misc_37_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_27() {
    let src = fs::read_to_string("./assets/20_misc_27.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_27_res())
}
fn test_parse_20_misc_27_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_27())
}
fn prog_20_misc_27() -> Prog {
    Prog(vec![Div])
}
#[test]
fn test_exec_20_misc_27() {
    assert_eq!(prog_20_misc_27().exec(false), test_exec_20_misc_27_res())
}
fn test_exec_20_misc_27_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_07_eq_cmd_009() {
    let src = fs::read_to_string("./assets/07_eq_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_009_res())
}
fn test_parse_07_eq_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_009())
}
fn prog_07_eq_cmd_009() -> Prog {
    Prog(vec![Eq, Num(3), Num(-3)])
}
#[test]
fn test_exec_07_eq_cmd_009() {
    assert_eq!(
        prog_07_eq_cmd_009().exec(false),
        test_exec_07_eq_cmd_009_res()
    )
}
fn test_exec_07_eq_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_07_eq_cmd_019() {
    let src = fs::read_to_string("./assets/07_eq_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_019_res())
}
fn test_parse_07_eq_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_019())
}
fn prog_07_eq_cmd_019() -> Prog {
    Prog(vec![Eq, Num(3)])
}
#[test]
fn test_exec_07_eq_cmd_019() {
    assert_eq!(
        prog_07_eq_cmd_019().exec(false),
        test_exec_07_eq_cmd_019_res()
    )
}
fn test_exec_07_eq_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_03() {
    let src = fs::read_to_string("./assets/16_exec_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_03_res())
}
fn test_parse_16_exec_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_03())
}
fn prog_16_exec_cmd_03() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Cmds(vec![Exec, Swap, Num(7)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_03() {
    assert_eq!(
        prog_16_exec_cmd_03().exec(false),
        test_exec_16_exec_cmd_03_res()
    )
}
fn test_exec_16_exec_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(-7)
}

#[test]
fn test_parse_03_sub_cmd_008() {
    let src = fs::read_to_string("./assets/03_sub_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_008_res())
}
fn test_parse_03_sub_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_008())
}
fn prog_03_sub_cmd_008() -> Prog {
    Prog(vec![Sub, Num(5), Num(2147483646)])
}
#[test]
fn test_exec_03_sub_cmd_008() {
    assert_eq!(
        prog_03_sub_cmd_008().exec(false),
        test_exec_03_sub_cmd_008_res()
    )
}
fn test_exec_03_sub_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-2147483641)
}

#[test]
fn test_parse_13_swap_cmd_07() {
    let src = fs::read_to_string("./assets/13_swap_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_07_res())
}
fn test_parse_13_swap_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_07())
}
fn prog_13_swap_cmd_07() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_07() {
    assert_eq!(
        prog_13_swap_cmd_07().exec(false),
        test_exec_13_swap_cmd_07_res()
    )
}
fn test_exec_13_swap_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_19() {
    let src = fs::read_to_string("./assets/14_rev_cmd_19.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_19_res())
}
fn test_parse_14_rev_cmd_19_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_19())
}
fn prog_14_rev_cmd_19() -> Prog {
    Prog(vec![Rev])
}
#[test]
fn test_exec_14_rev_cmd_19() {
    assert_eq!(
        prog_14_rev_cmd_19().exec(false),
        test_exec_14_rev_cmd_19_res()
    )
}
fn test_exec_14_rev_cmd_19_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_06() {
    let src = fs::read_to_string("./assets/20_misc_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_06_res())
}
fn test_parse_20_misc_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_06())
}
fn prog_20_misc_06() -> Prog {
    Prog(vec![Add, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_06() {
    assert_eq!(prog_20_misc_06().exec(false), test_exec_20_misc_06_res())
}
fn test_exec_20_misc_06_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_86() {
    let src = fs::read_to_string("./assets/20_misc_86.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_86_res())
}
fn test_parse_20_misc_86_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_86())
}
fn prog_20_misc_86() -> Prog {
    Prog(vec![Ifz, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_86() {
    assert_eq!(prog_20_misc_86().exec(false), test_exec_20_misc_86_res())
}
fn test_exec_20_misc_86_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_13_swap_cmd_10() {
    let src = fs::read_to_string("./assets/13_swap_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_10_res())
}
fn test_parse_13_swap_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_10())
}
fn prog_13_swap_cmd_10() -> Prog {
    Prog(vec![Swap])
}
#[test]
fn test_exec_13_swap_cmd_10() {
    assert_eq!(
        prog_13_swap_cmd_10().exec(false),
        test_exec_13_swap_cmd_10_res()
    )
}
fn test_exec_13_swap_cmd_10_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_15() {
    let src = fs::read_to_string("./assets/14_rev_cmd_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_15_res())
}
fn test_parse_14_rev_cmd_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_15())
}
fn prog_14_rev_cmd_15() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_15() {
    assert_eq!(
        prog_14_rev_cmd_15().exec(false),
        test_exec_14_rev_cmd_15_res()
    )
}
fn test_exec_14_rev_cmd_15_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_02_add_cmd_014() {
    let src = fs::read_to_string("./assets/02_add_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_014_res())
}
fn test_parse_02_add_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_014())
}
fn prog_02_add_cmd_014() -> Prog {
    Prog(vec![Add, Num(2147483648), Num(-3)])
}
#[test]
fn test_exec_02_add_cmd_014() {
    assert_eq!(
        prog_02_add_cmd_014().exec(false),
        test_exec_02_add_cmd_014_res()
    )
}
fn test_exec_02_add_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(2147483645)
}

#[test]
fn test_parse_09_gt_cmd_012() {
    let src = fs::read_to_string("./assets/09_gt_cmd_012.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_012_res())
}
fn test_parse_09_gt_cmd_012_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_012())
}
fn prog_09_gt_cmd_012() -> Prog {
    Prog(vec![Gt, Num(1), Num(-2147483647)])
}
#[test]
fn test_exec_09_gt_cmd_012() {
    assert_eq!(
        prog_09_gt_cmd_012().exec(false),
        test_exec_09_gt_cmd_012_res()
    )
}
fn test_exec_09_gt_cmd_012_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_08_lt_cmd_004() {
    let src = fs::read_to_string("./assets/08_lt_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_004_res())
}
fn test_parse_08_lt_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_004())
}
fn prog_08_lt_cmd_004() -> Prog {
    Prog(vec![Lt, Num(1), Num(-5)])
}
#[test]
fn test_exec_08_lt_cmd_004() {
    assert_eq!(
        prog_08_lt_cmd_004().exec(false),
        test_exec_08_lt_cmd_004_res()
    )
}
fn test_exec_08_lt_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_01_num_cmd_06() {
    let src = fs::read_to_string("./assets/01_num_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_06_res())
}
fn test_parse_01_num_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_06())
}
fn prog_01_num_cmd_06() -> Prog {
    Prog(vec![Num(2147483649), Num(2147483648), Num(2147483647)])
}
#[test]
fn test_exec_01_num_cmd_06() {
    assert_eq!(
        prog_01_num_cmd_06().exec(false),
        test_exec_01_num_cmd_06_res()
    )
}
fn test_exec_01_num_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(2147483649)
}

#[test]
fn test_parse_11_dup_cmd_06() {
    let src = fs::read_to_string("./assets/11_dup_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_11_dup_cmd_06_res())
}
fn test_parse_11_dup_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_11_dup_cmd_06())
}
fn prog_11_dup_cmd_06() -> Prog {
    Prog(vec![Dup, Num(1), Num(-4), Num(3), Num(-2), Num(1)])
}
#[test]
fn test_exec_11_dup_cmd_06() {
    assert_eq!(
        prog_11_dup_cmd_06().exec(false),
        test_exec_11_dup_cmd_06_res()
    )
}
fn test_exec_11_dup_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_16_exec_cmd_10() {
    let src = fs::read_to_string("./assets/16_exec_cmd_10.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_10_res())
}
fn test_parse_16_exec_cmd_10_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_10())
}
fn prog_16_exec_cmd_10() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(3),
        Cmds(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(5)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_10() {
    assert_eq!(
        prog_16_exec_cmd_10().exec(false),
        test_exec_16_exec_cmd_10_res()
    )
}
fn test_exec_16_exec_cmd_10_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_14_rev_cmd_08() {
    let src = fs::read_to_string("./assets/14_rev_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_08_res())
}
fn test_parse_14_rev_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_08())
}
fn prog_14_rev_cmd_08() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_08() {
    assert_eq!(
        prog_14_rev_cmd_08().exec(false),
        test_exec_14_rev_cmd_08_res()
    )
}
fn test_exec_14_rev_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_92() {
    let src = fs::read_to_string("./assets/20_misc_92.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_92_res())
}
fn test_parse_20_misc_92_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_92())
}
fn prog_20_misc_92() -> Prog {
    Prog(vec![Dup, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_92() {
    assert_eq!(prog_20_misc_92().exec(false), test_exec_20_misc_92_res())
}
fn test_exec_20_misc_92_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_06() {
    let src = fs::read_to_string("./assets/00_example_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_06_res())
}
fn test_parse_00_example_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_06())
}
fn prog_00_example_06() -> Prog {
    Prog(vec![Div, Num(16), Num(8), Num(4), Num(2)])
}
#[test]
fn test_exec_00_example_06() {
    assert_eq!(
        prog_00_example_06().exec(false),
        test_exec_00_example_06_res()
    )
}
fn test_exec_00_example_06_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_16_exec_cmd_09() {
    let src = fs::read_to_string("./assets/16_exec_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_09_res())
}
fn test_parse_16_exec_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_09())
}
fn prog_16_exec_cmd_09() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(1),
        Cmds(vec![Sub, Swap, Mul, Num(2), Swap]),
        Num(10),
    ])
}
#[test]
fn test_exec_16_exec_cmd_09() {
    assert_eq!(
        prog_16_exec_cmd_09().exec(false),
        test_exec_16_exec_cmd_09_res()
    )
}
fn test_exec_16_exec_cmd_09_res() -> Result<i64, ProgExecError> {
    Ok(-19)
}

#[test]
fn test_parse_06_rem_cmd_019() {
    let src = fs::read_to_string("./assets/06_rem_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_019_res())
}
fn test_parse_06_rem_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_019())
}
fn prog_06_rem_cmd_019() -> Prog {
    Prog(vec![Rem, Num(-2147483648)])
}
#[test]
fn test_exec_06_rem_cmd_019() {
    assert_eq!(
        prog_06_rem_cmd_019().exec(false),
        test_exec_06_rem_cmd_019_res()
    )
}
fn test_exec_06_rem_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_15_cmds_cmd_04() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_04_res())
}
fn test_parse_15_cmds_cmd_04_res() -> Result<Prog, ProgParseError> {
    Err(ProgParseError)
}

#[test]
fn test_parse_09_gt_cmd_014() {
    let src = fs::read_to_string("./assets/09_gt_cmd_014.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_014_res())
}
fn test_parse_09_gt_cmd_014_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_014())
}
fn prog_09_gt_cmd_014() -> Prog {
    Prog(vec![Gt, Num(-3), Num(5)])
}
#[test]
fn test_exec_09_gt_cmd_014() {
    assert_eq!(
        prog_09_gt_cmd_014().exec(false),
        test_exec_09_gt_cmd_014_res()
    )
}
fn test_exec_09_gt_cmd_014_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_13_swap_cmd_05() {
    let src = fs::read_to_string("./assets/13_swap_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_05_res())
}
fn test_parse_13_swap_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_05())
}
fn prog_13_swap_cmd_05() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_05() {
    assert_eq!(
        prog_13_swap_cmd_05().exec(false),
        test_exec_13_swap_cmd_05_res()
    )
}
fn test_exec_13_swap_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_80_tak() {
    let src = fs::read_to_string("./assets/80_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_80_tak_res())
}
fn test_parse_80_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_80_tak())
}
fn prog_80_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(24),
        Num(16),
        Num(8),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_80_tak() {
    assert_eq!(prog_80_tak().exec(false), test_exec_80_tak_res())
}
fn test_exec_80_tak_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_20_misc_77() {
    let src = fs::read_to_string("./assets/20_misc_77.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_77_res())
}
fn test_parse_20_misc_77_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_77())
}
fn prog_20_misc_77() -> Prog {
    Prog(vec![Exec, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_77() {
    assert_eq!(prog_20_misc_77().exec(false), test_exec_20_misc_77_res())
}
fn test_exec_20_misc_77_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_15_cmds_cmd_08() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_08_res())
}
fn test_parse_15_cmds_cmd_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_08())
}
fn prog_15_cmds_cmd_08() -> Prog {
    Prog(vec![Ifz, Cmds(vec![]), Num(2), Num(3)])
}
#[test]
fn test_exec_15_cmds_cmd_08() {
    assert_eq!(
        prog_15_cmds_cmd_08().exec(false),
        test_exec_15_cmds_cmd_08_res()
    )
}
fn test_exec_15_cmds_cmd_08_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_08_lt_cmd_011() {
    let src = fs::read_to_string("./assets/08_lt_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_011_res())
}
fn test_parse_08_lt_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_011())
}
fn prog_08_lt_cmd_011() -> Prog {
    Prog(vec![Lt, Num(-10), Num(10)])
}
#[test]
fn test_exec_08_lt_cmd_011() {
    assert_eq!(
        prog_08_lt_cmd_011().exec(false),
        test_exec_08_lt_cmd_011_res()
    )
}
fn test_exec_08_lt_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_09_gt_cmd_006() {
    let src = fs::read_to_string("./assets/09_gt_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_006_res())
}
fn test_parse_09_gt_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_006())
}
fn prog_09_gt_cmd_006() -> Prog {
    Prog(vec![Gt, Num(-2), Num(3)])
}
#[test]
fn test_exec_09_gt_cmd_006() {
    assert_eq!(
        prog_09_gt_cmd_006().exec(false),
        test_exec_09_gt_cmd_006_res()
    )
}
fn test_exec_09_gt_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_00_example_05() {
    let src = fs::read_to_string("./assets/00_example_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_05_res())
}
fn test_parse_00_example_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_05())
}
fn prog_00_example_05() -> Prog {
    Prog(vec![Sub, Num(5), Num(10)])
}
#[test]
fn test_exec_00_example_05() {
    assert_eq!(
        prog_00_example_05().exec(false),
        test_exec_00_example_05_res()
    )
}
fn test_exec_00_example_05_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_20_misc_11() {
    let src = fs::read_to_string("./assets/20_misc_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_11_res())
}
fn test_parse_20_misc_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_11())
}
fn prog_20_misc_11() -> Prog {
    Prog(vec![Sub, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_11() {
    assert_eq!(prog_20_misc_11().exec(false), test_exec_20_misc_11_res())
}
fn test_exec_20_misc_11_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_008() {
    let src = fs::read_to_string("./assets/04_mul_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_008_res())
}
fn test_parse_04_mul_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_008())
}
fn prog_04_mul_cmd_008() -> Prog {
    Prog(vec![Mul, Num(2147483646), Num(-10)])
}
#[test]
fn test_exec_04_mul_cmd_008() {
    assert_eq!(
        prog_04_mul_cmd_008().exec(false),
        test_exec_04_mul_cmd_008_res()
    )
}
fn test_exec_04_mul_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-21474836460)
}

#[test]
fn test_parse_07_eq_cmd_013() {
    let src = fs::read_to_string("./assets/07_eq_cmd_013.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_013_res())
}
fn test_parse_07_eq_cmd_013_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_013())
}
fn prog_07_eq_cmd_013() -> Prog {
    Prog(vec![Eq, Num(-5), Num(2147483648)])
}
#[test]
fn test_exec_07_eq_cmd_013() {
    assert_eq!(
        prog_07_eq_cmd_013().exec(false),
        test_exec_07_eq_cmd_013_res()
    )
}
fn test_exec_07_eq_cmd_013_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_39() {
    let src = fs::read_to_string("./assets/20_misc_39.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_39_res())
}
fn test_parse_20_misc_39_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_39())
}
fn prog_20_misc_39() -> Prog {
    Prog(vec![Rem, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_39() {
    assert_eq!(prog_20_misc_39().exec(false), test_exec_20_misc_39_res())
}
fn test_exec_20_misc_39_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_20_misc_62() {
    let src = fs::read_to_string("./assets/20_misc_62.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_62_res())
}
fn test_parse_20_misc_62_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_62())
}
fn prog_20_misc_62() -> Prog {
    Prog(vec![Lt, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_62() {
    assert_eq!(prog_20_misc_62().exec(false), test_exec_20_misc_62_res())
}
fn test_exec_20_misc_62_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_02_add_cmd_018() {
    let src = fs::read_to_string("./assets/02_add_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_018_res())
}
fn test_parse_02_add_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_018())
}
fn prog_02_add_cmd_018() -> Prog {
    Prog(vec![Add, Num(0), Num(0)])
}
#[test]
fn test_exec_02_add_cmd_018() {
    assert_eq!(
        prog_02_add_cmd_018().exec(false),
        test_exec_02_add_cmd_018_res()
    )
}
fn test_exec_02_add_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_31() {
    let src = fs::read_to_string("./assets/20_misc_31.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_31_res())
}
fn test_parse_20_misc_31_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_31())
}
fn prog_20_misc_31() -> Prog {
    Prog(vec![Div, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_31() {
    assert_eq!(prog_20_misc_31().exec(false), test_exec_20_misc_31_res())
}
fn test_exec_20_misc_31_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_005() {
    let src = fs::read_to_string("./assets/04_mul_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_005_res())
}
fn test_parse_04_mul_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_005())
}
fn prog_04_mul_cmd_005() -> Prog {
    Prog(vec![Mul, Num(0), Num(2147483647)])
}
#[test]
fn test_exec_04_mul_cmd_005() {
    assert_eq!(
        prog_04_mul_cmd_005().exec(false),
        test_exec_04_mul_cmd_005_res()
    )
}
fn test_exec_04_mul_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_16_exec_cmd_23() {
    let src = fs::read_to_string("./assets/16_exec_cmd_23.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_23_res())
}
fn test_parse_16_exec_cmd_23_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_23())
}
fn prog_16_exec_cmd_23() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Num(1),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_23() {
    assert_eq!(
        prog_16_exec_cmd_23().exec(false),
        test_exec_16_exec_cmd_23_res()
    )
}
fn test_exec_16_exec_cmd_23_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_08() {
    let src = fs::read_to_string("./assets/20_misc_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_08_res())
}
fn test_parse_20_misc_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_08())
}
fn prog_20_misc_08() -> Prog {
    Prog(vec![Add, Cmds(vec![Num(1)]), Cmds(vec![Num(2)]), Num(-99)])
}
#[test]
fn test_exec_20_misc_08() {
    assert_eq!(prog_20_misc_08().exec(false), test_exec_20_misc_08_res())
}
fn test_exec_20_misc_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_81() {
    let src = fs::read_to_string("./assets/20_misc_81.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_81_res())
}
fn test_parse_20_misc_81_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_81())
}
fn prog_20_misc_81() -> Prog {
    Prog(vec![Ifz])
}
#[test]
fn test_exec_20_misc_81() {
    assert_eq!(prog_20_misc_81().exec(false), test_exec_20_misc_81_res())
}
fn test_exec_20_misc_81_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_15() {
    let src = fs::read_to_string("./assets/16_exec_cmd_15.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_15_res())
}
fn test_parse_16_exec_cmd_15_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_15())
}
fn prog_16_exec_cmd_15() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Num(3),
        Swap,
        Cmds(vec![Mul, Num(2)]),
        Swap,
        Cmds(vec![Sub, Swap, Num(1)]),
        Cmds(vec![Exec, Swap, Exec, Swap]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_15() {
    assert_eq!(
        prog_16_exec_cmd_15().exec(false),
        test_exec_16_exec_cmd_15_res()
    )
}
fn test_exec_16_exec_cmd_15_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_03_sub_cmd_018() {
    let src = fs::read_to_string("./assets/03_sub_cmd_018.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_018_res())
}
fn test_parse_03_sub_cmd_018_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_018())
}
fn prog_03_sub_cmd_018() -> Prog {
    Prog(vec![Sub, Num(-3), Num(10)])
}
#[test]
fn test_exec_03_sub_cmd_018() {
    assert_eq!(
        prog_03_sub_cmd_018().exec(false),
        test_exec_03_sub_cmd_018_res()
    )
}
fn test_exec_03_sub_cmd_018_res() -> Result<i64, ProgExecError> {
    Ok(-13)
}

#[test]
fn test_parse_12_pop_cmd_04() {
    let src = fs::read_to_string("./assets/12_pop_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_04_res())
}
fn test_parse_12_pop_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_04())
}
fn prog_12_pop_cmd_04() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_04() {
    assert_eq!(
        prog_12_pop_cmd_04().exec(false),
        test_exec_12_pop_cmd_04_res()
    )
}
fn test_exec_12_pop_cmd_04_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_03_sub_cmd_006() {
    let src = fs::read_to_string("./assets/03_sub_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_006_res())
}
fn test_parse_03_sub_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_006())
}
fn prog_03_sub_cmd_006() -> Prog {
    Prog(vec![Sub, Num(-10), Num(1)])
}
#[test]
fn test_exec_03_sub_cmd_006() {
    assert_eq!(
        prog_03_sub_cmd_006().exec(false),
        test_exec_03_sub_cmd_006_res()
    )
}
fn test_exec_03_sub_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(-11)
}

#[test]
fn test_parse_13_swap_cmd_04() {
    let src = fs::read_to_string("./assets/13_swap_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_04_res())
}
fn test_parse_13_swap_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_04())
}
fn prog_13_swap_cmd_04() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_04() {
    assert_eq!(
        prog_13_swap_cmd_04().exec(false),
        test_exec_13_swap_cmd_04_res()
    )
}
fn test_exec_13_swap_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_20_misc_41() {
    let src = fs::read_to_string("./assets/20_misc_41.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_41_res())
}
fn test_parse_20_misc_41_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_41())
}
fn prog_20_misc_41() -> Prog {
    Prog(vec![Rem, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_41() {
    assert_eq!(prog_20_misc_41().exec(false), test_exec_20_misc_41_res())
}
fn test_exec_20_misc_41_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_08_lt_cmd_019() {
    let src = fs::read_to_string("./assets/08_lt_cmd_019.bir").unwrap();
    assert_eq!(src.parse(), test_parse_08_lt_cmd_019_res())
}
fn test_parse_08_lt_cmd_019_res() -> Result<Prog, ProgParseError> {
    Ok(prog_08_lt_cmd_019())
}
fn prog_08_lt_cmd_019() -> Prog {
    Prog(vec![Lt, Num(2147483647)])
}
#[test]
fn test_exec_08_lt_cmd_019() {
    assert_eq!(
        prog_08_lt_cmd_019().exec(false),
        test_exec_08_lt_cmd_019_res()
    )
}
fn test_exec_08_lt_cmd_019_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_60_fib() {
    let src = fs::read_to_string("./assets/60_fib.bir").unwrap();
    assert_eq!(src.parse(), test_parse_60_fib_res())
}
fn test_parse_60_fib_res() -> Result<Prog, ProgParseError> {
    Ok(prog_60_fib())
}
fn prog_60_fib() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Rev,
        Cmds(vec![
            Exec,
            Ifz,
            Swap,
            Cmds(vec![Num(1), Pop]),
            Swap,
            Cmds(vec![
                Add,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(2),
                Swap,
                Exec,
                Dup,
                Num(-1),
                Sub,
                Swap,
                Num(1),
                Dup,
                Num(0),
            ]),
            Lt,
            Num(1),
            Dup,
            Num(0),
        ]),
        Rev,
        Num(5),
    ])
}
#[test]
fn test_exec_60_fib() {
    assert_eq!(prog_60_fib().exec(false), test_exec_60_fib_res())
}
fn test_exec_60_fib_res() -> Result<i64, ProgExecError> {
    Ok(8)
}

#[test]
fn test_parse_12_pop_cmd_03() {
    let src = fs::read_to_string("./assets/12_pop_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_12_pop_cmd_03_res())
}
fn test_parse_12_pop_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_12_pop_cmd_03())
}
fn prog_12_pop_cmd_03() -> Prog {
    Prog(vec![
        Pop,
        Pop,
        Pop,
        Pop,
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_12_pop_cmd_03() {
    assert_eq!(
        prog_12_pop_cmd_03().exec(false),
        test_exec_12_pop_cmd_03_res()
    )
}
fn test_exec_12_pop_cmd_03_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_11() {
    let src = fs::read_to_string("./assets/14_rev_cmd_11.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_11_res())
}
fn test_parse_14_rev_cmd_11_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_11())
}
fn prog_14_rev_cmd_11() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_11() {
    assert_eq!(
        prog_14_rev_cmd_11().exec(false),
        test_exec_14_rev_cmd_11_res()
    )
}
fn test_exec_14_rev_cmd_11_res() -> Result<i64, ProgExecError> {
    Ok(-4)
}

#[test]
fn test_parse_04_mul_cmd_006() {
    let src = fs::read_to_string("./assets/04_mul_cmd_006.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_006_res())
}
fn test_parse_04_mul_cmd_006_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_006())
}
fn prog_04_mul_cmd_006() -> Prog {
    Prog(vec![Mul, Num(2), Num(2)])
}
#[test]
fn test_exec_04_mul_cmd_006() {
    assert_eq!(
        prog_04_mul_cmd_006().exec(false),
        test_exec_04_mul_cmd_006_res()
    )
}
fn test_exec_04_mul_cmd_006_res() -> Result<i64, ProgExecError> {
    Ok(4)
}

#[test]
fn test_parse_20_misc_85() {
    let src = fs::read_to_string("./assets/20_misc_85.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_85_res())
}
fn test_parse_20_misc_85_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_85())
}
fn prog_20_misc_85() -> Prog {
    Prog(vec![Ifz, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_85() {
    assert_eq!(prog_20_misc_85().exec(false), test_exec_20_misc_85_res())
}
fn test_exec_20_misc_85_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_07() {
    let src = fs::read_to_string("./assets/16_exec_cmd_07.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_07_res())
}
fn test_parse_16_exec_cmd_07_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_07())
}
fn prog_16_exec_cmd_07() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Swap,
        Cmds(vec![]),
        Swap,
        Cmds(vec![Sub, Num(0)]),
        Gt,
        Num(0),
        Dup,
        Num(0),
        Num(-7),
    ])
}
#[test]
fn test_exec_16_exec_cmd_07() {
    assert_eq!(
        prog_16_exec_cmd_07().exec(false),
        test_exec_16_exec_cmd_07_res()
    )
}
fn test_exec_16_exec_cmd_07_res() -> Result<i64, ProgExecError> {
    Ok(7)
}

#[test]
fn test_parse_05_div_cmd_003() {
    let src = fs::read_to_string("./assets/05_div_cmd_003.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_003_res())
}
fn test_parse_05_div_cmd_003_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_003())
}
fn prog_05_div_cmd_003() -> Prog {
    Prog(vec![Div, Num(3), Num(0)])
}
#[test]
fn test_exec_05_div_cmd_003() {
    assert_eq!(
        prog_05_div_cmd_003().exec(false),
        test_exec_05_div_cmd_003_res()
    )
}
fn test_exec_05_div_cmd_003_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_000() {
    let src = fs::read_to_string("./assets/04_mul_cmd_000.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_000_res())
}
fn test_parse_04_mul_cmd_000_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_000())
}
fn prog_04_mul_cmd_000() -> Prog {
    Prog(vec![Mul, Num(-2147483646), Num(-2147483647)])
}
#[test]
fn test_exec_04_mul_cmd_000() {
    assert_eq!(
        prog_04_mul_cmd_000().exec(false),
        test_exec_04_mul_cmd_000_res()
    )
}
fn test_exec_04_mul_cmd_000_res() -> Result<i64, ProgExecError> {
    Ok(4611686011984936962)
}

#[test]
fn test_parse_16_exec_cmd_19() {
    let src = fs::read_to_string("./assets/16_exec_cmd_19.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_19_res())
}
fn test_parse_16_exec_cmd_19_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_19())
}
fn prog_16_exec_cmd_19() -> Prog {
    Prog(vec![
        Exec,
        Exec,
        Swap,
        Cmds(vec![Sub, Swap, Num(5)]),
        Cmds(vec![
            Swap,
            Cmds(vec![Exec, Swap, Mul, Num(2)]),
            Cmds(vec![Exec, Swap, Dup, Num(2)]),
        ]),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_19() {
    assert_eq!(
        prog_16_exec_cmd_19().exec(false),
        test_exec_16_exec_cmd_19_res()
    )
}
fn test_exec_16_exec_cmd_19_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_09_gt_cmd_005() {
    let src = fs::read_to_string("./assets/09_gt_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_09_gt_cmd_005_res())
}
fn test_parse_09_gt_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_09_gt_cmd_005())
}
fn prog_09_gt_cmd_005() -> Prog {
    Prog(vec![Gt, Num(-1), Num(2147483647)])
}
#[test]
fn test_exec_09_gt_cmd_005() {
    assert_eq!(
        prog_09_gt_cmd_005().exec(false),
        test_exec_09_gt_cmd_005_res()
    )
}
fn test_exec_09_gt_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_00_example_01() {
    let src = fs::read_to_string("./assets/00_example_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_01_res())
}
fn test_parse_00_example_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_01())
}
fn prog_00_example_01() -> Prog {
    Prog(vec![Add, Num(2), Num(2)])
}
#[test]
fn test_exec_00_example_01() {
    assert_eq!(
        prog_00_example_01().exec(false),
        test_exec_00_example_01_res()
    )
}
fn test_exec_00_example_01_res() -> Result<i64, ProgExecError> {
    Ok(4)
}

#[test]
fn test_parse_10_ifz_cmd_05() {
    let src = fs::read_to_string("./assets/10_ifz_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_10_ifz_cmd_05_res())
}
fn test_parse_10_ifz_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_10_ifz_cmd_05())
}
fn prog_10_ifz_cmd_05() -> Prog {
    Prog(vec![Ifz, Num(0)])
}
#[test]
fn test_exec_10_ifz_cmd_05() {
    assert_eq!(
        prog_10_ifz_cmd_05().exec(false),
        test_exec_10_ifz_cmd_05_res()
    )
}
fn test_exec_10_ifz_cmd_05_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_25() {
    let src = fs::read_to_string("./assets/20_misc_25.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_25_res())
}
fn test_parse_20_misc_25_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_25())
}
fn prog_20_misc_25() -> Prog {
    Prog(vec![Mul, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_25() {
    assert_eq!(prog_20_misc_25().exec(false), test_exec_20_misc_25_res())
}
fn test_exec_20_misc_25_res() -> Result<i64, ProgExecError> {
    Ok(2)
}

#[test]
fn test_parse_03_sub_cmd_016() {
    let src = fs::read_to_string("./assets/03_sub_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_016_res())
}
fn test_parse_03_sub_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_016())
}
fn prog_03_sub_cmd_016() -> Prog {
    Prog(vec![Sub, Num(3), Num(-5)])
}
#[test]
fn test_exec_03_sub_cmd_016() {
    assert_eq!(
        prog_03_sub_cmd_016().exec(false),
        test_exec_03_sub_cmd_016_res()
    )
}
fn test_exec_03_sub_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(8)
}

#[test]
fn test_parse_13_swap_cmd_09() {
    let src = fs::read_to_string("./assets/13_swap_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_09_res())
}
fn test_parse_13_swap_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_09())
}
fn prog_13_swap_cmd_09() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_09() {
    assert_eq!(
        prog_13_swap_cmd_09().exec(false),
        test_exec_13_swap_cmd_09_res()
    )
}
fn test_exec_13_swap_cmd_09_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_01() {
    let src = fs::read_to_string("./assets/20_misc_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_01_res())
}
fn test_parse_20_misc_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_01())
}
fn prog_20_misc_01() -> Prog {
    Prog(vec![Add, Num(1)])
}
#[test]
fn test_exec_20_misc_01() {
    assert_eq!(prog_20_misc_01().exec(false), test_exec_20_misc_01_res())
}
fn test_exec_20_misc_01_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_008() {
    let src = fs::read_to_string("./assets/05_div_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_008_res())
}
fn test_parse_05_div_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_008())
}
fn prog_05_div_cmd_008() -> Prog {
    Prog(vec![Div, Num(2), Num(2147483648)])
}
#[test]
fn test_exec_05_div_cmd_008() {
    assert_eq!(
        prog_05_div_cmd_008().exec(false),
        test_exec_05_div_cmd_008_res()
    )
}
fn test_exec_05_div_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_15_cmds_cmd_00() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_00_res())
}
fn test_parse_15_cmds_cmd_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_00())
}
fn prog_15_cmds_cmd_00() -> Prog {
    Prog(vec![Num(3), Num(2), Num(1), Cmds(vec![])])
}
#[test]
fn test_exec_15_cmds_cmd_00() {
    assert_eq!(
        prog_15_cmds_cmd_00().exec(false),
        test_exec_15_cmds_cmd_00_res()
    )
}
fn test_exec_15_cmds_cmd_00_res() -> Result<i64, ProgExecError> {
    Ok(3)
}

#[test]
fn test_parse_02_add_cmd_011() {
    let src = fs::read_to_string("./assets/02_add_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_011_res())
}
fn test_parse_02_add_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_011())
}
fn prog_02_add_cmd_011() -> Prog {
    Prog(vec![Add, Num(-2147483647), Num(2147483648)])
}
#[test]
fn test_exec_02_add_cmd_011() {
    assert_eq!(
        prog_02_add_cmd_011().exec(false),
        test_exec_02_add_cmd_011_res()
    )
}
fn test_exec_02_add_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_02_add_cmd_008() {
    let src = fs::read_to_string("./assets/02_add_cmd_008.bir").unwrap();
    assert_eq!(src.parse(), test_parse_02_add_cmd_008_res())
}
fn test_parse_02_add_cmd_008_res() -> Result<Prog, ProgParseError> {
    Ok(prog_02_add_cmd_008())
}
fn prog_02_add_cmd_008() -> Prog {
    Prog(vec![Add, Num(-2147483648), Num(-2)])
}
#[test]
fn test_exec_02_add_cmd_008() {
    assert_eq!(
        prog_02_add_cmd_008().exec(false),
        test_exec_02_add_cmd_008_res()
    )
}
fn test_exec_02_add_cmd_008_res() -> Result<i64, ProgExecError> {
    Ok(-2147483650)
}

#[test]
fn test_parse_01_num_cmd_01() {
    let src = fs::read_to_string("./assets/01_num_cmd_01.bir").unwrap();
    assert_eq!(src.parse(), test_parse_01_num_cmd_01_res())
}
fn test_parse_01_num_cmd_01_res() -> Result<Prog, ProgParseError> {
    Ok(prog_01_num_cmd_01())
}
fn prog_01_num_cmd_01() -> Prog {
    Prog(vec![Num(-42)])
}
#[test]
fn test_exec_01_num_cmd_01() {
    assert_eq!(
        prog_01_num_cmd_01().exec(false),
        test_exec_01_num_cmd_01_res()
    )
}
fn test_exec_01_num_cmd_01_res() -> Result<i64, ProgExecError> {
    Ok(-42)
}

#[test]
fn test_parse_05_div_cmd_009() {
    let src = fs::read_to_string("./assets/05_div_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_009_res())
}
fn test_parse_05_div_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_009())
}
fn prog_05_div_cmd_009() -> Prog {
    Prog(vec![Div, Num(2147483648), Num(1)])
}
#[test]
fn test_exec_05_div_cmd_009() {
    assert_eq!(
        prog_05_div_cmd_009().exec(false),
        test_exec_05_div_cmd_009_res()
    )
}
fn test_exec_05_div_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_16_exec_cmd_20() {
    let src = fs::read_to_string("./assets/16_exec_cmd_20.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_20_res())
}
fn test_parse_16_exec_cmd_20_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_20())
}
fn prog_16_exec_cmd_20() -> Prog {
    Prog(vec![Exec, Num(42)])
}
#[test]
fn test_exec_16_exec_cmd_20() {
    assert_eq!(
        prog_16_exec_cmd_20().exec(false),
        test_exec_16_exec_cmd_20_res()
    )
}
fn test_exec_16_exec_cmd_20_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_005() {
    let src = fs::read_to_string("./assets/05_div_cmd_005.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_005_res())
}
fn test_parse_05_div_cmd_005_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_005())
}
fn prog_05_div_cmd_005() -> Prog {
    Prog(vec![Div, Num(-2147483646), Num(-10)])
}
#[test]
fn test_exec_05_div_cmd_005() {
    assert_eq!(
        prog_05_div_cmd_005().exec(false),
        test_exec_05_div_cmd_005_res()
    )
}
fn test_exec_05_div_cmd_005_res() -> Result<i64, ProgExecError> {
    Ok(214748364)
}

#[test]
fn test_parse_20_misc_00() {
    let src = fs::read_to_string("./assets/20_misc_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_00_res())
}
fn test_parse_20_misc_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_00())
}
fn prog_20_misc_00() -> Prog {
    Prog(vec![Add])
}
#[test]
fn test_exec_20_misc_00() {
    assert_eq!(prog_20_misc_00().exec(false), test_exec_20_misc_00_res())
}
fn test_exec_20_misc_00_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_20_misc_65() {
    let src = fs::read_to_string("./assets/20_misc_65.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_65_res())
}
fn test_parse_20_misc_65_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_65())
}
fn prog_20_misc_65() -> Prog {
    Prog(vec![Gt, Cmds(vec![Num(1)])])
}
#[test]
fn test_exec_20_misc_65() {
    assert_eq!(prog_20_misc_65().exec(false), test_exec_20_misc_65_res())
}
fn test_exec_20_misc_65_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_09() {
    let src = fs::read_to_string("./assets/14_rev_cmd_09.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_09_res())
}
fn test_parse_14_rev_cmd_09_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_09())
}
fn prog_14_rev_cmd_09() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_09() {
    assert_eq!(
        prog_14_rev_cmd_09().exec(false),
        test_exec_14_rev_cmd_09_res()
    )
}
fn test_exec_14_rev_cmd_09_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_15_cmds_cmd_03() {
    let src = fs::read_to_string("./assets/15_cmds_cmd_03.bir").unwrap();
    assert_eq!(src.parse(), test_parse_15_cmds_cmd_03_res())
}
fn test_parse_15_cmds_cmd_03_res() -> Result<Prog, ProgParseError> {
    Ok(prog_15_cmds_cmd_03())
}
fn prog_15_cmds_cmd_03() -> Prog {
    Prog(vec![Cmds(vec![Num(3), Num(2), Num(1)])])
}
#[test]
fn test_exec_15_cmds_cmd_03() {
    assert_eq!(
        prog_15_cmds_cmd_03().exec(false),
        test_exec_15_cmds_cmd_03_res()
    )
}
fn test_exec_15_cmds_cmd_03_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_14_rev_cmd_06() {
    let src = fs::read_to_string("./assets/14_rev_cmd_06.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_06_res())
}
fn test_parse_14_rev_cmd_06_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_06())
}
fn prog_14_rev_cmd_06() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_06() {
    assert_eq!(
        prog_14_rev_cmd_06().exec(false),
        test_exec_14_rev_cmd_06_res()
    )
}
fn test_exec_14_rev_cmd_06_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_13_swap_cmd_02() {
    let src = fs::read_to_string("./assets/13_swap_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_13_swap_cmd_02_res())
}
fn test_parse_13_swap_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_13_swap_cmd_02())
}
fn prog_13_swap_cmd_02() -> Prog {
    Prog(vec![
        Swap,
        Pop,
        Swap,
        Pop,
        Swap,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_13_swap_cmd_02() {
    assert_eq!(
        prog_13_swap_cmd_02().exec(false),
        test_exec_13_swap_cmd_02_res()
    )
}
fn test_exec_13_swap_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-6)
}

#[test]
fn test_parse_20_misc_88() {
    let src = fs::read_to_string("./assets/20_misc_88.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_88_res())
}
fn test_parse_20_misc_88_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_88())
}
fn prog_20_misc_88() -> Prog {
    Prog(vec![Ifz, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_88() {
    assert_eq!(prog_20_misc_88().exec(false), test_exec_20_misc_88_res())
}
fn test_exec_20_misc_88_res() -> Result<i64, ProgExecError> {
    Ok(-99)
}

#[test]
fn test_parse_20_misc_90() {
    let src = fs::read_to_string("./assets/20_misc_90.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_90_res())
}
fn test_parse_20_misc_90_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_90())
}
fn prog_20_misc_90() -> Prog {
    Prog(vec![Dup])
}
#[test]
fn test_exec_20_misc_90() {
    assert_eq!(prog_20_misc_90().exec(false), test_exec_20_misc_90_res())
}
fn test_exec_20_misc_90_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_21() {
    let src = fs::read_to_string("./assets/16_exec_cmd_21.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_21_res())
}
fn test_parse_16_exec_cmd_21_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_21())
}
fn prog_16_exec_cmd_21() -> Prog {
    Prog(vec![
        Swap,
        Num(3),
        Cmds(vec![Exec, Cmds(vec![Mul, Num(2)]), Num(5)]),
    ])
}
#[test]
fn test_exec_16_exec_cmd_21() {
    assert_eq!(
        prog_16_exec_cmd_21().exec(false),
        test_exec_16_exec_cmd_21_res()
    )
}
fn test_exec_16_exec_cmd_21_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_05_div_cmd_020() {
    let src = fs::read_to_string("./assets/05_div_cmd_020.bir").unwrap();
    assert_eq!(src.parse(), test_parse_05_div_cmd_020_res())
}
fn test_parse_05_div_cmd_020_res() -> Result<Prog, ProgParseError> {
    Ok(prog_05_div_cmd_020())
}
fn prog_05_div_cmd_020() -> Prog {
    Prog(vec![Div])
}
#[test]
fn test_exec_05_div_cmd_020() {
    assert_eq!(
        prog_05_div_cmd_020().exec(false),
        test_exec_05_div_cmd_020_res()
    )
}
fn test_exec_05_div_cmd_020_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_14() {
    let src = fs::read_to_string("./assets/16_exec_cmd_14.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_14_res())
}
fn test_parse_16_exec_cmd_14_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_14())
}
fn prog_16_exec_cmd_14() -> Prog {
    Prog(vec![
        Exec,
        Ifz,
        Lt,
        Num(6),
        Num(5),
        Cmds(vec![Add, Num(3), Num(2)]),
        Num(4),
    ])
}
#[test]
fn test_exec_16_exec_cmd_14() {
    assert_eq!(
        prog_16_exec_cmd_14().exec(false),
        test_exec_16_exec_cmd_14_res()
    )
}
fn test_exec_16_exec_cmd_14_res() -> Result<i64, ProgExecError> {
    Ok(5)
}

#[test]
fn test_parse_07_eq_cmd_015() {
    let src = fs::read_to_string("./assets/07_eq_cmd_015.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_015_res())
}
fn test_parse_07_eq_cmd_015_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_015())
}
fn prog_07_eq_cmd_015() -> Prog {
    Prog(vec![Eq, Num(-2147483646), Num(2147483646)])
}
#[test]
fn test_exec_07_eq_cmd_015() {
    assert_eq!(
        prog_07_eq_cmd_015().exec(false),
        test_exec_07_eq_cmd_015_res()
    )
}
fn test_exec_07_eq_cmd_015_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_70() {
    let src = fs::read_to_string("./assets/20_misc_70.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_70_res())
}
fn test_parse_20_misc_70_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_70())
}
fn prog_20_misc_70() -> Prog {
    Prog(vec![Gt, Num(1), Num(2), Num(-99)])
}
#[test]
fn test_exec_20_misc_70() {
    assert_eq!(prog_20_misc_70().exec(false), test_exec_20_misc_70_res())
}
fn test_exec_20_misc_70_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_20_misc_36() {
    let src = fs::read_to_string("./assets/20_misc_36.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_36_res())
}
fn test_parse_20_misc_36_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_36())
}
fn prog_20_misc_36() -> Prog {
    Prog(vec![Rem])
}
#[test]
fn test_exec_20_misc_36() {
    assert_eq!(prog_20_misc_36().exec(false), test_exec_20_misc_36_res())
}
fn test_exec_20_misc_36_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_04_mul_cmd_011() {
    let src = fs::read_to_string("./assets/04_mul_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_011_res())
}
fn test_parse_04_mul_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_011())
}
fn prog_04_mul_cmd_011() -> Prog {
    Prog(vec![Mul, Num(-2147483647), Num(-1)])
}
#[test]
fn test_exec_04_mul_cmd_011() {
    assert_eq!(
        prog_04_mul_cmd_011().exec(false),
        test_exec_04_mul_cmd_011_res()
    )
}
fn test_exec_04_mul_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(2147483647)
}

#[test]
fn test_parse_00_example_00() {
    let src = fs::read_to_string("./assets/00_example_00.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_00_res())
}
fn test_parse_00_example_00_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_00())
}
fn prog_00_example_00() -> Prog {
    Prog(vec![Num(1), Num(2), Num(3)])
}
#[test]
fn test_exec_00_example_00() {
    assert_eq!(
        prog_00_example_00().exec(false),
        test_exec_00_example_00_res()
    )
}
fn test_exec_00_example_00_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_14_rev_cmd_02() {
    let src = fs::read_to_string("./assets/14_rev_cmd_02.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_02_res())
}
fn test_parse_14_rev_cmd_02_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_02())
}
fn prog_14_rev_cmd_02() -> Prog {
    Prog(vec![
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_02() {
    assert_eq!(
        prog_14_rev_cmd_02().exec(false),
        test_exec_14_rev_cmd_02_res()
    )
}
fn test_exec_14_rev_cmd_02_res() -> Result<i64, ProgExecError> {
    Ok(-2)
}

#[test]
fn test_parse_20_misc_69() {
    let src = fs::read_to_string("./assets/20_misc_69.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_69_res())
}
fn test_parse_20_misc_69_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_69())
}
fn prog_20_misc_69() -> Prog {
    Prog(vec![Gt, Cmds(vec![Num(1)]), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_69() {
    assert_eq!(prog_20_misc_69().exec(false), test_exec_20_misc_69_res())
}
fn test_exec_20_misc_69_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_16_exec_cmd_04() {
    let src = fs::read_to_string("./assets/16_exec_cmd_04.bir").unwrap();
    assert_eq!(src.parse(), test_parse_16_exec_cmd_04_res())
}
fn test_parse_16_exec_cmd_04_res() -> Result<Prog, ProgParseError> {
    Ok(prog_16_exec_cmd_04())
}
fn prog_16_exec_cmd_04() -> Prog {
    Prog(vec![
        Exec,
        Swap,
        Exec,
        Swap,
        Dup,
        Num(3),
        Cmds(vec![Mul, Dup, Num(0)]),
        Cmds(vec![Sub, Swap, Mul]),
        Num(-10),
        Num(2),
    ])
}
#[test]
fn test_exec_16_exec_cmd_04() {
    assert_eq!(
        prog_16_exec_cmd_04().exec(false),
        test_exec_16_exec_cmd_04_res()
    )
}
fn test_exec_16_exec_cmd_04_res() -> Result<i64, ProgExecError> {
    Ok(42)
}

#[test]
fn test_parse_20_misc_59() {
    let src = fs::read_to_string("./assets/20_misc_59.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_59_res())
}
fn test_parse_20_misc_59_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_59())
}
fn prog_20_misc_59() -> Prog {
    Prog(vec![Lt, Cmds(vec![Num(1)]), Num(2)])
}
#[test]
fn test_exec_20_misc_59() {
    assert_eq!(prog_20_misc_59().exec(false), test_exec_20_misc_59_res())
}
fn test_exec_20_misc_59_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_00_example_08() {
    let src = fs::read_to_string("./assets/00_example_08.bir").unwrap();
    assert_eq!(src.parse(), test_parse_00_example_08_res())
}
fn test_parse_00_example_08_res() -> Result<Prog, ProgParseError> {
    Ok(prog_00_example_08())
}
fn prog_00_example_08() -> Prog {
    Prog(vec![Dup, Num(3), Num(20), Num(1)])
}
#[test]
fn test_exec_00_example_08() {
    assert_eq!(
        prog_00_example_08().exec(false),
        test_exec_00_example_08_res()
    )
}
fn test_exec_00_example_08_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_82_tak() {
    let src = fs::read_to_string("./assets/82_tak.bir").unwrap();
    assert_eq!(src.parse(), test_parse_82_tak_res())
}
fn test_parse_82_tak_res() -> Result<Prog, ProgParseError> {
    Ok(prog_82_tak())
}
fn prog_82_tak() -> Prog {
    Prog(vec![
        Exec,
        Dup,
        Num(-1),
        Num(24),
        Num(16),
        Num(8),
        Cmds(vec![
            Exec,
            Ifz,
            Sub,
            Num(1),
            Lt,
            Dup,
            Num(4),
            Dup,
            Num(2),
            Cmds(vec![
                Pop,
                Swap,
                Pop,
                Swap,
                Pop,
                Swap,
                Exec,
                Dup,
                Num(-1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(4),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(4),
                Dup,
                Num(1),
                Exec,
                Dup,
                Num(-1),
                Sub,
                Dup,
                Num(5),
                Num(1),
                Dup,
                Num(1),
                Dup,
                Num(1),
            ]),
            Cmds(vec![Pop, Pop]),
        ]),
    ])
}
#[test]
fn test_exec_82_tak() {
    assert_eq!(prog_82_tak().exec(false), test_exec_82_tak_res())
}
fn test_exec_82_tak_res() -> Result<i64, ProgExecError> {
    Ok(9)
}

#[test]
fn test_parse_04_mul_cmd_016() {
    let src = fs::read_to_string("./assets/04_mul_cmd_016.bir").unwrap();
    assert_eq!(src.parse(), test_parse_04_mul_cmd_016_res())
}
fn test_parse_04_mul_cmd_016_res() -> Result<Prog, ProgParseError> {
    Ok(prog_04_mul_cmd_016())
}
fn prog_04_mul_cmd_016() -> Prog {
    Prog(vec![Mul, Num(2147483648), Num(1)])
}
#[test]
fn test_exec_04_mul_cmd_016() {
    assert_eq!(
        prog_04_mul_cmd_016().exec(false),
        test_exec_04_mul_cmd_016_res()
    )
}
fn test_exec_04_mul_cmd_016_res() -> Result<i64, ProgExecError> {
    Ok(2147483648)
}

#[test]
fn test_parse_06_rem_cmd_004() {
    let src = fs::read_to_string("./assets/06_rem_cmd_004.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_004_res())
}
fn test_parse_06_rem_cmd_004_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_004())
}
fn prog_06_rem_cmd_004() -> Prog {
    Prog(vec![Rem, Num(-5), Num(-10)])
}
#[test]
fn test_exec_06_rem_cmd_004() {
    assert_eq!(
        prog_06_rem_cmd_004().exec(false),
        test_exec_06_rem_cmd_004_res()
    )
}
fn test_exec_06_rem_cmd_004_res() -> Result<i64, ProgExecError> {
    Ok(-5)
}

#[test]
fn test_parse_20_misc_40() {
    let src = fs::read_to_string("./assets/20_misc_40.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_40_res())
}
fn test_parse_20_misc_40_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_40())
}
fn prog_20_misc_40() -> Prog {
    Prog(vec![Rem, Num(1), Cmds(vec![Num(2)])])
}
#[test]
fn test_exec_20_misc_40() {
    assert_eq!(prog_20_misc_40().exec(false), test_exec_20_misc_40_res())
}
fn test_exec_20_misc_40_res() -> Result<i64, ProgExecError> {
    Err(ProgExecError)
}

#[test]
fn test_parse_06_rem_cmd_010() {
    let src = fs::read_to_string("./assets/06_rem_cmd_010.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_010_res())
}
fn test_parse_06_rem_cmd_010_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_010())
}
fn prog_06_rem_cmd_010() -> Prog {
    Prog(vec![Rem, Num(-10), Num(-2147483646)])
}
#[test]
fn test_exec_06_rem_cmd_010() {
    assert_eq!(
        prog_06_rem_cmd_010().exec(false),
        test_exec_06_rem_cmd_010_res()
    )
}
fn test_exec_06_rem_cmd_010_res() -> Result<i64, ProgExecError> {
    Ok(-10)
}

#[test]
fn test_parse_06_rem_cmd_011() {
    let src = fs::read_to_string("./assets/06_rem_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_06_rem_cmd_011_res())
}
fn test_parse_06_rem_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_06_rem_cmd_011())
}
fn prog_06_rem_cmd_011() -> Prog {
    Prog(vec![Rem, Num(5), Num(2)])
}
#[test]
fn test_exec_06_rem_cmd_011() {
    assert_eq!(
        prog_06_rem_cmd_011().exec(false),
        test_exec_06_rem_cmd_011_res()
    )
}
fn test_exec_06_rem_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(1)
}

#[test]
fn test_parse_03_sub_cmd_009() {
    let src = fs::read_to_string("./assets/03_sub_cmd_009.bir").unwrap();
    assert_eq!(src.parse(), test_parse_03_sub_cmd_009_res())
}
fn test_parse_03_sub_cmd_009_res() -> Result<Prog, ProgParseError> {
    Ok(prog_03_sub_cmd_009())
}
fn prog_03_sub_cmd_009() -> Prog {
    Prog(vec![Sub, Num(10), Num(0)])
}
#[test]
fn test_exec_03_sub_cmd_009() {
    assert_eq!(
        prog_03_sub_cmd_009().exec(false),
        test_exec_03_sub_cmd_009_res()
    )
}
fn test_exec_03_sub_cmd_009_res() -> Result<i64, ProgExecError> {
    Ok(10)
}

#[test]
fn test_parse_20_misc_12() {
    let src = fs::read_to_string("./assets/20_misc_12.bir").unwrap();
    assert_eq!(src.parse(), test_parse_20_misc_12_res())
}
fn test_parse_20_misc_12_res() -> Result<Prog, ProgParseError> {
    Ok(prog_20_misc_12())
}
fn prog_20_misc_12() -> Prog {
    Prog(vec![Sub, Num(1), Num(2)])
}
#[test]
fn test_exec_20_misc_12() {
    assert_eq!(prog_20_misc_12().exec(false), test_exec_20_misc_12_res())
}
fn test_exec_20_misc_12_res() -> Result<i64, ProgExecError> {
    Ok(-1)
}

#[test]
fn test_parse_07_eq_cmd_011() {
    let src = fs::read_to_string("./assets/07_eq_cmd_011.bir").unwrap();
    assert_eq!(src.parse(), test_parse_07_eq_cmd_011_res())
}
fn test_parse_07_eq_cmd_011_res() -> Result<Prog, ProgParseError> {
    Ok(prog_07_eq_cmd_011())
}
fn prog_07_eq_cmd_011() -> Prog {
    Prog(vec![Eq, Num(1), Num(-5)])
}
#[test]
fn test_exec_07_eq_cmd_011() {
    assert_eq!(
        prog_07_eq_cmd_011().exec(false),
        test_exec_07_eq_cmd_011_res()
    )
}
fn test_exec_07_eq_cmd_011_res() -> Result<i64, ProgExecError> {
    Ok(0)
}

#[test]
fn test_parse_14_rev_cmd_05() {
    let src = fs::read_to_string("./assets/14_rev_cmd_05.bir").unwrap();
    assert_eq!(src.parse(), test_parse_14_rev_cmd_05_res())
}
fn test_parse_14_rev_cmd_05_res() -> Result<Prog, ProgParseError> {
    Ok(prog_14_rev_cmd_05())
}
fn prog_14_rev_cmd_05() -> Prog {
    Prog(vec![
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Pop,
        Rev,
        Swap,
        Rev,
        Num(9),
        Num(-8),
        Num(7),
        Num(-6),
        Num(5),
        Num(-4),
        Num(3),
        Num(-2),
        Num(1),
    ])
}
#[test]
fn test_exec_14_rev_cmd_05() {
    assert_eq!(
        prog_14_rev_cmd_05().exec(false),
        test_exec_14_rev_cmd_05_res()
    )
}
fn test_exec_14_rev_cmd_05_res() -> Result<i64, ProgExecError> {
    Ok(7)
}
