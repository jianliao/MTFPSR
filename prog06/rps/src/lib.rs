use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

use async_std::prelude::*;
use async_std::{
    future,
    future::TimeoutError,
    io::{BufReader, Lines},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    task,
};

use futures::channel::oneshot;
fn oneshot_send_err<T>(_: T) -> BoxErr {
    #[derive(Debug, thiserror::Error)]
    #[error("{:?}", self)]
    struct OneshotSendError;
    OneshotSendError.into()
}

use log::{debug, error, info, log, Level};

/* ************************************************************************* */
/* ************************************************************************* */

type BoxErr = Box<dyn std::error::Error + Send + Sync>;
type ResultBoxErr<T> = Result<T, BoxErr>;

/* ************************************************************************* */

async fn log_termination_gen<F, E>(fut: F, ok_level: Level, err_level: E)
where
    F: Future<Output = ResultBoxErr<()>>,
    E: Fn(&BoxErr) -> Level,
{
    let task = task::current();
    let name = task.name().unwrap_or("???");
    match fut.await {
        Ok(()) => {
            log!(ok_level, "{}:: Terminated", name)
        }
        Err(err) => {
            log!(
                err_level(&err),
                "{}:: Terminated with error ({})",
                name,
                err
            )
        }
    }
}

async fn log_termination<F>(fut: F)
where
    F: Future<Output = ResultBoxErr<()>>,
{
    log_termination_gen(fut, Level::Info, |err| {
        if err.downcast_ref::<PeerConnectionError>().is_some() {
            Level::Warn
        } else {
            Level::Error
        }
    })
    .await
}

/* ************************************************************************* */

#[derive(Debug)]
struct PeerConnection {
    peer_addr: SocketAddr,
    reader_lines: Lines<BufReader<TcpStream>>,
    writer: TcpStream,
}
#[derive(Debug, thiserror::Error)]
#[error("disconnected while reading during {phase} phase")]
struct ReadDisconnectError {
    phase: &'static str,
}
#[derive(Debug, thiserror::Error)]
#[error("disconnected while writing during {phase} phase")]
struct WriteDisconnectError {
    phase: &'static str,
}
#[derive(Debug, thiserror::Error)]
#[error("{err}")]
struct PeerConnectionError {
    err: BoxErr,
}
impl PeerConnection {
    fn new(stream: TcpStream) -> ResultBoxErr<Self> {
        let peer_addr = stream.peer_addr()?;
        let reader_lines = BufReader::new(stream.clone()).lines();
        let writer = stream;
        Ok(PeerConnection {
            peer_addr,
            writer,
            reader_lines,
        })
    }
    fn error(&self, err: BoxErr) -> BoxErr {
        PeerConnectionError { err }.into()
    }
    fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    async fn next_line(&mut self, phase: &'static str) -> ResultBoxErr<String> {
        Ok(match self.reader_lines.next().await {
            Some(line) => String::from(line?.trim()),
            None => return Err(self.error(ReadDisconnectError { phase }.into())),
        })
    }
    async fn next_line_timeout(
        &mut self,
        phase: &'static str,
        dur: Duration,
    ) -> ResultBoxErr<Result<String, TimeoutError>> {
        Ok(match future::timeout(dur, self.next_line(phase)).await {
            Err(err) => Err(err),
            Ok(res) => Ok(res?),
        })
    }

    // https://github.com/rust-lang/rust/issues/63033
    fn write_str<'a, 'b, 'r>(
        &'a mut self,
        msg: &'b str,
        phase: &'static str,
    ) -> impl Future<Output = ResultBoxErr<()>> + 'r
    where
        'a: 'r,
        'b: 'r,
    {
        async move {
            self.writer
                .write_all(msg.as_bytes())
                .await
                .map_err(|_| self.error(WriteDisconnectError { phase }.into()))
        }
    }
    async fn writeln_str(&mut self, msg: &str, phase: &'static str) -> ResultBoxErr<()> {
        self.write_str(&format!("{}\n", msg), phase).await
    }
    async fn newline(&mut self, phase: &'static str) -> ResultBoxErr<()> {
        self.write_str("\n", phase).await
    }

    async fn prompt_timeout(
        &mut self,
        msg: &str,
        phase: &'static str,
        dur: Duration,
    ) -> ResultBoxErr<Result<String, TimeoutError>> {
        self.write_str(msg, phase).await?;
        self.writeln_str(&format!(" [{:.2}s timeout]:", dur.as_secs_f32()), phase)
            .await?;
        Ok(match self.next_line_timeout(phase, dur).await? {
            Ok(line) => Ok(line),
            Err(err) => {
                let msg = format!("Timeout after {:.2}s", dur.as_secs_f32());
                self.writeln_str(&msg, "command").await?;
                Err(err)
            }
        })
    }
    async fn prompt_timeout_err(
        &mut self,
        msg: &str,
        phase: &'static str,
        dur: Duration,
    ) -> ResultBoxErr<String> {
        self.prompt_timeout(msg, phase, dur)
            .await?
            .map_err(|err| self.error(err.into()))
    }
}

/* ************************************************************************* */
/* ************************************************************************* */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Weapon {
    Rock,
    Paper,
    Scissors,
}
impl Display for Weapon {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Weapon::Rock => "rock",
            Weapon::Paper => "paper",
            Weapon::Scissors => "scissors",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Loss,
    Forfeit,
}
impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Outcome::Win => "win",
            Outcome::Draw => "draw",
            Outcome::Loss => "lose",
            Outcome::Forfeit => "forfeit",
        })
    }
}

fn battle(weapon1: Option<Weapon>, weapon2: Option<Weapon>) -> (Outcome, Outcome) {
    match (weapon1, weapon2) {
        (None, None) => (Outcome::Forfeit, Outcome::Forfeit),
        (Some(_), None) => (Outcome::Win, Outcome::Forfeit),
        (None, Some(_)) => (Outcome::Forfeit, Outcome::Win),
        (Some(Weapon::Rock), Some(Weapon::Paper)) => (Outcome::Loss, Outcome::Win),
        (Some(Weapon::Rock), Some(Weapon::Scissors)) => (Outcome::Win, Outcome::Loss),
        (Some(Weapon::Paper), Some(Weapon::Rock)) => (Outcome::Win, Outcome::Loss),
        (Some(Weapon::Paper), Some(Weapon::Scissors)) => (Outcome::Loss, Outcome::Win),
        (Some(Weapon::Scissors), Some(Weapon::Rock)) => (Outcome::Loss, Outcome::Win),
        (Some(Weapon::Scissors), Some(Weapon::Paper)) => (Outcome::Win, Outcome::Loss),
        (Some(_), Some(_)) => (Outcome::Draw, Outcome::Draw),
    }
}

/* ************************************************************************* */
/* ************************************************************************* */

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Stats {
    wins: usize,
    draws: usize,
    losses: usize,
    forfeits: usize,
}
impl Stats {
    fn new() -> Self {
        Stats {
            wins: 0,
            draws: 0,
            losses: 0,
            forfeits: 0,
        }
    }
    fn inc_by_outcome(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Win => self.wins += 1,
            Outcome::Draw => self.draws += 1,
            Outcome::Loss => self.losses += 1,
            Outcome::Forfeit => self.forfeits += 1,
        }
    }
}
impl Display for Stats {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "wins: {}, draws: {}, losses: {}, forfeits: {}",
            self.wins, self.draws, self.losses, self.forfeits,
        )
    }
}
#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("{:?}", self)]
struct ParseStatsError;
impl FromStr for Stats {
    type Err = ParseStatsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = ParseStatsError;
        let doit = |s: &str, expected: &str| {
            let mut iter = s.split(": ");
            let label = iter.next().ok_or(err)?;
            if label != expected {
                return Err(err);
            }
            let count = iter.next().ok_or(err)?.parse::<usize>().map_err(|_| err)?;
            if iter.next().is_some() {
                return Err(err);
            }
            Ok(count)
        };
        let mut iter = s.split(", ");
        let wins = doit(iter.next().ok_or(err)?, "wins")?;
        let draws = doit(iter.next().ok_or(err)?, "draws")?;
        let losses = doit(iter.next().ok_or(err)?, "losses")?;
        let forfeits = doit(iter.next().ok_or(err)?, "forfeits")?;
        if iter.next().is_some() {
            return Err(err);
        }
        Ok(Stats {
            wins,
            draws,
            losses,
            forfeits,
        })
    }
}

/* ************************************************************************* */
/* ************************************************************************* */

struct UserData {
    passwd: String,
    online: Option<SocketAddr>,
    stats: Stats,
}
impl UserData {
    fn create(passwd: String, peer_addr: SocketAddr) -> Self {
        UserData {
            passwd,
            online: Some(peer_addr),
            stats: Stats::new(),
        }
    }
}

type UserName = String;

/* ************************************************************************* */
/* ************************************************************************* */

struct DataBase {
    db: Arc<Mutex<HashMap<UserName, UserData>>>,
}

impl DataBase {
    fn new() -> Self {
        DataBase {
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for DataBase {
    fn clone(&self) -> Self {
        DataBase {
            db: Arc::clone(&self.db),
        }
    }
}

impl DataBase {
    async fn create_user(&self, uname: &str, password: String, peer_addr: SocketAddr) -> bool {
        match self.db.lock().await.entry(String::from(uname)) {
            hash_map::Entry::Occupied(_) => false,
            hash_map::Entry::Vacant(entry) => {
                let user_data = UserData::create(password, peer_addr);
                entry.insert(user_data);
                true
            }
        }
    }

    async fn login(&self, uname: &str, peer_addr: SocketAddr) -> bool {
        match self.db.lock().await.get_mut(uname) {
            None => panic!("DataBase::login: user not found ({})", uname),
            Some(UserData {
                online: Some(_), ..
            }) => false,
            Some(UserData {
                online: online @ None,
                ..
            }) => {
                *online = Some(peer_addr);
                true
            }
        }
    }
    async fn logout(&self, uname: &str) {
        match self.db.lock().await.get_mut(uname) {
            None => panic!("DataBase::logout: user not found ({})", uname),
            Some(UserData { online: None, .. }) => {
                panic!("DataBase::logout: user not online ({})", uname)
            }
            Some(UserData {
                online: online @ Some(_),
                ..
            }) => *online = None,
        }
    }

    async fn password(&self, uname: &str) -> Option<String> {
        self.db
            .lock()
            .await
            .get(uname)
            .map(|user_data| user_data.passwd.clone())
    }
    async fn set_password(&self, uname: &str, new_passwd: String) {
        match self.db.lock().await.get_mut(uname) {
            None => {
                panic!("DataBase::set_password: user not found ({})", uname)
            }
            Some(UserData { online: None, .. }) => {
                panic!("DataBase::set_password: user not online ({})", uname)
            }
            Some(UserData { passwd, .. }) => *passwd = new_passwd,
        }
    }

    async fn players(&self) -> Vec<String> {
        self.db
            .lock()
            .await
            .iter()
            .filter_map(|(uname, user_data)| match user_data {
                UserData {
                    online: Some(_), ..
                } => Some(uname.clone()),
                _ => None,
            })
            .collect()
    }

    async fn standings(&self) -> Vec<(UserName, Stats)> {
        let mut standings = Vec::new();
        for (uname, user_data) in self.db.lock().await.iter() {
            standings.push((uname.clone(), user_data.stats));
        }
        standings.sort_by(
            |(uname1, stats1), (uname2, stats2)| match stats2.cmp(stats1) {
                Ordering::Equal => uname1.cmp(uname2),
                ord => ord,
            },
        );
        standings
    }

    async fn stats(&self, uname: &str) -> Stats {
        match self.db.lock().await.get(uname) {
            None => panic!("DataBase::stats: user not found ({})", uname),
            Some(user_data) => user_data.stats,
        }
    }
    async fn with_stats<F, R>(&self, uname: &str, mut f: F) -> R
    where
        F: FnMut(&mut Stats) -> R,
    {
        match self.db.lock().await.get_mut(uname) {
            None => panic!("DataBase::with_stats: user not found ({})", uname),
            Some(user_data) => f(&mut user_data.stats),
        }
    }
    #[allow(dead_code)]
    async fn stats_inc_by_outcome(&self, uname: &str, outcome: Outcome) {
        self.with_stats(uname, |stats| stats.inc_by_outcome(outcome))
            .await
    }
}

/* ************************************************************************* */
/* ************************************************************************* */

const HOSTNAME_DEFAULT: &str = "localhost";
const PORT_DEFAULT: u16 = 8203;

const COMMAND_TIMEOUT: SecsArg = SecsArg(Duration::from_secs(60));
const PLAY_TIMEOUT: SecsArg = SecsArg(Duration::from_secs(30));
const WEAPON_TIMEOUT: SecsArg = SecsArg(Duration::from_secs(10));

const LOG_LEVEL_DEFAULT: log::LevelFilter = log::LevelFilter::Error;

#[derive(Debug, Clone, clap::Parser)]
#[clap(about, long_about = None)]
pub struct Cmd {
    #[clap(flatten)]
    pub args: Args,
    #[clap(subcommand)]
    pub subcmd: Option<Subcmd>,
}
#[derive(Debug, Clone, clap::Args)]
pub struct Args {
    /// RPS Game Server hostname
    #[clap(short, long, default_value = HOSTNAME_DEFAULT, global(true))]
    hostname: String,
    /// RPS Game Server port
    #[clap(short, long, default_value_t = PORT_DEFAULT, global(true))]
    port: u16,
    /// Command (and login and password) timeout
    #[clap(long, value_name = "SECS", default_value_t = COMMAND_TIMEOUT, global(true))]
    command_timeout: SecsArg,
    /// Play timeout
    #[clap(long, value_name = "SECS", default_value_t = PLAY_TIMEOUT, global(true))]
    play_timeout: SecsArg,
    /// Weapon timeout
    #[clap(long, value_name = "SECS", default_value_t = WEAPON_TIMEOUT, global(true))]
    weapon_timeout: SecsArg,
    /// Logging level
    #[clap(short, long, value_name = "off|error|warn|info|debug|trace", default_value_t = LOG_LEVEL_DEFAULT, global(true))]
    pub log_level: log::LevelFilter,
}
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Subcmd {
    #[clap(about = concat!(clap::crate_description!(), " Test"), long_about = None)]
    Test(test::Args),
}

#[derive(Debug, Clone)]
struct SecsArg(Duration);
impl From<SecsArg> for Duration {
    fn from(s: SecsArg) -> Self {
        s.0
    }
}
impl Display for SecsArg {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.2}", self.0.as_secs_f32())
    }
}
#[derive(Debug, thiserror::Error)]
enum ParseSecsArgError {
    #[error("{0:?}")]
    ParseFloatError(std::num::ParseFloatError),
    #[error("not >= 0.0")]
    Negative,
}
impl FromStr for SecsArg {
    type Err = ParseSecsArgError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse() {
            Ok(f) => {
                if f >= 0.0 {
                    Ok(SecsArg(Duration::from_secs_f32(f)))
                } else {
                    Err(ParseSecsArgError::Negative)
                }
            }
            Err(err) => Err(ParseSecsArgError::ParseFloatError(err)),
        }
    }
}

/* ************************************************************************* */
/* ************************************************************************* */

pub async fn server(args: Args, ready: Option<oneshot::Sender<()>>) {
    async fn server(args: Args, ready: Option<oneshot::Sender<()>>) -> ResultBoxErr<()> {
        let hostname = args.hostname.as_str();
        let port = args.port;
        let command_timeout = args.command_timeout.into();
        let play_timeout = args.play_timeout.into();
        let weapon_timeout = args.weapon_timeout.into();
        let db = DataBase::new();

        let listener = TcpListener::bind(format!("{}:{}", hostname, port)).await?;
        match ready {
            None => {}
            Some(ready) => {
                let _ = ready.send(());
            }
        };
        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let mut peer_connection = PeerConnection::new(stream)?;
            let peer_addr = peer_connection.peer_addr();
            info!("server:: Accepted {}", peer_addr);
            let _client_handle = {
                let db = db.clone();
                let welcome = async move {
                    welcome(
                        &mut peer_connection,
                        &db,
                        command_timeout,
                        play_timeout,
                        weapon_timeout,
                    )
                    .await
                };
                let welcome = log_termination(welcome);
                task::Builder::new()
                    .name(format!("server:client({})", peer_addr))
                    .spawn(welcome)
            };
        }
        Ok(())
    }

    log_termination(server(args, ready)).await
}

async fn welcome(
    peer_connection: &mut PeerConnection,
    db: &DataBase,
    command_timeout: Duration,
    play_timeout: Duration,
    weapon_timeout: Duration,
) -> ResultBoxErr<()> {
    peer_connection
        .writeln_str("Welcome to ROCK-PAPER-SCISSORS!\n", "welcome")
        .await?;
    match login(peer_connection, db, command_timeout).await? {
        None => Ok(()),
        Some(uname) => {
            let res = command_loop(
                &uname,
                peer_connection,
                db,
                command_timeout,
                play_timeout,
                weapon_timeout,
            )
            .await;
            db.logout(&uname).await;
            res
        }
    }
}

async fn login(
    peer_connection: &mut PeerConnection,
    db: &DataBase,
    command_timeout: Duration,
) -> ResultBoxErr<Option<String>> {
    loop {
        let line = peer_connection
            .prompt_timeout_err("Enter username", "login", command_timeout)
            .await?;
        if line.is_empty()
            || !(line.chars().next().unwrap().is_alphabetic()
                && line.chars().all(char::is_alphanumeric))
        {
            peer_connection
                .writeln_str(&format!("Invalid username (`{}`)", line), "login")
                .await?;
            continue;
        }
        let uname = line;
        match db.password(&uname).await {
            None => {
                peer_connection.newline("login").await?;
                peer_connection
                    .writeln_str("Welcome new user!\n", "login")
                    .await?;
                let init_passwd = password_loop(
                    peer_connection,
                    "initial password",
                    command_timeout,
                    |passwd| {
                        if passwd.is_empty() {
                            Err("Invalid password")
                        } else {
                            Ok(passwd)
                        }
                    },
                )
                .await?;
                if !db
                    .create_user(&uname, init_passwd, peer_connection.peer_addr)
                    .await
                {
                    peer_connection
                        .writeln_str(&format!("User {} already online", uname), "login")
                        .await?;
                    return Ok(None);
                }
            }
            Some(cur_passwd) => {
                password_loop(peer_connection, "password", command_timeout, |passwd| {
                    if cur_passwd == passwd {
                        Ok(())
                    } else {
                        Err("Incorrect password")
                    }
                })
                .await?;
                if !db.login(&uname, peer_connection.peer_addr).await {
                    peer_connection
                        .writeln_str(&format!("User {} already online", uname), "login")
                        .await?;
                    return Ok(None);
                }
            }
        }
        return Ok(Some(uname));
    }
}

async fn password_loop<Chk, T>(
    peer_connection: &mut PeerConnection,
    msg: &str,
    command_timeout: Duration,
    chk: Chk,
) -> ResultBoxErr<T>
where
    Chk: FnMut(String) -> Result<T, &'static str>,
{
    let mut chk = chk;
    loop {
        let line = peer_connection
            .prompt_timeout_err(&format!("Enter {}", msg), "password", command_timeout)
            .await?;
        match chk(line) {
            Ok(res) => return Ok(res),
            Err(msg) => peer_connection.writeln_str(msg, "password").await?,
        }
    }
}

async fn command_loop(
    uname: &str,
    peer_connection: &mut PeerConnection,
    db: &DataBase,
    command_timeout: Duration,
    play_timeout: Duration,
    weapon_timeout: Duration,
) -> ResultBoxErr<()> {
    loop {
        peer_connection.newline("command").await?;
        let line = peer_connection
            .prompt_timeout_err(
                "Enter command {passwd,play,players,standings,stats,quit}",
                "command",
                command_timeout,
            )
            .await?;
        debug!(
            "server:client({};{}):: Command (`{}`)",
            peer_connection.peer_addr(),
            uname,
            line
        );
        match line.as_str() {
            "passwd" => passwd_command(uname, peer_connection, db, command_timeout).await?,
            "play" => play_command(uname, peer_connection, play_timeout, weapon_timeout).await?,
            "players" => players_command(peer_connection, db).await?,
            "standings" => standings_command(peer_connection, db).await?,
            "stats" => stats_command(uname, peer_connection, db).await?,
            "quit" => {
                peer_connection.writeln_str("Goodbye!", "command").await?;
                return Ok(());
            }
            _ => {
                peer_connection
                    .writeln_str(&format!("Invalid command (`{}`)", line), "command")
                    .await?
            }
        }
    }
}

async fn passwd_command(
    uname: &str,
    peer_connection: &mut PeerConnection,
    db: &DataBase,
    command_timeout: Duration,
) -> ResultBoxErr<()> {
    peer_connection.newline("passwd").await?;
    let old_passwd = db.password(uname).await.unwrap();
    password_loop(peer_connection, "old password", command_timeout, |passwd| {
        if old_passwd == passwd {
            Ok(())
        } else {
            Err("Incorrect password")
        }
    })
    .await?;
    let new_passwd = password_loop(peer_connection, "new password", command_timeout, |passwd| {
        if passwd.is_empty() {
            Err("Invalid password")
        } else {
            Ok(passwd)
        }
    })
    .await?;
    db.set_password(uname, new_passwd).await;
    Ok(())
}

async fn players_command(peer_connection: &mut PeerConnection, db: &DataBase) -> ResultBoxErr<()> {
    peer_connection.newline("players").await?;
    let players = db.players().await;
    peer_connection
        .writeln_str("Online users:", "players")
        .await?;
    for player in players.into_iter() {
        peer_connection.writeln_str(&player, "players").await?
    }
    Ok(())
}

async fn stats_command(
    uname: &str,
    peer_connection: &mut PeerConnection,
    db: &DataBase,
) -> ResultBoxErr<()> {
    peer_connection.newline("stats").await?;
    let stats = db.stats(uname).await;
    peer_connection
        .writeln_str(&format!("{}: {}", uname, stats), "stats")
        .await?;
    Ok(())
}

async fn standings_command(
    peer_connection: &mut PeerConnection,
    db: &DataBase,
) -> ResultBoxErr<()> {
    peer_connection.newline("standings").await?;
    let standings = db.standings().await;
    let width = standings.len().to_string().len();
    for (rank, (uname, stats)) in standings.iter().enumerate() {
        peer_connection
            .writeln_str(
                &format!("{0:>1$}. {2}: {3}", rank + 1, width, uname, stats),
                "standings",
            )
            .await?;
    }
    Ok(())
}

/*

Describe the design of the implementation of `play_command`.  Especially comment
on what synchronization primitives are used, how data is communicated or shared
between tasks, and any additional tasks that are spawned.

YOUR ANSWER HERE.

*/

async fn play_command(
    uname: &str,
    peer_connection: &mut PeerConnection,
    play_timeout: Duration,
    weapon_timeout: Duration,
    // Your code here; additional arguments as necessary
) -> ResultBoxErr<()> {
    peer_connection.newline("play").await?;
    peer_connection
        .writeln_str(
            &format!(
                "Waiting for opponent [{:.2}s timeout]...",
                play_timeout.as_secs_f32()
            ),
            "play",
        )
        .await?;
    // Your code here
    unimplemented!()
}

/* ************************************************************************* */
/* ************************************************************************* */

pub mod test;
