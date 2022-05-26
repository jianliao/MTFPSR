use std::collections::HashMap;
use std::collections::VecDeque;
use std::hash::Hash;
use std::marker::Sized;

/// A generic puzzle solver using BFS with hashing of states.

/// Trait for puzzles that can be goal using BFS with hashing of states.
pub trait Puzzle {
    /// The type of moves for this puzzle.
    type Move;

    /// Determines whether or not the puzzle state represents a solved puzzle.
    fn is_goal(&self) -> bool;

    /// Enumerates all of the (legal) successor puzzle states of the current
    /// puzzle state, along with the move that leads to that successor puzzle
    /// state.
    ///
    /// The `Self: Sized` trait bound is a technical requirement for a trait
    /// method that requires the type of `Self` to be known at compile time (in
    /// this case, in order to know the size of each element of the result
    /// vector).  Most types in Rust are (and are assumed to be) `Sized`, but
    /// the `Self` type of a trait is an exception to that rule that types are
    /// assumed to be `Sized`.
    fn next(&self) -> Vec<(Self::Move, Self)>
    where
        Self: Sized;
}

/// Verify that a sequence of moves solves a puzzle.
///
/// Returns `Some(p)`, if `p` is the goal puzzle state reached from `p0` by the moves `ms`.
///
/// Returns `None`, if either the sequence of moves `ms` starting from `p0` is
/// not legal or if the puzzle state reached from `p0` by the moves `ms` is not
/// a goal state.
pub fn check<P>(p0: P, ms: &[P::Move]) -> Option<P>
where
    P: Puzzle,
    P::Move: Eq,
{
    let mut p = p0;
    'moves: for cm in ms {
        for (m, pp) in p.next() {
            if cm == &m {
                p = pp;
                continue 'moves;
            }
        }
        return None;
    }
    if p.is_goal() {
        Some(p)
    } else {
        None
    }
}

/// Solve a puzzle using BFS with hashing of states.
///
/// Returns `Some((ms,p))` if puzzle `p0` can be solved by the sequence of moves
/// `ms` to a goal state `p`.  The sequence of moves `ms` should be one of the
/// shortest sequence of moves from `p0` to a goal state; that is, for any
/// sequence of moves `ns` from `p0` to a goal state, `ms.len() <= ns.len()`.
/// Note that there may not be a unique goal state for a puzzle, therefore, the
/// goal state `p` reached by the sequence of moves `ms` is returned.
///
/// Returns `None` if `p0` cannot be solved by any sequence of moves.
///
/// A BFS is used to find the shortest sequence of moves from `p0` to a goal
/// state.  A hash set or hash table is used to avoid redundant puzzle states
/// (e.g., different sequences of moves may lead to the same puzzle state).
///
/// The generic type parameter `P` must implement `Puzzle` (because it
/// represents a puzzle state), `Eq` and `Hash` (in order to for puzzle states
/// to serve as hash-set or hash-table keys), and `Clone` (in order for puzzle
/// states to be placed in both the hash set or hash table and the BFS queue).
///
/// For simplicity, we assume that the `P::Move` type implements `Clone`.
/// However, there is an implementation of `solve` that does not require this
/// trait bound (at the expense of performing additional copies of puzzle
/// states).
pub fn solve<P>(p0: P) -> Option<(Vec<P::Move>, P)>
where
    P: Puzzle + Eq + Hash + Clone,
    P::Move: Clone,
{
    // Your code here
    let mut visited: HashMap<P, Vec<P::Move>> = HashMap::new();
    let mut todos: VecDeque<P> = VecDeque::new();
    visited.insert(p0.clone(), vec![]); // Marked it as visited
    todos.push_back(p0);
    while !todos.is_empty() {
        let p = todos.pop_front().unwrap();
        if p.is_goal() {
            return Some((visited.get(&p).unwrap().clone(), p));
        }
        for (m, q) in p.next() {
            if !visited.contains_key(&q) {
                let mut new_ms = visited.get(&p).unwrap().to_vec(); // Create a new vector from cloning all previous moves
                new_ms.push(m); // Push new move 
                visited.insert(q.clone(), new_ms); // Marked it as visited and associate m
                todos.push_back(q);
            }
        }
    }

    None
}

#[allow(clippy::type_complexity)]
pub mod test;
