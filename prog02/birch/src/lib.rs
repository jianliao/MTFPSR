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
            Command::Eq => f.write_str("gt"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgParseError;
impl FromStr for Prog {
    type Err = ProgParseError;
    fn from_str(s: &str) -> Result<Prog, ProgParseError> {
        // Your code here
        unimplemented!()
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
        unimplemented!();
    }
    // Your code here; additional methods as necessary
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
            unimplemented!();
            step += 1;
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
