use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Num(i64),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Lt,
    Gt,
    Ifz,
    Dup,
    Pop,
    Swap,
    Rev,
    Cmds(Vec<Command>),
    Exec,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Command::Num(n) => n.fmt(f),
            Command::Add => f.write_str("add"),
            Command::Sub => f.write_str("sub"),
            Command::Mul => f.write_str("mul"),
            Command::Div => f.write_str("div"),
            Command::Rem => f.write_str("rem"),
            Command::Eq => f.write_str("eq"),
            Command::Lt => f.write_str("lt"),
            Command::Gt => f.write_str("gt"),
            Command::Ifz => f.write_str("ifz"),
            Command::Dup => f.write_str("dup"),
            Command::Pop => f.write_str("pop"),
            Command::Swap => f.write_str("swap"),
            Command::Rev => f.write_str("rev"),
            Command::Cmds(cmds) => {
                if cmds.is_empty() {
                    f.write_str("[ ]")
                } else {
                    f.write_str("[ ")?;
                    fmt_slice_rev(&cmds[..], f)?;
                    f.write_str(" ]")
                }
            }
            Command::Exec => f.write_str("exec"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prog(Vec<Command>);
impl Display for Prog {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}

fn parse_commands<'a, I>(
    word_iterator: &mut I,
    is_sub_sequence: bool,
) -> Result<Vec<Command>, ProgParseError>
where
    I: Iterator<Item = &'a str>,
{
    let mut cmds: Vec<Command> = vec![];
    loop {
        match word_iterator.next() {
            Some(w) => {
                let cmd: Command = match w {
                    "add" => Command::Add,
                    "sub" => Command::Sub,
                    "mul" => Command::Mul,
                    "div" => Command::Div,
                    "rem" => Command::Rem,
                    "eq" => Command::Eq,
                    "lt" => Command::Lt,
                    "gt" => Command::Gt,
                    "ifz" => Command::Ifz,
                    "dup" => Command::Dup,
                    "pop" => Command::Pop,
                    "swap" => Command::Swap,
                    "rev" => Command::Rev,
                    "exec" => Command::Exec,
                    "[" => Command::Cmds(match parse_commands(word_iterator, true) {
                        Ok(sub_cmds) => sub_cmds,
                        Err(_) => return Err(ProgParseError),
                    }),
                    "]" => match !is_sub_sequence {
                        true => return Err(ProgParseError),
                        _ => break,
                    },
                    x => match x.parse::<i64>() {
                        Ok(n) => Command::Num(n),
                        Err(_) => return Err(ProgParseError),
                    },
                };
                cmds.push(cmd);
            }
            None => match is_sub_sequence {
                true => return Err(ProgParseError),
                _ => break,
            },
        }
    }
    cmds.reverse();
    Ok(cmds)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgParseError;
impl FromStr for Prog {
    type Err = ProgParseError;
    fn from_str(s: &str) -> Result<Prog, ProgParseError> {
        // Your code here
        Ok(Prog(parse_commands(
            &mut s.split_ascii_whitespace(),
            false,
        )?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgExecError;

#[derive(Debug)]
struct CmdStack(Vec<Command>);
impl Display for CmdStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}
impl CmdStack {
    fn new(prog: &Prog) -> Self {
        // Your code here
        CmdStack(prog.0.to_owned())
    }
    // Your code here; additional methods as necessary
    fn pop(&mut self) -> Option<Command> {
        self.0.pop()
    }
    fn push(&mut self, c: Command) {
        self.0.push(c);
    }
}

#[derive(Debug, Clone)]
enum DataElem {
    Num(i64),
    Cmds(Vec<Command>),
}
impl Display for DataElem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DataElem::Num(n) => n.fmt(f),
            DataElem::Cmds(cmds) => {
                if cmds.is_empty() {
                    f.write_str("[ ]")
                } else {
                    f.write_str("[ ")?;
                    fmt_slice_rev(&cmds[..], f)?;
                    f.write_str(" ]")
                }
            }
        }
    }
}

#[derive(Debug)]
struct DataStack(Vec<DataElem>);
impl Display for DataStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_slice_rev(&self.0[..], f)
    }
}
impl DataStack {
    fn new() -> Self {
        DataStack(Vec::new())
    }
    fn push(&mut self, de: DataElem) {
        self.0.push(de)
    }
    fn pop(&mut self) -> Result<DataElem, ProgExecError> {
        match self.0.pop() {
            Some(de) => Ok(de),
            _ => Err(ProgExecError),
        }
    }
    // Your code here; additional methods as necessary
    fn clone_index_from_stack_bottom(&mut self, i: i64) -> Result<DataElem, ProgExecError> {
        let i: usize = i.abs() as usize;
        if i > self.0.len() - 1 {
            Err(ProgExecError)
        } else {
            Ok(self.0[i].clone())
        }
    }
    fn clone_index_from_stack_top(&mut self, i: i64) -> Result<DataElem, ProgExecError> {
        let index: i64 = self.0.len() as i64 - i - 1;
        if index > self.0.len() as i64 - 1 || index < 0 {
            Err(ProgExecError)
        } else {
            Ok(self.0[index as usize].clone())
        }
    }
    fn reverse(&mut self) {
        self.0.reverse();
    }
}

impl Prog {
    pub fn exec(&self, trace: bool) -> Result<i64, ProgExecError> {
        if trace {
            println!("prog: {}\n", self)
        }

        let mut cstk = CmdStack::new(self);
        let mut dstk = DataStack::new();

        let mut step: u64 = 0;
        loop {
            if trace {
                println!("step: {}\ncstk: {}\ndstk: {}\n", step, cstk, dstk)
            };
            // Your code here
            match cstk.pop() {
                None => break,
                Some(cmd) => match cmd {
                    Command::Ifz => {
                        let e1 = dstk.pop()?;
                        let e2 = dstk.pop()?;
                        let e3 = dstk.pop()?;
                        match e1 {
                            DataElem::Num(i) => {
                                if i == 0 {
                                    dstk.push(e2)
                                } else {
                                    dstk.push(e3)
                                }
                            }
                            DataElem::Cmds(cmds) if cmds.len() == 0 => {
                                // Empty sub sequence means none zero ???
                                dstk.push(e3)
                            }
                            DataElem::Cmds(cmds) => {
                                dstk.push(e3);
                                dstk.push(e2);
                                cstk.push(Command::Ifz);
                                for cmd in cmds {
                                    cstk.push(cmd);
                                }
                            }
                        }
                    }
                    Command::Num(i) => dstk.push(DataElem::Num(i)),
                    Command::Gt => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => {
                            if l > r {
                                dstk.push(DataElem::Num(1))
                            } else {
                                dstk.push(DataElem::Num(0))
                            }
                        }
                        _ => return Err(ProgExecError),
                    },
                    Command::Add => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => dstk.push(DataElem::Num(l + r)),
                        _ => return Err(ProgExecError),
                    },
                    Command::Sub => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => dstk.push(DataElem::Num(l - r)),
                        _ => return Err(ProgExecError),
                    },
                    Command::Mul => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => dstk.push(DataElem::Num(l * r)),
                        _ => return Err(ProgExecError),
                    },
                    Command::Div => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) if r != 0 => {
                            dstk.push(DataElem::Num(l / r))
                        }
                        _ => return Err(ProgExecError),
                    },
                    Command::Rem => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) if r != 0 => {
                            dstk.push(DataElem::Num(l % r))
                        }
                        _ => return Err(ProgExecError),
                    },
                    Command::Eq => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => {
                            if l == r {
                                dstk.push(DataElem::Num(1))
                            } else {
                                dstk.push(DataElem::Num(0))
                            }
                        }
                        _ => return Err(ProgExecError),
                    },
                    Command::Lt => match (dstk.pop()?, dstk.pop()?) {
                        (DataElem::Num(l), DataElem::Num(r)) => {
                            if l < r {
                                dstk.push(DataElem::Num(1))
                            } else {
                                dstk.push(DataElem::Num(0))
                            }
                        }
                        _ => return Err(ProgExecError),
                    },
                    Command::Dup => {
                        if let DataElem::Num(n) = dstk.pop()? {
                            if n < 0 {
                                let dup: DataElem = dstk.clone_index_from_stack_bottom(n + 1)?;
                                dstk.push(dup);
                            } else {
                                let dup: DataElem = dstk.clone_index_from_stack_top(n)?;
                                dstk.push(dup);
                            }
                        } else {
                            return Err(ProgExecError);
                        }
                    }
                    Command::Pop => {
                        dstk.pop()?;
                    }
                    Command::Swap => {
                        let n1: DataElem = dstk.pop()?;
                        let n2: DataElem = dstk.pop()?;
                        dstk.push(n1);
                        dstk.push(n2);
                    }
                    Command::Rev => {
                        dstk.reverse();
                    }
                    Command::Cmds(cmds) => {
                        dstk.push(DataElem::Cmds(cmds));
                    }
                    Command::Exec => match dstk.pop()? {
                        DataElem::Num(_) => {
                            return Err(ProgExecError);
                        }
                        DataElem::Cmds(cmds) => {
                            for cmd in cmds {
                                cstk.push(cmd);
                            }
                        }
                    },
                },
            };

            step += 1;
        }

        match dstk.pop() {
            Ok(i) => match i {
                DataElem::Num(i) => Ok(i),
                DataElem::Cmds(_) => return Err(ProgExecError),
            },
            Err(_) => return Err(ProgExecError),
        }
    }
}

/// A (private) helper function to display a slice in "reverse" order with
/// single spaces between elements.  Useful for displaying a `Vec<T>` being used
/// as a stack, so that the top element appears to the left and the bottom
/// element appears to the right.
fn fmt_slice_rev<T>(slc: &[T], f: &mut Formatter) -> fmt::Result
where
    T: Display,
{
    let mut first = true;
    for x in slc.iter().rev() {
        if first {
            first = false;
        } else {
            f.write_str(" ")?;
        };
        x.fmt(f)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
